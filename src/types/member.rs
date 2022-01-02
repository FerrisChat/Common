use crate::types::{Guild, User, Role};

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Deserialize, Clone)]
pub struct Member {
    pub user_id: Option<u128>,
    pub user: Option<User>,

    pub roles: Vec<Role>,

    pub guild_id: Option<u128>,
    pub guild: Option<Guild>,
}

impl Serialize for Member {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("Member", 6)?;

        self_ser.serialize_field("user_id", &self.user_id)?;
        self_ser.serialize_field("user_id_string", &self.user_id.map(|x| x.to_string()))?;
        self_ser.serialize_field("user", &self.user)?;

        self_ser.serialize_field("roles", &self.roles)?;

        self_ser.serialize_field("guild_id", &self.guild_id)?;
        self_ser.serialize_field("guild_id_string", &self.guild_id.map(|x| x.to_string()))?;
        self_ser.serialize_field("guild", &self.guild)?;

        self_ser.end()
    }
}
