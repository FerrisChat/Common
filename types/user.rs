use crate::types::Guild;

pub struct User {
    /// The user's ID
    ///
    /// 64 bit signed integer
    pub id: i64,

    /// The user's name
    ///
    /// String of max length 100 characters
    pub name: String,

    /// The list of guilds the user is in
    ///
    /// Only sent to the user.
    pub guilds: Option<Vec<Guild>>

    /// User flags
    ///
    /// Bitmask of user info
    /// 0:
    pub flags: i64,
}