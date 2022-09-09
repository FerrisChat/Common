use crate::crate_prelude::*;

bitflags::bitflags! {
    /// A bitmask of permission flags, representing what members of a guild
    /// are allowed to do.
    #[derive(Default)]
    pub struct Permissions: u32 {
        /// People with this permission can view channels and receive message
        /// events from them.
        const VIEW_CHANNEL = 1 << 0;
        /// People with this permission can send messages in channels.
        const SEND_MESSAGES = 1 << 1;
    }
}

serde_for_bitflags!(Permissions);
