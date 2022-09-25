#![allow(clippy::use_self)]

use super::{Member, User};
use crate::crate_prelude::*;
use serde::Serialize;

/// The type of a message embed.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbedType {
    /// A custom, rich embed that is manually constructed. This is the only type that is available
    /// when creating a message. Other types of embeds are resolved automatically.
    Rich,
    /// An embedded image, likely from an image link. This is *not* an attachment.
    Image,
    /// An embedded video, likely from a video link. This is *not* a video attachment.
    Video,
    /// An embed resolved from the `meta` tags from a website's HTML head tags.
    Meta,
}

/// The author information of a message embed.
#[derive(Clone, Debug, Serialize)]
pub struct EmbedAuthor {
    /// The name of the author.
    pub name: String,
    /// The URL of the author, shown as a hyperlink of the author's name.
    pub url: Option<String>,
    /// The URL of the author's icon.
    pub icon_url: Option<String>,
}

/// The footer information of a message embed.
#[derive(Clone, Debug, Serialize)]
pub struct EmbedFooter {
    /// The text of the footer.
    pub text: String,
    /// The URL of the footer's icon.
    pub icon_url: Option<String>,
}

/// The alignment type of a message embed field.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum MessageEmbedFieldAlignment {
    /// The field is aligned to the left.
    Left,
    /// The field is centered.
    Center,
    /// The field is aligned to the right.
    Right,
    /// The field is displayed inline to other inline fields.
    /// This is the default.
    #[default]
    Inline,
}

/// Information about an embed's field.
#[derive(Clone, Debug, Serialize)]
pub struct EmbedField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: String,
    /// The alignment of the field.
    #[serde(default)]
    pub align: MessageEmbedFieldAlignment,
}

/// Represents a special card shown in the UI for various purposes, embedding extra information
/// to the user in a more visually appealing way. These are known as embeds and are used in
/// messages.
#[derive(Clone, Debug, Serialize)]
pub struct Embed {
    /// The type of the embed.
    #[serde(rename = "type")]
    pub kind: EmbedType,
    /// The title of the embed.
    pub title: Option<String>,
    /// The description, or body text of the embed.
    pub description: Option<String>,
    /// The URL of the embed, shown as a hyperlink in the title. Only available if the embed has a
    /// title.
    pub url: Option<String>,
    /// The timestamp of the embed.
    pub timestamp: Option<crate::Timestamp>,
    /// The color of the embed, shown as a stripe on the left side of the embed.
    pub color: Option<u32>,
    /// The hue of the main body of the background. This is only available for rich embeds. This
    /// should be a number between `0` and `100`, measured as a percentage.
    pub hue: Option<u8>,
    /// The author of the embed.
    pub author: EmbedAuthor,
    /// The footer of the embed.
    pub footer: EmbedFooter,
    /// The image URL of the embed.
    pub image: Option<String>,
    /// The thumbnail URL of the embed.
    pub thumbnail: Option<String>,
    /// A list of fields in the embed.
    pub fields: Option<Vec<EmbedField>>,
}

/// Represents a message attachment.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct Attachment<Id: Snowflake = u128> {
    /// The snowflake ID of the attachment.
    pub id: Id,
    /// The filename of the attachment.
    pub filename: String,
    /// The description/alt text of the attachment.
    pub description: Option<String>,
    /// The size of the attachment, in bytes.
    pub size: u64,
    /// The CDN URL of the attachment.
    pub url: String,
}

/// Represents the type of a message.
#[derive(CastSnowflakes, Clone, Copy, Debug, Default, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MessageType<Id: Snowflake = u128> {
    /// A normal message.
    #[default]
    Default,
    /// A join message, sent when a user joins either a group DM or a guild.
    Join {
        /// The ID of the user who joined.
        user_id: Id,
    },
    /// A leave message, sent when a user leaves either a group DM or a guild.
    Leave {
        /// The ID of the user who left.
        user_id: Id,
    },
    /// A message that indicates another message has been pinned.
    Pin {
        /// The ID of the message that was pinned.
        pinned_message_id: Id,
        /// The ID of the user that pinned the message.
        pinned_by: Id,
    },
}

/// Represents either a member or a user.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum MemberOrUser<Id: Snowflake = u128> {
    /// A member.
    Member(Member<Id>),
    /// A user.
    User(User<Id>),
}

/// Represents a text or system message in a channel.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct Message<Id: Snowflake = u128> {
    /// The snowflake ID of the message.
    pub id: Id,
    /// The revision ID of the message. This is `None` if this message is the current revision.
    pub revision_id: Option<Id>,
    /// The snowflake ID of the channel this message was sent in.
    pub channel_id: Id,
    /// The snowflake ID of the guild this message was sent in, if any.
    pub guild_id: Option<Id>,
    /// The snowflake ID of the author of this message, or `None` if this is a system message.
    pub author_id: Option<Id>,
    /// Resolved data about the user or member that sent this message.
    /// This is only present for new messages that are received.
    pub author: Option<MemberOrUser<Id>>,
    /// The type of this message.
    #[serde(flatten)]
    pub kind: MessageType,
    /// The text content of this message.
    pub content: Option<String>,
    /// A list of embeds included in this message.
    pub embeds: Vec<Embed>,
    /// A list of attachments included in this message.
    pub attachments: Vec<Attachment<Id>>,
    /// A bitmask of message flags to indicate special properties of the message.
    pub flags: MessageFlags,
    /// The amount of stars this message has received.
    pub stars: u32,
    /// A custom nonce for this message. This is a random string that if used, a message with the
    /// same nonce will be dispatched by the websocket, indicating that the message was sent.
    ///
    /// This is only used once and it is not stored.
    pub nonce: Option<String>,
}

bitflags::bitflags! {
    /// A bitmask of message flags to indicate special properties of the message.
    #[derive(Default)]
    pub struct MessageFlags: u32 {
        /// The message is pinned.
        const PINNED = 1 << 0;
        /// The message is a system message.
        const SYSTEM = 1 << 1;
        /// The message is a subscribed crosspost from an announcement channel.
        const CROSSPOST = 1 << 2;
        /// This message has been published to subscribed channels in an announcement channel.
        const PUBLISHED = 1 << 3;
    }
}

serde_for_bitflags!(u32: MessageFlags);