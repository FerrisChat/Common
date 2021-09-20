use crate::types::{Guild, User};

#[derive(Serialize, Deserialize, Clone)]
pub struct Member {
    pub user_id: Option<u128>,
    pub user: Option<User>,

    pub guild_id: Option<u128>,
    pub guild: Option<Guild>,
}
