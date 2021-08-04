use crate::types::{User, Guild};

#[derive(Serialize, Deserialize)]
pub struct Member {
    pub user_id: Option<i64>,
    pub user: Option<User>,

    pub guild_id: Option<i64>,
    pub guild: Option<Guild>,
}