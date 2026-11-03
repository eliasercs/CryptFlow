use crate::domain::error::DomainError;
use crate::domain::user::MIN_PASSWORD_CHARS;

/// Traduce un error de negocio al texto que ve la persona usuaria.
/// El `match` es exhaustivo: si agregas una variante a `DomainError`, el compilador
/// te obliga a decidir aquí qué mensaje le corresponde.
pub fn error_message(error: &DomainError) -> String {
    match error {
        DomainError::InvalidUsername => {
            "El usuario debe tener entre 3 y 32 caracteres (letras, números, _ . -)".to_string()
        }
        DomainError::WeakPassword => {
            format!("La contraseña debe tener al menos {MIN_PASSWORD_CHARS} caracteres")
        }
        DomainError::UsernameTaken => "Ese nombre de usuario ya está en uso".to_string(),
        DomainError::InvalidCredentials => "Usuario o contraseña incorrectos".to_string(),
        DomainError::AccountLocked { seconds_left } => {
            format!("Demasiados intentos. Vuelve a intentar en {seconds_left} s")
        }
        DomainError::Internal(detail) => format!("Error interno: {detail}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_error_is_generic() {
        assert_eq!(
            error_message(&DomainError::InvalidCredentials),
            "Usuario o contraseña incorrectos"
        );
    }

    #[test]
    fn messages_carry_the_useful_details() {
        assert!(error_message(&DomainError::WeakPassword).contains(&MIN_PASSWORD_CHARS.to_string()));
        assert!(error_message(&DomainError::AccountLocked { seconds_left: 42 }).contains("42 s"));
        assert!(error_message(&DomainError::Internal("disco lleno".into())).contains("disco lleno"));
    }

    #[test]
    fn no_error_produces_an_empty_message() {
        let all = [
            DomainError::InvalidUsername,
            DomainError::WeakPassword,
            DomainError::UsernameTaken,
            DomainError::InvalidCredentials,
            DomainError::AccountLocked { seconds_left: 1 },
            DomainError::Internal("x".into()),
        ];
        for e in &all {
            assert!(!error_message(e).trim().is_empty(), "{e:?}");
        }
    }
}