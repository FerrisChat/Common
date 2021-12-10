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

impl Permissions {
    #[inline]
    pub fn all_false() -> Self {
        Self {
            send_messages: Some(false),
            delete_messages: Some(false),
            edit_channels: Some(false),
            add_remove_channels: Some(false),
            kick_members: Some(false),
            ban_members: Some(false),
            mute_members: Some(false),
            add_bots: Some(false),
            adminstrator: Some(false),
        }
    }

    #[inline]
    pub fn all_true() -> Self {
        Self {
            send_messages: Some(true),
            delete_messages: Some(true),
            edit_channels: Some(true),
            add_remove_channels: Some(true),
            kick_members: Some(true),
            ban_members: Some(true),
            mute_members: Some(true),
            add_bots: Some(true),
            adminstrator: Some(true),
        }
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            send_messages: None,
            delete_messages: None,
            edit_channels: None,
            add_remove_channels: None,
            kick_members: None,
            ban_members: None,
            mute_members: None,
            add_bots: None,
            adminstrator: None,
        }
    }
}
