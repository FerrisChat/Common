/// JSON accepted for POST /api/v0/guilds/ (create guild)
#[derive(Serialize, Deserialize)]
pub struct GuildCreateJson {
    /// Guild name.
    ///
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: String,
}
