use crate::types::Channel;
use crate::types::Member;

#[derive(Serialize, Deserialize)]
pub struct Guild {
    /// Guild ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// Owner ID
    ///
    /// 128 bit unsigned integer
    pub owner_id: i64,

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
}
