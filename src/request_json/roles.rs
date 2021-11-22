use crate::perms::Permissions;

/// JSON accepted for POST /v0/guilds/{guild_id}/roles
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

    /// Role permissions
    pub permissions: Option<Permissions>,
}

/// JSON accepted for PATCH /v0/guilds/{guild_id}/roles/{role_id}
#[derive(Serialize, Deserialize, Clone)]
pub struct RoleUpdateJson {
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

    /// Role permissions
    pub permissions: Option<Permissions>,
}
