use super::PermissionPair;
use crate::crate_prelude::*;
use serde::{Deserialize, Serialize};

#[derive(CastSnowflakes, Clone, Debug, Deserialize, Serialize)]
pub struct PermissionOverwrite<Id: Snowflake = u128> {
    /// The ID of the role or user this overwrite applies to. The model type can be extracted from
    /// the ID.
    id: Id,
    /// The permissions this overwrite grants or denies.
    #[serde(flatten)]
    permissions: PermissionPair,
}

/// Represents common information about a guild channel.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct PartialGuildChannel<Id: Snowflake = u128> {
    /// The ID of the channel.
    pub id: Id,
    /// The ID of the guild this channel is in.
    pub guild_id: Id,
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
}

/// Represents a channel.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Channel<Id: Snowflake = u128> {
    /// A guild text channel.
    Text {
        /// Common information about the channel.
        #[serde(flatten)]
        partial: PartialGuildChannel<Id>,
        /// The topic or description about the channel.
        topic: Option<String>,
        /// The URL of the small icon of the channel. `None` if the channel has no icon.
        icon: Option<String>,
        /// The slowmode delay of the channel, in **milliseconds**. This should be a value between
        /// `0` and `86_400_000` (24 hours).
        slowmode: u32,
        /// Whether the channel is locked. Only people with the `MANAGE_CHANNELS` permission can
        /// send messages in locked channels.
        locked: bool,
        /// Whether the channel is considered NSFW.
        nsfw: bool,
        /// The last ID of the previous message sent in the channel.
        last_message_id: Option<Id>,
    },
    /// A guild announcement channel.
    Announcement {
        /// Common information about the channel.
        #[serde(flatten)]
        partial: PartialGuildChannel<Id>,
        /// The topic or description about the channel.
        topic: Option<String>,
        /// The URL of the small icon of the channel. `None` if the channel has no icon.
        icon: Option<String>,
        /// The slowmode delay of the channel, in **milliseconds**. This should be a value between
        /// `0` and `86_400_000` (24 hours).
        slowmode: u32,
        /// Whether the channel is locked. Only people with the `MANAGE_CHANNELS` permission can
        /// send messages in locked channels.
        locked: bool,
        /// Whether the channel is considered NSFW.
        nsfw: bool,
        /// The last ID of the previous message sent in the channel.
        last_message_id: Option<Id>,
    },
    /// A guild voice channel.
    Voice {
        /// Common information about the channel.
        #[serde(flatten)]
        partial: PartialGuildChannel<Id>,
        /// The maximum number of users that can be in the channel at once. This is `None` for
        /// non-limited channels, else it should be a number between `1` and `500`.
        user_limit: Option<u16>,
    },
    /// A guild category.
    Category {
        /// Common information about the category.
        #[serde(flatten)]
        partial: PartialGuildChannel<Id>,
        /// The URL of the small icon of the channel. `None` if the category has no icon.
        icon: Option<String>,
    },
    /// A direct message channel to another user.
    DM {
        /// The ID of the channel.
        id: Id,
        /// The recipient of the DM.
        recipient_id: Id,
        /// The last ID of the previous message sent in the channel.
        last_message_id: Option<Id>,
    },
    /// A group DM channel.
    GroupDM {
        /// The ID of the channel.
        id: Id,
        /// The owner ID of the group DM.
        owner_id: Id,
        /// The IDs of the recipients of the group DM.
        recipient_ids: Vec<Id>,
        /// The topic or description about the channel.
        topic: Option<String>,
        /// The URL of the small icon of the channel. `None` if the group DM has no icon.
        icon: Option<String>,
        /// The last ID of the previous message sent in the channel.
        last_message_id: Option<Id>,
    },
}
