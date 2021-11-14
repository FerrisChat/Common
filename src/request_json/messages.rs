/// JSON accepted for POST /api/v0/guilds/{guild_id}/channels/{channel_id}/messages (create message)
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageCreateJson {
    /// Message content.
    pub content: String,
    pub nonce: Option<String>,
}

/// PATCH /api/v0/channels/{channel_id}/messages/{message_id}
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageUpdateJson {
    /// Message content.
    pub content: Option<String>,
}

/// GET /api/v0/channels/{channel_id}/messages
#[derive(Serialize, Deserialize, Clone)]
pub struct GetMessageHistoryParams {
    pub limit: Option<i64>,
    pub oldest_first: Option<bool>,
    pub offset: Option<i64>,
}
