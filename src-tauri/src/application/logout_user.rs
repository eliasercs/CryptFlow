use std::sync::Arc;

use crate::domain::ports::SessionStore;

pub struct LogoutUser {
    sessions: Arc<dyn SessionStore>,
}

impl LogoutUser {
    pub fn new(sessions: Arc<dyn SessionStore>) -> Self {
        Self { sessions }
    }

    pub fn execute(&self) {
        self.sessions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_support::MemSession;
    use crate::domain::session::Session;
    use crate::domain::user::{UserId, Username};

    fn open_session() -> Session {
        Session { user_id: UserId(1), username: Username::parse("ana").unwrap() }
    }

    #[test]
    fn clears_the_session() {
        let sessions = Arc::new(MemSession::default());
        sessions.set(open_session());

        LogoutUser::new(sessions.clone()).execute();

        assert_eq!(sessions.current(), None);
    }

    #[test]
    fn logging_out_without_session_is_harmless() {
        let sessions = Arc::new(MemSession::default());
        let logout = LogoutUser::new(sessions.clone());
        logout.execute();
        logout.execute();
        assert_eq!(sessions.current(), None);
    }
}