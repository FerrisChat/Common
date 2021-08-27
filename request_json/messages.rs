/// JSON accepted for POST /api/v0/guilds/{guild_id}/channels/{channel_id}/messages (create message)
#[derive(Serialize, Deserialize)]
pub struct MessageCreateJson {
    /// Message content.
    pub content: String,
}

/// PATCH /api/v0/channels/{channel_id}/messages/{message_id}
#[derive(Serialize, Deserialize)]
pub struct MessageEditJson {
    /// Message content.
    pub content: Option<String>,
}

/// GET /api/v0/channels/{channel_id}/messages
#[derive(Serialize, Deserialize)]
pub struct GetMessageHistoryParams {
    pub limit: Option<u64>,
}
