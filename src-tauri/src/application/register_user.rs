use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::ports::{Clock, PasswordHasher, SessionStore, UserRepository};
use crate::domain::session::Session;
use crate::domain::user::{ensure_strong_password, Username};

pub struct RegisterUser {
    users: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
    clock: Arc<dyn Clock>,
    sessions: Arc<dyn SessionStore>,
}

impl RegisterUser {
    pub fn new(
        users: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        clock: Arc<dyn Clock>,
        sessions: Arc<dyn SessionStore>,
    ) -> Self {
        Self { users, hasher, clock, sessions }
    }

    pub fn execute(&self, username: &str, password: &str) -> Result<Session, DomainError> {
        let username = Username::parse(username)?;
        ensure_strong_password(password)?;

        if self.users.find_by_username(&username)?.is_some() {
            return Err(DomainError::UsernameTaken);
        }

        let hash = self.hasher.hash(password)?;
        let user = self.users.create(&username, &hash, self.clock.now())?;

        let session = Session::for_user(&user);
        self.sessions.set(session.clone());
        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_support::{FakeClock, MemSession, MemUsers, PlainHasher};

    struct Fixture {
        register: RegisterUser,
        users: Arc<MemUsers>,
        sessions: Arc<MemSession>,
    }

    fn setup() -> Fixture {
        let users = Arc::new(MemUsers::default());
        let sessions = Arc::new(MemSession::default());
        let register = RegisterUser::new(
            users.clone(),
            Arc::new(PlainHasher),
            Arc::new(FakeClock::new(1_000)),
            sessions.clone(),
        );
        Fixture { register, users, sessions }
    }

    #[test]
    fn registers_user_stores_hash_and_starts_session() {
        let f = setup();
        let session = f.register.execute("Ana", "clave-larga-1").unwrap();

        assert_eq!(session.username.as_str(), "Ana");
        assert_eq!(f.sessions.current(), Some(session));

        let stored = f.users.all();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].password_hash, "hash:clave-larga-1");
        assert_ne!(stored[0].password_hash, "clave-larga-1");
    }

    #[test]
    fn rejects_duplicate_username_ignoring_case() {
        let f = setup();
        f.register.execute("Ana", "clave-larga-1").unwrap();
        assert_eq!(
            f.register.execute("ana", "otra-clave-12"),
            Err(DomainError::UsernameTaken)
        );
        assert_eq!(f.users.all().len(), 1);
    }

    #[test]
    fn rejects_invalid_input_without_side_effects() {
        let f = setup();
        assert_eq!(f.register.execute("a", "clave-larga-1"), Err(DomainError::InvalidUsername));
        assert_eq!(f.register.execute("ana", "corta"), Err(DomainError::WeakPassword));
        assert!(f.users.all().is_empty());
        assert_eq!(f.sessions.current(), None);
    }
}