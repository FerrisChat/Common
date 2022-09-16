#![allow(clippy::use_self)]

use super::PermissionPair;
use crate::crate_prelude::*;
use serde::{Deserialize, Serialize};

/// Represents any channel.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Channel<Id: Snowflake = u128> {
    Guild(GuildChannel<Id>),
    DM(DMChannel<Id>),
}

/// Represents a permission overwrite.
#[derive(CastSnowflakes, Clone, Debug, Deserialize, Serialize)]
pub struct PermissionOverwrite<Id: Snowflake = u128> {
    /// The ID of the role or user this overwrite applies to. The model type can be extracted from
    /// the ID.
    id: Id,
    /// The permissions this overwrite grants or denies.
    #[serde(flatten)]
    permissions: PermissionPair,
}

/// Represents the type of guild channel.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GuildChannelType {
    /// A normal text channel.
    Text,
    /// A text channel that has an announcement feed that can be subscribed to.
    Announcement,
    /// A voice channel.
    Voice,
}

/// Represents a channel in a guild.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct GuildChannel<Id: Snowflake = u128> {
    /// The ID of the channel.
    pub id: Id,
    /// The ID of the guild this channel is in.
    pub guild_id: Id,
    /// The type of the channel.
    #[serde(rename = "type")]
    pub kind: GuildChannelType,
    /// The name of the channel.
    pub name: String,
    /// The position of the channel in the channel list. A lower value means appearing "higher" in
    /// the UI, basically think of this as a 0-indexed listing of the channels from top-to-bottom.
    ///
    /// Positions are scoped per category, and categories have their own positions. Channels that
    /// lack a category will be shown above all categories. This is because no channels can be
    /// displayed in between or after categories - in the UI all non-category channels are displayed
    /// above any other category channels.
    ///
    /// For example:
    ///
    /// ```text
    /// [0] text-channel
    /// [1] voice-channel
    /// [2] another-text-channel
    /// [0] Category
    ///   [0] another-text-channel
    ///   [1] another-voice-channel
    ///   [0] Another Category
    ///     [1] nested-voice-channel
    ///     [2] nested-voice-channel-2
    /// [1] Yet Another Category
    ///   [0] another-text-channel
    /// ```
    pub position: u16,
    /// The permission overwrites for this channel.
    pub overwrites: Vec<PermissionOverwrite<Id>>,
    /// The ID of the parent category of the channel. `None` if the channel is not in a
    /// category.
    pub parent_id: Option<Id>,
    /// The topic or description about the channel. Only applies to text-based channels.
    pub topic: Option<String>,
    /// The URL of the small icon of the channel. `None` if the channel has no icon.
    pub icon: Option<String>,
    /// The slowmode delay of the channel, in **milliseconds**. This should be a value between
    /// `0` and `86_400_000` (24 hours). This is only available in text-based channels.
    pub slowmode: Option<u32>,
    /// Whether the channel is locked. Only people with the `MANAGE_CHANNELS` permission can
    /// send messages in locked channels. This is only available for text-based channels.
    pub locked: Option<bool>,
    /// Whether the channel is considered NSFW. This is only available for text-based channels.
    pub nsfw: Option<bool>,
    /// The last ID of the previous message sent in the channel. This is only available for
    /// text-based channels.
    pub last_message_id: Option<Id>,
    /// The maximum number of users that can be in the voice channel at once. This is `None` for
    /// non-limited channels, else it should be a number between `1` and `500`. Only available for
    /// voice channels.
    pub user_limit: Option<u16>,
}

/// Represents the type of DM channel.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DMChannelType {
    /// A normal DM channel.
    DM,
    /// A group chat consisting of multiple users.
    Group,
}

/// Represents a direct-message-like channel that does not live in a guild.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct DMChannel<Id: Snowflake = u128> {
    /// The ID of the channel.
    pub id: Id,
    /// The type of the channel.
    #[serde(rename = "type")]
    pub kind: DMChannelType,
    /// The ID of the last message sent in the channel.
    pub last_message_id: Option<Id>,
    /// The name of the group chat. Only applies to group DMs.
    pub name: Option<String>,
    /// The topic or description about the channel. Only available in group DMs.
    pub topic: Option<String>,
    /// The URL of the small icon of the channel. `None` if the group DM has no icon. This is only
    /// available in group DMs.
    pub icon: Option<String>,
    /// The ID of the user that owns the group DM. This is only available in group DMs.
    pub owner_id: Option<Id>,
    /// The user IDs of the recipients of the DM. For normal DMs, this is an array consisting of two
    /// elements - you and the other user. For group DMs, this is an array consisting of all the
    /// users in the group DM.
    pub recipient_ids: Vec<Id>,
}
