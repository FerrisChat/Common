use crate::types::User;
use serde::{Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct DMChannel {
    /// The channel ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The channel's name
    /// Only applies to Group DMs
    /// String of max length 100 chars
    pub name: Option<String>,

    /// The users inside the DM channel
    ///
    /// Vec of `User`s
    pub users: Vec<User>,

    /// Boolean indicating whether the DM being created is a group or not.
    /// If false is passed, it will be a regular direct message between 2 people
    /// true/false
    pub group: bool,
}

impl Serialize for DMChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("DMChannel", 4)?;

        self_ser.serialize_field("id", &self.id)?;
        self_ser.serialize_field("id_string", &self.id.to_string())?;

        self_ser.serialize_field("name", &self.name)?;

        self_ser.serialize_field("users", &self.users)?;
        self_ser.end()
    }
}
