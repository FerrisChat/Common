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

/// JSON accepted for DELETE /api/v0/users/{user_id}/bots/{bot_id}
#[derive(Serialize, Deserialize)]
pub struct BotDeleteJson {
    /// Always set this to true.
    pub are_you_sure: bool,

    /// Also always set this to true
    pub are_you_really_sure: bool,

    /// Just for fun, if you want, set this to true.
    pub oletko_varma: Option<bool>,
}
