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

/// The request body sent to modify a channel.
#[derive(Clone, Debug, Deserialize)]
pub struct EditChannelPayload {
    /// The new name of the channel. If left blank, the name will not be changed. Takes effect for
    /// all channels except for user DMs.
    pub name: Option<String>,
    /// The new topic or description of the channel. Explicitly setting this to `None` will clear
    /// the topic. Only takes effect for text-based channels in guilds, or group chats.
    pub topic: Maybe<String>,
    /// The new icon URL of the channel. Explicitly setting this to `None` will clear the icon.
    /// Takes effect for all channels except for user DMs.
    pub icon: Maybe<String>,
    /// The new user limit of the voice channel. Explicitly setting this to `None` will remove the
    /// current limit, if there is any. Only takes effect for guild voice channels.
    pub user_limit: Maybe<u16>,
}

/// The payload used per channel to specify its new position data.
#[derive(CastSnowflakes, Clone, Debug, Deserialize)]
pub struct EditChannelPositionPayload<Id: Snowflake = u128> {
    /// The ID of the channel to modify.
    pub id: Id,
    /// The new position of the channel.
    pub position: u16,
    /// The new scope of the channel. If left blank, the scope will not be changed. If set to
    /// `None`, the channel will be moved to the root of the channel list.
    pub scope: Maybe<Id>,
}

/// The request body sent to modify channel positions.
#[derive(CastSnowflakes, Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct EditChannelPositionsPayload<Id: Snowflake = u128> {
    /// A list of channel positions to modify.
    pub positions: Vec<EditChannelPositionPayload<Id>>,
}
