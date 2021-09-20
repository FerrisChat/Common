use crate::types::Embed;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    /// The ID of the message
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The content of the message
    ///
    /// String of max length 10,240 characters
    pub content: Option<String>,

    /// The channel this message is in
    pub channel_id: u128,

    /// The author ID
    ///
    /// 128 bit unsigned integer
    pub author_id: u128,

    /// The last time this message was edited
    ///
    /// None if it was never edited, otherwise a UTC datetime.
    pub edited_at: Option<time::PrimitiveDateTime>,

    /// A list of embeds in the message.
    ///
    /// Maximum 10 embeds
    pub embeds: Vec<Embed>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageHistory {
    /// Vec of messages
    pub messages: Vec<Message>,
}
