use super::error::DomainError;
use super::session::Session;
use super::user::{User, Username};

pub trait UserRepository: Send + Sync {
    fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError>;

    /// Debe devolver `UsernameTaken` si el nombre ya existe (sin distinguir mayúsculas).
    fn create(&self, username: &Username, password_hash: &str, now: i64) -> Result<User, DomainError>;

    fn save_lock_state(&self, user: &User) -> Result<(), DomainError>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, plain: &str) -> Result<String, DomainError>;
    fn verify(&self, plain: &str, hash: &str) -> bool;
}

pub trait Clock: Send + Sync {
    fn now(&self) -> i64;
}

pub trait SessionStore: Send + Sync {
    fn set(&self, session: Session);
    fn current(&self) -> Option<Session>;
    fn clear(&self);
}