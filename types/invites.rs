use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct Invite {
    /// The invite's unique code (string)
    pub code: String,

    /// The invite owner's user ID
    ///
    /// 128 bit unsigned integer
    pub owner_id: u128,

    /// The invite's guild ID
    ///
    /// 128 bit unsigned integer
    pub guild_id: u128,

    /// When the invite was created as a timezone-naive datetime
    pub created_at: i64,

    /// How many times the invite was used
    pub uses: i32,

    /// The maximum amount of times the invite can be used before being invalidated
    pub max_uses: Option<i16>,

    /// The amount of seconds from the time the invite was created until it will expire
    pub max_age: Option<i64>,
}

impl Serialize for Invite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("Invite", 9)?;

        self_ser.serialize_field("code", &self.code)?;

        self_ser.serialize_field("owner_id", &self.owner_id)?;
        self_ser.serialize_field("owner_id_string", &self.owner_id.to_string())?;

        self_ser.serialize_field("guild_id", &self.guild_id)?;
        self_ser.serialize_field("guild_id_string", &self.guild_id.to_string())?;

        self_ser.serialize_field("created_at", &self.created_at)?;
        self_ser.serialize_field("uses", &self.uses)?;
        self_ser.serialize_field("max_uses", &self.max_uses)?;
        self_ser.serialize_field("max_age", &self.max_age)?;

        self_ser.end()
    }
}

