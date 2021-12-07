use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Permissions: i64 {
        /// Permissions to send messages.
        const SEND_MESSAGES =       1 << 0;
        /// Permissions to delete messages.
        const DELETE_MESSAGES =     1 << 1;
        /// Permissions to edit channels.
        const EDIT_CHANNELS =       1 << 2;
        /// Permissions to add/remove channels.
        const ADD_REMOVE_CHANNELS = 1 << 3;
        /// Permissions to remove members (aka kick them).
        const KICK_MEMBERS =        1 << 4;
        /// Permissions to remove members permanently (aka ban them).
        const BAN_MEMBERS =         1 << 5;
        /// Permissions to disable a user's ability to talk (aka mute them).
        const MUTE_MEMBERS =        1 << 6;
    }
}

serde_for_bitflags!(Permissions);
