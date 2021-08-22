/// JSON accepted for POST /api/v0/guilds/ (create guild)
#[derive(Serialize, Deserialize)]
pub struct GuildCreateJson {
    /// Guild name.
    ///
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: String,
}

/// URL parameters accepted for GET /api/v0/guilds/{guild_id} (get guild)
pub struct GetGuildUrlParams {
    /// Return the list of channels in the response?
    ///
    /// Defaults to `true` if not specified.
    channels: Option<bool>,

    /// Return the list of members in the response?
    ///
    /// Defaults to `false` if not specified.
    members: Option<bool>
}