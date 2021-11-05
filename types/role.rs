use crate::perms::Permissions;
use crate::types::Guild;

#[derive(Serialize, Deserialize, Clone)]
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

    /// The role permissions
    ///
    /// Bitflags representing permission bits
    pub permissions: Permissions,
}