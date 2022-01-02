use crate::perms::ChannelPermissions;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct Channel {
    /// The channel ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The channel's name
    ///
    /// String of max length 100 chars
    pub name: String,

    /// The guild ID
    ///
    /// 128 bit unsigned integer
    pub guild_id: u128,

    /// The channel's permission overwrites
    ///
    /// Vec of (u128, ChannelPermissions)
    pub permission_overwrites: Vec<(u128, ChannelPermissions)>,
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("Channel", 6)?;

        self_ser.serialize_field("id", &self.id)?;
        self_ser.serialize_field("id_string", &self.id.to_string())?;

        self_ser.serialize_field("guild_id", &self.guild_id)?;
        self_ser.serialize_field("guild_id_string", &self.guild_id.to_string())?;

        self_ser.serialize_field("name", &self.name)?;

        self_ser.serialize_field("permissions_overwrites", &self.permission_overwrites)?;
        self_ser.end()
    }
}
