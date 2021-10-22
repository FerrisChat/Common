/// JSON accepted for POST /api/v0/guilds/{guild_id}/channels/ (create channel)
#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelCreateJson {
    /// Channel name.
    ///
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: String,
}

// PATCH /api/v0/channels/{channel_id}
#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelUpdateJson {
    /// Channel name.
    ///
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: Option<String>,
}
