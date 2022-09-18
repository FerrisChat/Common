use crate::crate_prelude::*;
use crate::models::{GuildChannelType, PermissionOverwrite};
use serde::Deserialize;

/// Common fields for all channel types when creating a channel.
#[derive(CastSnowflakes, Clone, Debug, Deserialize)]
pub struct PartialCreateChannelPayload<Id: Snowflake = u128> {
    /// The name of the text channel.
    pub name: String,
    /// The icon of the text channel, if any.
    pub icon: Option<String>,
    /// The ID of the category to create the channel in, if any.
    pub parent_id: Option<Id>,
    /// A list of permission overwrites to apply to the channel, if any.
    pub overwrites: Option<Vec<PermissionOverwrite<Id>>>,
    /// The position of the channel in the channel list. If one isn't provided, the position
    /// will be the last in its position scope.
    pub position: Option<u16>,
}

/// The request body sent to create a new channel in a guild.
#[derive(CastSnowflakes, Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum CreateChannelPayload<Id: Snowflake = u128> {
    /// A request to create a new guild text channel.
    Text {
        /// The inner payload.
        #[serde(flatten)]
        inner: PartialCreateChannelPayload<Id>,
        /// The topic of the text channel, if any.
        topic: Option<String>,
        /// The URL of the icon of the channel, if any.
        icon: Option<String>,
    },
    /// A request to create a new guild announcement channel.
    Announcement {
        /// The inner payload.
        #[serde(flatten)]
        inner: PartialCreateChannelPayload<Id>,
        /// The topic of the text channel, if any.
        topic: Option<String>,
        /// The URL of the icon of the channel, if any.
        icon: Option<String>,
    },
    /// A request to create a new guild voice channel.
    Voice {
        /// The inner payload.
        #[serde(flatten)]
        inner: PartialCreateChannelPayload<Id>,
        /// The maximum number of users that can be in the voice channel at once, or unlimited
        /// if left blank.
        user_limit: Option<u16>,
        /// The URL of the icon of the channel, if any.
        icon: Option<String>,
    },
    /// A request to create a new category.
    Category {
        /// The inner payload.
        #[serde(flatten)]
        inner: PartialCreateChannelPayload<Id>,
    },
}

impl<Id: Snowflake> CreateChannelPayload<Id> {
    /// Gets a reference to the inner payload.
    pub const fn inner(&self) -> &PartialCreateChannelPayload<Id> {
        match self {
            Self::Text { inner, .. }
            | Self::Announcement { inner, .. }
            | Self::Voice { inner, .. }
            | Self::Category { inner, .. } => inner,
        }
    }

    /// Gets the channel type of the channel.
    pub const fn kind(&self) -> GuildChannelType {
        match self {
            Self::Text { .. } => GuildChannelType::Text,
            Self::Announcement { .. } => GuildChannelType::Announcement,
            Self::Voice { .. } => GuildChannelType::Voice,
            Self::Category { .. } => GuildChannelType::Category,
        }
    }
}
