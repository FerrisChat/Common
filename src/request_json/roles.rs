use crate::perms::GuildPermissions;

/// JSON accepted for POST `/v0/guilds/{guild_id}/roles`
#[derive(Serialize, Deserialize, Clone)]
pub struct RoleCreateJson {
    /// Role name
    ///
    /// Must be between 1 and 256 Unicode codepoints.
    pub name: Option<String>,

    /// Role color
    ///
    /// Must be an integer between 0 and 16777215 (0xFFFFFF)
    pub color: Option<i32>,

    /// Role position
    ///
    /// Must be an integer between 0 and 1023
    pub position: Option<i16>,

    /// Role guild permissions
    pub guild_permissions: Option<GuildPermissions>,
}

/// JSON accepted for PATCH `/v0/guilds/{guild_id}/roles/{role_id}`
#[derive(Serialize, Deserialize, Clone)]
pub struct RoleUpdateJson {
    /// Role name
    ///
    /// Must be between 1 and 256 Unicode codepoints.
    pub name: Option<String>,

    /// Role color
    ///
    /// Must be an integer between 0 and 16777216
    ///
    /// Set to 16777216 to remove color
    ///
    /// Anything equal to or below 16777215 will be interpreted as a color
    pub color: Option<i32>,

    /// Role position
    ///
    /// Must be an integer between 0 and 1023
    pub position: Option<i16>,

    /// Role permissions
    pub permissions: Option<GuildPermissions>,
}
