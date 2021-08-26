use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Permissions: i64 {
        /// Permissions to send messages.
        const SEND_MESSAGES =       0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
        /// Permissions to delete messages.
        const DELETE_MESSAGES =     0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
        /// Permissions to edit channels.
        const EDIT_CHANNELS =       0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100;
        /// Permissions to add/remove channels.
        const ADD_REMOVE_CHANNELS = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000;
        /// Permissions to remove members (aka kick them).
        const REMOVE_MEMBERS =      0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000;
    }
}
