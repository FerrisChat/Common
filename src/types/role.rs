use crate::perms::GuildPermissions;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct Role {
    /// The role ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The id of the guild that the role belongs to
    ///
    /// 128 bit unsigned integer
    pub guild_id: u128,

    /// The role name
    ///
    /// String of up to 100 unicode characters
    pub name: String,

    /// The role color
    ///
    /// Integer between 0 and 16777215 (0xFFFFFF)
    pub color: Option<i32>,

    /// The role position
    ///
    /// Integer between 0 and 1023
    pub position: i16,

    /// The role guild permissions
    ///
    /// Bitflags representing permission bits
    pub guild_permissions: GuildPermissions,
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("Role", 8)?;

        self_ser.serialize_field("id", &self.id)?;
        self_ser.serialize_field("id_string", &self.id.to_string())?;

        self_ser.serialize_field("guild_id", &self.guild_id)?;
        self_ser.serialize_field("guild_id_string", &self.guild_id.to_string())?;

        self_ser.serialize_field("name", &self.name)?;
        self_ser.serialize_field("color", &self.color)?;
        self_ser.serialize_field("position", &self.position)?;
        self_ser.serialize_field("guild_permissions", &self.guild_permissions)?;

        self_ser.end()
    }
}
