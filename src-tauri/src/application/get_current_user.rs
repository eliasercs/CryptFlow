use std::sync::Arc;

use crate::domain::ports::SessionStore;
use crate::domain::session::Session;

pub struct GetCurrentUser {
    sessions: Arc<dyn SessionStore>,
}

impl GetCurrentUser {
    pub fn new(sessions: Arc<dyn SessionStore>) -> Self {
        Self { sessions }
    }

    pub fn execute(&self) -> Option<Session> {
        self.sessions.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::test_support::MemSession;
    use crate::domain::user::{UserId, Username};

    #[test]
    fn returns_none_without_session_and_the_session_when_active() {
        let sessions = Arc::new(MemSession::default());
        let current = GetCurrentUser::new(sessions.clone());
        assert_eq!(current.execute(), None);

        let session = Session { user_id: UserId(1), username: Username::parse("ana").unwrap() };
        sessions.set(session.clone());
        assert_eq!(current.execute(), Some(session));
    }
}