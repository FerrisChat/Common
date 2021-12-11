use std::ops::{BitOr, BitOrAssign};
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

impl BitOr for Permissions {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self {
            send_messages: self.send_messages | other.send_messages,
            delete_messages: self.delete_messages | other.delete_messages,
            edit_channels: self.edit_channels | other.edit_channels,
            add_remove_channels: self.add_remove_channels | other.add_remove_channels,
            kick_members: self.kick_members | other.kick_members,
            ban_members: self.ban_members | other.ban_members,
            mute_members: self.mute_members | other.mute_members,
            add_bots: self.add_bots | other.add_bots,
            adminstrator: self.adminstrator | other.adminstrator,
        }
    }
}

impl BitOrAssign for Permissions {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}
