use crate::types::Guild;

#[derive(Serialize, Deserialize)]
pub struct User {
    /// The user's ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The user's name
    ///
    /// String of max length 100 characters
    pub name: String,

    /// The list of guilds the user is in
    ///
    /// Only sent to the user.
    pub guilds: Option<Vec<Guild>>,

    /// User flags
    ///
    /// Bitmask of user info
    pub flags: u128,
}
