use bitflags::bitflags;

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct Intents: u64 {
        /// Messages in a guild.
        const GUILD_MESSAGES = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
        /// Messages in DMs.
        const DM_MESSAGES =    0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
        /// Shorthand for `GUILD_MESSAGES | DM_MESSAGES`.
        const MESSAGES =       Self::GUILD_MESSAGES.bits | Self::DM_MESSAGES.bits;

        /// All guild events. This includes creation, deletion, and others.
        const GUILDS =         0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100;

        /// All channel update events. This includes creation, deletion, and others.
        const CHANNELS =       0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000;

        /// Member creation events.
        const MEMBER_CREATE =  0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000;
        /// Member deletion/removal events.
        const MEMBER_DELETE =  0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000;
        /// Member update events.
        const MEMBER_UPDATE =  0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000;
        /// All member-related events. Shorthand for `MEMBER_CREATE | MEMBER_DELETE | MEMBER_UPDATE`.
        const MEMBER_EVENT =   Self::MEMBER_CREATE.bits | Self::MEMBER_DELETE.bits | Self::MEMBER_UPDATE.bits;
    }
}
