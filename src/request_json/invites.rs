/// JSON accepted for POST /api/v0/guilds/{guild_id}/invites/ (create invite)
#[derive(Serialize, Deserialize)]
pub struct InviteCreateJson {
  /// Max age of the invite (Invite will become invalid after this amount of seconds)
  pub max_age: u32,

  /// Max uses of the invite (Invite will become invalid after this amount of uses)
  pub max_uses: u16,
}
