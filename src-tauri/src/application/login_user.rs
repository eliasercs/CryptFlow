use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::ports::{Clock, PasswordHasher, SessionStore, UserRepository};
use crate::domain::session::Session;
use crate::domain::user::Username;

pub struct LoginUser {
    users: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
    clock: Arc<dyn Clock>,
    sessions: Arc<dyn SessionStore>,
}

impl LoginUser {
    pub fn new(
        users: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        clock: Arc<dyn Clock>,
        sessions: Arc<dyn SessionStore>,
    ) -> Self {
        Self { users, hasher, clock, sessions }
    }

    pub fn execute(&self, username: &str, password: &str) -> Result<Session, DomainError> {
        // Un nombre mal formado no puede existir: misma respuesta que "no existe".
        let username = Username::parse(username).map_err(|_| DomainError::InvalidCredentials)?;

        let Some(mut user) = self.users.find_by_username(&username)? else {
            // Gasta un tiempo parecido al de un usuario real para no revelar si existe.
            let _ = self.hasher.hash(password);
            return Err(DomainError::InvalidCredentials);
        };

        let now = self.clock.now();
        user.ensure_not_locked(now)?;

        if self.hasher.verify(password, &user.password_hash) {
            user.record_success();
            self.users.save_lock_state(&user)?;

            let session = Session::for_user(&user);
            self.sessions.set(session.clone());
            Ok(session)
        } else {
            user.record_failure(now);
            self.users.save_lock_state(&user)?;
            Err(DomainError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_support::{FakeClock, MemSession, MemUsers, PlainHasher};
    use crate::domain::user::{LOCK_SECONDS, MAX_FAILED_ATTEMPTS};
    use std::sync::atomic::{AtomicUsize, Ordering};

    const PASSWORD: &str = "clave-larga-1";

    struct Fixture {
        login: LoginUser,
        users: Arc<MemUsers>,
        sessions: Arc<MemSession>,
        clock: Arc<FakeClock>,
    }

    /// Deja listo un usuario "ana" ya registrado, sin pasar por `RegisterUser`:
    /// cada caso de uso se prueba por separado.
    fn setup() -> Fixture {
        let users = Arc::new(MemUsers::default());
        let sessions = Arc::new(MemSession::default());
        let clock = Arc::new(FakeClock::new(1_000));
        users
            .create(&Username::parse("ana").unwrap(), &format!("hash:{PASSWORD}"), 0)
            .unwrap();
        let login = LoginUser::new(
            users.clone(),
            Arc::new(PlainHasher),
            clock.clone(),
            sessions.clone(),
        );
        Fixture { login, users, sessions, clock }
    }

    #[test]
    fn logs_in_with_correct_password_ignoring_case() {
        let f = setup();
        let session = f.login.execute("ANA", PASSWORD).unwrap();
        assert_eq!(session.username.as_str(), "ana");
        assert_eq!(f.sessions.current(), Some(session));
    }

    #[test]
    fn unknown_user_and_wrong_password_give_the_same_error() {
        let f = setup();
        let unknown = f.login.execute("nadie", PASSWORD);
        let wrong = f.login.execute("ana", "incorrecta-1");
        let malformed = f.login.execute("!!", PASSWORD);
        assert_eq!(unknown, Err(DomainError::InvalidCredentials));
        assert_eq!(wrong, unknown);
        assert_eq!(malformed, unknown);
        assert_eq!(f.sessions.current(), None);
    }

    #[test]
    fn failures_are_persisted_and_success_resets_them() {
        let f = setup();
        f.login.execute("ana", "incorrecta-1").unwrap_err();
        f.login.execute("ana", "incorrecta-1").unwrap_err();
        assert_eq!(f.users.all()[0].failed_attempts, 2);

        f.login.execute("ana", PASSWORD).unwrap();
        assert_eq!(f.users.all()[0].failed_attempts, 0);
    }

    #[test]
    fn locks_after_max_failures_and_unlocks_when_time_passes() {
        let f = setup();
        for _ in 0..MAX_FAILED_ATTEMPTS {
            assert_eq!(f.login.execute("ana", "incorrecta-1"), Err(DomainError::InvalidCredentials));
        }

        // Bloqueada: ni la contraseña correcta entra, y no se abre sesión.
        assert_eq!(
            f.login.execute("ana", PASSWORD),
            Err(DomainError::AccountLocked { seconds_left: LOCK_SECONDS })
        );
        assert_eq!(f.sessions.current(), None);

        f.clock.advance(LOCK_SECONDS); // "pasa" un minuto, sin esperar
        assert!(f.login.execute("ana", PASSWORD).is_ok());
        assert!(f.sessions.current().is_some());
    }

    /// Espía: cuenta cuántas veces se le pide calcular un hash.
    struct CountingHasher(AtomicUsize);
    impl PasswordHasher for CountingHasher {
        fn hash(&self, _plain: &str) -> Result<String, DomainError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(String::new())
        }
        fn verify(&self, _plain: &str, _hash: &str) -> bool {
            false
        }
    }

    #[test]
    fn unknown_user_still_spends_hashing_time() {
        let hasher = Arc::new(CountingHasher(AtomicUsize::new(0)));
        let login = LoginUser::new(
            Arc::new(MemUsers::default()),
            hasher.clone(),
            Arc::new(FakeClock::new(1_000)),
            Arc::new(MemSession::default()),
        );
        login.execute("nadie", PASSWORD).unwrap_err();
        assert_eq!(hasher.0.load(Ordering::SeqCst), 1);
    }
}