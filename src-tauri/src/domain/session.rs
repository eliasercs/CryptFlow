use super::user::{User, UserId, Username};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub user_id: UserId,
    pub username: Username,
}

impl Session {
    pub fn for_user(user: &User) -> Self {
        Self { user_id: user.id, username: user.username.clone() }
    }
}


