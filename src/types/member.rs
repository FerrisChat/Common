use crate::types::{User, Guild};

pub struct Member {
    pub user_id: Option<i64>,
    pub user: Option<User>,

    pub guild_id: Option<i64>,
    pub guild: Option<Guild>,
}