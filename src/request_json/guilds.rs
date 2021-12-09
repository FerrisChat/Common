/// JSON accepted for POST `/v0/guilds/` (create guild)
#[derive(Serialize, Deserialize, Clone)]
pub struct GuildCreateJson {
    /// Guild name.
    ///
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: String,
}

/// URL parameters accepted for GET `/v0/guilds/{guild_id}` (get guild)
#[derive(Serialize, Deserialize, Clone)]
pub struct GetGuildUrlParams {
    /// Return the list of channels in the response?
    ///
    /// Defaults to `true` if not specified.
    pub channels: Option<bool>,

    /// Return the list of members in the response?
    ///
    /// Defaults to `false` if not specified.
    pub members: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GuildUpdateJson {
    /// Guild name.
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: Option<String>,
}
