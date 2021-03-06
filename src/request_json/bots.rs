/// JSON accepted for POST `/v0/users/{user_id}/bots`
#[derive(Serialize, Deserialize, Clone)]
pub struct BotCreateJson {
    /// Bot's Name
    pub username: String,
}

/// JSON accepted for PATCH `/v0/users/{user_id}/bots/{bot_id}`
#[derive(Serialize, Deserialize, Clone)]
pub struct BotUpdateJson {
    /// Bot's username
    pub username: Option<String>,

    /// Bot's avatar,
    pub avatar: Option<String>,
}

/// POST `/v0/bots/{bot_id}/add`
#[derive(Serialize, Deserialize, Clone)]
pub struct BotInviteJson {
    /// ID of guild bot is to be added to
    pub guild_id: u128,
}
