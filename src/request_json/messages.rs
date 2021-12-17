use crate::types::Embed;

/// JSON accepted for POST `/v0/channels/{channel_id}/messages` (create message)
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageCreateJson {
    /// Message content.
    pub content: String,
    /// Message embed.
    pub embed: Option<Embed>,
    /// Message Nonce
    pub nonce: Option<String>,
}

/// PATCH `/v0/channels/{channel_id}/messages/{message_id}`
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageUpdateJson {
    /// Message content.
    pub content: Option<String>,
    /// Message Embed.
    pub embed: Option<Embed>,
}

/// GET `/v0/channels/{channel_id}/messages`
#[derive(Serialize, Deserialize, Clone)]
pub struct GetMessageHistoryParams {
    pub limit: Option<i64>,
    pub oldest_first: Option<bool>,
    pub offset: Option<i64>,
}
