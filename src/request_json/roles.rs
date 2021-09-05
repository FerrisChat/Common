/// JSON accepted for POST /api/v0/guilds/{guild_id}/roles
#[derive(Serialize, Deserialize)]
pub struct RoleCreateJson {
    /// Role name
    ///
    /// Must be between 1 and 100 Unicode codepoints.
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
    ///
    /// Must be an integer between 0 and 9,223,372,036,854,775,807
    pub permissions: Option<i64>,
}

/// JSON accepted for PATCH /api/v0/guilds/{guild_id}/roles/{role_id}
#[derive(Serialize, Deserialize)]
pub struct RoleUpdateJson {
    /// Role name
    ///
    /// Must be between 1 and 100 Unicode codepoints.
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
    ///
    /// Must be an integer between 0 and 9,223,372,036,854,775,807
    pub permissions: Option<i64>,
}
