/// JSON accepted for POST /api/v0/guilds/{guild_id}/channels/{channel_id}/messages (create message)
#[derive(Serialize, Deserialize)]
pub struct MessageCreateJson {
    /// Message content.
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetMessageHistoryParams {
    pub limit: Option<i64>,
}
