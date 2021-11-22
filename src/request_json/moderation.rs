/// JSON accepted for POST /v0/guilds/{guild_id}/members/{member_id}/ban (ban user)
#[derive(Serialize, Deserialize, Clone)]
pub struct BanJson {
    /// Amount of seconds to ban the user for
    /// a value of 0 or not including it will make the ban permanent
    pub duration: Option<i64>,

    /// Reason for ban, will be shown to the user and in audit log
    pub reason: Option<String>,
}
/// JSON accepted for POST /v0/guilds/{guild_id}/members/{member_id}/kick (kick user)
#[derive(Serialize, Deserialize, Clone)]
pub struct KickJson {
    /// Reason for kick, will be shown to the user and in audit log
    pub reason: Option<String>,
}
/// JSON accepted for POST /v0/guilds/{guild_id}/members/{member_id}/mute (mute user)
#[derive(Serialize, Deserialize, Clone)]
pub struct MuteJson {
    /// Amount of seconds to mute the user for
    /// a value of 0 or not including it will make the mute permanent
    pub duration: Option<i64>,

    /// Reason for mute, will be shown to the user and in audit log
    pub reason: Option<String>,
}
