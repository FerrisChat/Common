/// JSON accepted for POST /api/v0/guilds/{guild_id}/invites/ (create invite)
#[derive(Serialize, Deserialize, Clone)]
pub struct InviteCreateJson {
    /// Max age of the invite (Invite will become invalid after this amount of seconds)
    pub max_age: Option<i64>,

    /// Max uses of the invite (Invite will become invalid after this amount of uses)
    pub max_uses: Option<i16>,
}
