use crate::types::{Channel, Embed, User};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Deserialize, Clone)]
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
    pub channel: Channel,

    /// The ID of the channel this message is in
    pub channel_id: u128,

    /// The author ID
    ///
    /// 128 bit unsigned integer
    pub author_id: u128,

    /// The resolved data of the author of this message
    pub author: Option<User>,

    /// The last time this message was edited
    ///
    /// None if it was never edited, otherwise a UTC datetime.
    pub edited_at: Option<time::PrimitiveDateTime>,

    /// A list of embeds in the message.
    ///
    /// Maximum 10 embeds
    pub embeds: Vec<Embed>,

    /// The nonce of the message
    ///
    /// Is not stored on db so it can be None.
    pub nonce: Option<String>,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("Message", 9)?;

        self_ser.serialize_field("id", &self.id)?;
        self_ser.serialize_field("id_string", &self.id.to_string())?;

        self_ser.serialize_field("content", &self.content)?;

        self_ser.serialize_field("channel", &self.channel)?;
        self_ser.serialize_field("channel_id", &self.channel_id)?;
        self_ser.serialize_field("channel_id_string", &self.channel_id.to_string())?;

        self_ser.serialize_field("author_id", &self.author_id)?;
        self_ser.serialize_field("author_id_string", &self.author_id.to_string())?;
        self_ser.serialize_field("author", &self.author)?;

        self_ser.serialize_field("edited_at", &self.edited_at)?;
        self_ser.serialize_field("embeds", &self.embeds)?;

        self_ser.serialize_field("nonce", &self.nonce)?;

        self_ser.end()
    }
}

#[derive(Serialize, Deserialize)]
pub struct MessageHistory {
    /// Vec of messages
    pub messages: Vec<Message>,
}
