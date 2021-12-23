/// JSON accepted for POST `/v0/users/me/channels` (create dm channel)
#[derive(Serialize, Deserialize, Clone)]
pub struct DMChannelCreateJson {
    /// Boolean indicating whether the DM being created is a group or not.
    /// If false is passed, it will be a regular direct message between 2 people
    /// true/false
    pub group: bool,

    /// Group name
    /// Only applies to group DMs, not regular direct messages
    /// Must be between 1 and 100 Unicode codepoints.
    pub name: Option<String>,
}

/// GET `/v0/users/me/channels`
#[derive(Serialize, Deserialize, Clone)]
pub struct GetSelfDmChannelParams {
    pub limit: Option<i64>,
}

/// POST `/v0/users/me/channels`
#[derive(Serialize, Deserialize, Clone)]
pub struct CreateDmChannelParams {
    pub users: String,
}
