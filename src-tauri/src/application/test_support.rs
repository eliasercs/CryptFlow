//! Dobles de prueba: implementaciones falsas de los puertos, todo en memoria.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Mutex;

use crate::domain::error::DomainError;
use crate::domain::ports::{Clock, PasswordHasher, SessionStore, UserRepository};
use crate::domain::session::Session;
use crate::domain::user::{User, UserId, Username, LOCK_SECONDS, MAX_FAILED_ATTEMPTS};

#[derive(Default)]
pub struct MemUsers {
    users: Mutex<Vec<User>>,
}

impl MemUsers {
    pub fn all(&self) -> Vec<User> {
        self.users.lock().unwrap().clone()
    }
}

impl UserRepository for MemUsers {
    fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        let all = self.users.lock().unwrap();
        Ok(all
            .iter()
            .find(|u| u.username.as_str().eq_ignore_ascii_case(username.as_str()))
            .cloned())
    }

    fn create(&self, username: &Username, password_hash: &str, _now: i64) -> Result<User, DomainError> {
        let mut all = self.users.lock().unwrap();
        if all.iter().any(|u| u.username.as_str().eq_ignore_ascii_case(username.as_str())) {
            return Err(DomainError::UsernameTaken);
        }
        let user = User {
            id: UserId(all.len() as i64 + 1),
            username: username.clone(),
            password_hash: password_hash.to_string(),
            failed_attempts: 0,
            locked_until: 0,
        };
        all.push(user.clone());
        Ok(user)
    }

    fn save_lock_state(&self, user: &User) -> Result<(), DomainError> {
        let mut all = self.users.lock().unwrap();
        if let Some(stored) = all.iter_mut().find(|u| u.id == user.id) {
            stored.failed_attempts = user.failed_attempts;
            stored.locked_until = user.locked_until;
        }
        Ok(())
    }
}

/// Sin Argon2: instantáneo y predecible.
pub struct PlainHasher;

impl PasswordHasher for PlainHasher {
    fn hash(&self, plain: &str) -> Result<String, DomainError> {
        Ok(format!("hash:{plain}"))
    }
    fn verify(&self, plain: &str, hash: &str) -> bool {
        hash == format!("hash:{plain}")
    }
}

/// Reloj que controla la prueba.
pub struct FakeClock(AtomicI64);

impl FakeClock {
    pub fn new(start: i64) -> Self {
        Self(AtomicI64::new(start))
    }

    /// Hacer avanzar el tiempo sin esperar
    pub fn advance(&self, seconds: i64) {
        self.0.fetch_add(seconds, Ordering::SeqCst);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> i64 {
        self.0.load(Ordering::SeqCst)
    }
}

#[derive(Default)]
pub struct MemSession(Mutex<Option<Session>>);

impl SessionStore for MemSession {
    fn set(&self, session: Session) {
        *self.0.lock().unwrap() = Some(session);
    }
    fn current(&self) -> Option<Session> {
        self.0.lock().unwrap().clone()
    }
    fn clear(&self) {
        *self.0.lock().unwrap() = None;
    }
}

/// Contrato que debe cumplir CUALQUIER implementación de `UserRepository`.
/// Se ejecuta contra la falsa y contra la real: si ambas lo pasan, son intercambiables.
pub fn assert_user_repository_contract(repo: &dyn UserRepository) {
    let ana = Username::parse("Ana").unwrap();
    assert!(repo.find_by_username(&ana).unwrap().is_none());

    // crear devuelve el usuario con su id asignado y contadores en cero
    let created = repo.create(&ana, "hash-ana", 100).unwrap();
    assert_eq!(created.username.as_str(), "Ana");
    assert_eq!(created.password_hash, "hash-ana");
    assert_eq!((created.failed_attempts, created.locked_until), (0, 0));

    // la búsqueda no distingue mayúsculas
    let found = repo
        .find_by_username(&Username::parse("ANA").unwrap())
        .unwrap()
        .unwrap();
    assert_eq!(found.id, created.id);

    // un nombre repetido (aunque cambien las mayúsculas) se rechaza
    assert_eq!(
        repo.create(&Username::parse("ana").unwrap(), "otro", 101).err(),
        Some(DomainError::UsernameTaken)
    );

    // otro usuario recibe otro id
    let luis = repo.create(&Username::parse("luis").unwrap(), "hash-luis", 102).unwrap();
    assert_ne!(luis.id, created.id);

    // el estado de bloqueo se guarda... y solo para ese usuario
    let mut changed = found;
    changed.record_failure(1_000);
    changed.record_failure(1_000);
    repo.save_lock_state(&changed).unwrap();
    let again = repo.find_by_username(&ana).unwrap().unwrap();
    assert_eq!(again.failed_attempts, 2);
    let other = repo
        .find_by_username(&Username::parse("luis").unwrap())
        .unwrap()
        .unwrap();
    assert_eq!(other.failed_attempts, 0);

    // la fecha de desbloqueo también se guarda
    let mut locked = again;
    for _ in 0..MAX_FAILED_ATTEMPTS {
        locked.record_failure(2_000);
    }
    repo.save_lock_state(&locked).unwrap();
    let stored = repo.find_by_username(&ana).unwrap().unwrap();
    assert_eq!(stored.locked_until, 2_000 + LOCK_SECONDS);
}

#[test]
fn in_memory_fake_meets_the_repository_contract() {
    assert_user_repository_contract(&MemUsers::default());
}