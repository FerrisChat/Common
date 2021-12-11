use std::ops::BitOr;
use tribool::Tribool;

#[derive(Serialize, Deserialize, Default, Debug, PartialOrd, PartialEq, Copy, Clone, Hash)]
pub struct Permissions {
    pub send_messages: Tribool,
    pub delete_messages: Tribool,
    pub edit_channels: Tribool,
    pub add_remove_channels: Tribool,
    pub kick_members: Tribool,
    pub ban_members: Tribool,
    pub mute_members: Tribool,
    pub add_bots: Tribool,
    pub adminstrator: Tribool,
}

impl Permissions {
    #[inline]
    #[must_use]
    pub const fn all_false() -> Self {
        Self {
            send_messages: Tribool::False,
            delete_messages: Tribool::False,
            edit_channels: Tribool::False,
            add_remove_channels: Tribool::False,
            kick_members: Tribool::False,
            ban_members: Tribool::False,
            mute_members: Tribool::False,
            add_bots: Tribool::False,
            adminstrator: Tribool::False,
        }
    }

    #[inline]
    #[must_use]
    pub const fn all_true() -> Self {
        Self {
            send_messages: Tribool::True,
            delete_messages: Tribool::True,
            edit_channels: Tribool::True,
            add_remove_channels: Tribool::True,
            kick_members: Tribool::True,
            ban_members: Tribool::True,
            mute_members: Tribool::True,
            add_bots: Tribool::True,
            adminstrator: Tribool::True,
        }
    }

    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            send_messages: Tribool::Indeterminate,
            delete_messages: Tribool::Indeterminate,
            edit_channels: Tribool::Indeterminate,
            add_remove_channels: Tribool::Indeterminate,
            kick_members: Tribool::Indeterminate,
            ban_members: Tribool::Indeterminate,
            mute_members: Tribool::Indeterminate,
            add_bots: Tribool::Indeterminate,
            adminstrator: Tribool::Indeterminate,
        }
    }
}
