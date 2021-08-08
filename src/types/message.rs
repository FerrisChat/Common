use crate::types::Channel;

#[derive(Serialize, Deserialize)]
pub struct Message {
    /// The ID of the message
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The content of the message
    ///
    /// String of max length 10,240 characters
    pub content: String,

    /// The channel this message is in
    pub channel: Channel,
}
