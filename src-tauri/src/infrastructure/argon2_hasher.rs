use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString,
};
use argon2::Argon2;

use crate::domain::error::DomainError;
use crate::domain::ports::PasswordHasher;

/// Adaptador del puerto `PasswordHasher` usando Argon2id.
pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, plain: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(plain.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| DomainError::Internal(format!("hash de contraseña: {e}")))
    }

    fn verify(&self, plain: &str, hash: &str) -> bool {
        PasswordHash::new(hash)
            .map(|parsed| Argon2::default().verify_password(plain.as_bytes(), &parsed).is_ok())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify_roundtrip() {
        let hasher = Argon2PasswordHasher;
        let hash = hasher.hash("clave-larga-1").unwrap();
        assert!(hasher.verify("clave-larga-1", &hash));
        assert!(!hasher.verify("otra-clave-12", &hash));
    }

    #[test]
    fn same_password_gives_different_hashes_thanks_to_the_salt() {
        let hasher = Argon2PasswordHasher;
        let a = hasher.hash("clave-larga-1").unwrap();
        let b = hasher.hash("clave-larga-1").unwrap();
        assert_ne!(a, b);
        assert!(hasher.verify("clave-larga-1", &a));
        assert!(hasher.verify("clave-larga-1", &b));
    }

    #[test]
    fn stored_value_is_a_phc_argon2id_string_without_the_password() {
        let hash = Argon2PasswordHasher.hash("clave-larga-1").unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(!hash.contains("clave-larga-1"));
    }

    #[test]
    fn garbage_hash_never_verifies_and_never_panics() {
        let hasher = Argon2PasswordHasher;
        assert!(!hasher.verify("clave-larga-1", ""));
        assert!(!hasher.verify("clave-larga-1", "no-es-un-hash"));
    }
}