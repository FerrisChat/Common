use crate::types::User;

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

    /// The users inside the channel
    ///
    /// Vec of `User`s
    pub users: Vec<User>,
}
