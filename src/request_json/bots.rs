/// JSON accepted for POST /api/v0/users/{user_id}/bots
#[derive(Serialize, Deserialize, Clone)]
pub struct BotCreateJson {
    /// Bot's Name
    pub username: String,
}

/// JSON accepted for PATCH /api/v0/users/{user_id}/bots/{bot_id}
#[derive(Serialize, Deserialize, Clone)]
pub struct BotUpdateJson {
    /// Bot's username
    pub username: Option<String>,
}
