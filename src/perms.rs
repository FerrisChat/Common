#[derive(
    Serialize, Deserialize, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
pub struct Permissions {
    pub send_messages: Option<bool>,
    pub delete_messages: Option<bool>,
    pub edit_channels: Option<bool>,
    pub add_remove_channels: Option<bool>,
    pub kick_members: Option<bool>,
    pub ban_members: Option<bool>,
    pub mute_members: Option<bool>,
    pub add_bots: Option<bool>,
    pub adminstrator: Option<bool>,
}
