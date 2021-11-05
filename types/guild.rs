use crate::types::Member;
use crate::types::{Channel, Role};
use bitflags::bitflags;
use serde::ser::SerializeStruct;

#[derive(Deserialize, Clone)]
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

    /// Role list
    ///
    /// Not sent at times to reduce bandwidth usage
    pub roles: Option<Vec<Role>>,

    /// Guild flags
    ///
    /// Bitmask of guild info
    pub flags: GuildFlags,
}

impl Serialize for Guild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("Guild", 9)?;

        self_ser.serialize_field("id", &self.id)?;
        self_ser.serialize_field("id_string", &self.id.to_string())?;

        self_ser.serialize_field("owner_id", &self.owner_id)?;
        self_ser.serialize_field("owner_id_string", &self.owner_id.to_string())?;

        self_ser.serialize_field("name", &self.name)?;
        self_ser.serialize_field("channels", &self.channels)?;
        self_ser.serialize_field("members", &self.members)?;
        self_ser.serialize_field("roles", &self.roles)?;
        self_ser.serialize_field("flags", &self.flags)?;
        self_ser.end()
    }
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
