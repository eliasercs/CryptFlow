#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    InvalidUsername,
    WeakPassword,
    AccountLocked { seconds_left: i64},
    UsernameTaken,
    InvalidCredentials,
    /// Fallo técnico de un adaptador
    Internal(String),
}