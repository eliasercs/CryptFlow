use super::error::DomainError;

pub const MAX_FAILED_ATTEMPTS: u32 = 5;
pub const LOCK_SECONDS: i64 = 60;
pub const MIN_PASSWORD_CHARS: usize = 8;

/// Si existe un username ya es válido, no hace falta volver a comprobarlo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        let name = raw.trim();
        let len_ok = (3..=32).contains(&name.chars().count());

        let chars_ok = name
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '-'));

        if len_ok && chars_ok {
            Ok(Self(name.to_string()))
        } else {
            Err(DomainError::InvalidUsername)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn ensure_strong_password(plain: &str) -> Result<(), DomainError> {
    if plain.chars().count() >= MIN_PASSWORD_CHARS {
        Ok(())
    } else {
        Err(DomainError::WeakPassword)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(pub i64);

/// Entidad: tiene identidad (`id`) y cambia con el tiempo.
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password_hash: String,
    pub failed_attempts: u32,
    pub locked_until: i64,
}

impl User {
    pub fn ensure_not_locked(&self, now: i64) -> Result<(), DomainError> {
        if self.locked_until > now {
            Err(DomainError::AccountLocked { seconds_left: self.locked_until - now })
        } else {
            Ok(())
        }
    }

    pub fn record_failure(&mut self, now: i64) {
        self.failed_attempts += 1;
        if self.failed_attempts >= MAX_FAILED_ATTEMPTS {
            self.failed_attempts = 0;
            self.locked_until = now + LOCK_SECONDS;
        }
    }

    pub fn record_success(&mut self) {
        self.failed_attempts = 0;
        self.locked_until = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user() -> User {
        User {
            id: UserId(1),
            username: Username::parse("ana").unwrap(),
            password_hash: "hash".into(),
            failed_attempts: 0,
            locked_until: 0,
        }
    }

    #[test]
    fn accepts_valid_names_and_trims_spaces() {
        assert_eq!(Username::parse("  ana_01  ").unwrap().as_str(), "ana_01");
    }

    #[test]
    fn rejects_invalid_names() {
        for bad in ["", "ab", "con espacio", "a/b", &"x".repeat(33)] {
            assert_eq!(Username::parse(bad), Err(DomainError::InvalidUsername), "{bad:?}");
        }
    }

    #[test]
    fn password_rules() {
        assert_eq!(ensure_strong_password("corta"), Err(DomainError::WeakPassword));
        assert!(ensure_strong_password("suficiente").is_ok());
    }

    #[test]
    fn locks_after_max_failures_and_unlocks_with_time() {
        let mut u = user();
        for _ in 0..MAX_FAILED_ATTEMPTS - 1 {
            u.record_failure(1000);
            assert!(u.ensure_not_locked(1000).is_ok());
        }
        u.record_failure(1000);
        assert_eq!(
            u.ensure_not_locked(1010),
            Err(DomainError::AccountLocked { seconds_left: LOCK_SECONDS - 10 })
        );
        assert!(u.ensure_not_locked(1000 + LOCK_SECONDS).is_ok());
    }

    #[test]
    fn success_resets_counters() {
        let mut u = user();
        u.record_failure(10);
        u.record_failure(10);
        u.record_success();
        assert_eq!((u.failed_attempts, u.locked_until), (0, 0));
    }
}