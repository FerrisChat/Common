use crate::types::Channel;
use crate::types::Member;
use bitflags::bitflags;

#[derive(Serialize, Deserialize, Clone)]
pub struct Guild {
    /// Guild ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// Owner ID
    ///
    /// 128 bit unsigned integer
    pub owner_id: u128,

    /// Guild name
    ///
    /// String up to 100 characters long.
    pub name: String,

    /// Channels list
    ///
    /// May not be sent at times to reduce bandwidth usage.
    pub channels: Option<Vec<Channel>>,

    /// Member list
    ///
    /// Not sent at times to reduce bandwidth usage
    pub members: Option<Vec<Member>>,

    /// Guild flags
    ///
    /// Bitmask of guild info
    pub flags: i64,
}

bitflags! {
    #[derive(Default)]
    pub struct GuildFlags: i64 {
        /// This guild's owner has been verified (i.e. for content creators, etc.)
        const VERIFIED_GUILD = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
        /// This guild has been reported and confirmed as promoting scams/other potentially harmful content
        const VERIFIED_SCAM  = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
    }
}

serde_for_bitflags!(GuildFlags);
