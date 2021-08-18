/// JSON accepted for POST /api/v0/guilds/{guild_id}/channels/ (create channel)
#[derive(Serialize, Deserialize)]
pub struct ChannelCreateJson {
    /// Channel name.
    ///
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: String,
}
