use std::sync::Arc;

use super::get_current_user::GetCurrentUser;
use super::login_user::LoginUser;
use super::logout_user::LogoutUser;
use super::register_user::RegisterUser;
use crate::domain::ports::{Clock, PasswordHasher, SessionStore, UserRepository};

/// Agrupa los casos de uso de autenticación para entregarlos juntos a la capa de presentación.
pub struct AuthUseCases {
    pub register: RegisterUser,
    pub login: LoginUser,
    pub logout: LogoutUser,
    pub current_user: GetCurrentUser,
}

impl AuthUseCases {
    pub fn new(
        users: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        clock: Arc<dyn Clock>,
        sessions: Arc<dyn SessionStore>,
    ) -> Self {
        Self {
            register: RegisterUser::new(users.clone(), hasher.clone(), clock.clone(), sessions.clone()),
            login: LoginUser::new(users, hasher, clock, sessions.clone()),
            logout: LogoutUser::new(sessions.clone()),
            current_user: GetCurrentUser::new(sessions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_support::{FakeClock, MemSession, MemUsers, PlainHasher};

    #[test]
    fn full_flow_register_logout_login_shares_one_session() {
        let auth = AuthUseCases::new(
            Arc::new(MemUsers::default()),
            Arc::new(PlainHasher),
            Arc::new(FakeClock::new(1_000)),
            Arc::new(MemSession::default()),
        );

        assert_eq!(auth.current_user.execute(), None);

        let registered = auth.register.execute("ana", "clave-larga-1").unwrap();
        assert_eq!(auth.current_user.execute(), Some(registered));

        auth.logout.execute();
        assert_eq!(auth.current_user.execute(), None);

        let logged = auth.login.execute("ana", "clave-larga-1").unwrap();
        assert_eq!(auth.current_user.execute(), Some(logged));
    }
}