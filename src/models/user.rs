#![allow(clippy::use_self)]

use crate::crate_prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a FerrisChat user account.
///
/// A lot of information is stored in the user's flags, including whether or not the user is a bot
/// account.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct User<Id: Snowflake = u128> {
    /// The snowflake ID of the user.
    pub id: Id,
    /// The username of the user.
    pub username: String,
    /// The discriminator of the user, between 0 and 9999.
    pub discriminator: u16,
    /// The URL of the user's avatar. This is `None` if the user has no avatar.
    pub avatar: Option<String>,
    /// A bitmask of extra information associated with this user.
    pub flags: UserFlags,
}

// TODO: should this be backwards compatible with the old bitflags?
bitflags::bitflags! {
    /// A bitmask of extra information associated with a user.
    #[derive(Default)]
    pub struct UserFlags: u32 {
        /// The user is a bot account.
        const BOT = 1 << 0;
        /// The user was flagged as a spam or compromised account by a large number of users.
        const POTENTIAL_SPAM = 1 << 1;
        /// The user was verified to be a spam or compromised account by a large number of users.
        ///
        /// Users with this flag cannot join any new guilds and cannot request new relationsihps
        /// with other users.
        const SPAM = 1 << 2;
        /// The user is a system account.
        const SYSTEM = 1 << 3;
        /// The bot was one of the first 100 bots created on FerrisChat.
        const EARLY_BOT = 1 << 4;
        /// The user owns a bot that was one of the first 100 bots created on FerrisChat.
        const EARLY_BOT_DEVELOPER = 1 << 5;
        /// The account was one of the first 1,000 non-bot accounts created on FerrisChat.
        const EARLY_SUPPORTER = 1 << 6;
        /// The user has donated to FerrisChat to support the project and keep the platform running.
        const DONATOR = 1 << 7;
        /// The account is owned by a maintainer of a library or framework that wraps around
        /// FerrisChat's API.
        const LIBRARY_DEVELOPER = 1 << 8;
        /// This account is owned by someone who has contributed to FerrisChat's codebase.
        const CONTRIBUTOR = 1 << 9;
        /// This account is owned by a core maintainer or developer of FerrisChat.
        const MAINTAINER = 1 << 10;
        /// This account is owned by someone who has reported or fixed a major bug in FerrisChat's
        /// codebase.
        const BUG_HUNTER = 1 << 11;
        /// The user is the winner of a FerrisChat event.
        const EVENT_WINNER = 1 << 12;
    }
}

serde_for_bitflags!(UserFlags);

/// Represents information such as the name and color of a guild folder.
/// This is only shown in the client's UI.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildFolderInfo {
    /// The name of the folder.
    pub name: String,
    /// The color of the folder.
    pub color: u32,
}

/// Represents a folder that contains a collection of guilds. This is only shown in the client's UI.
#[derive(CastSnowflakes, Clone, Debug, Deserialize, Serialize)]
pub struct GuildFolder<Id: Snowflake = u128> {
    /// The path of the folder, with the top-level folder first.
    ///
    /// This is `None` if this folder represents the collection of guilds
    /// that are not in any folders, or in other terms, the root folder.
    pub path: Option<Vec<GuildFolderInfo>>,
    /// A list of guild IDs representing guilds that were placed in this folder, in order from
    /// top to bottom.
    pub guilds: Vec<Id>,
}

/// Represents user info about the client. This has other information that is not available to the
/// public, such as emails, guilds, and relationships (friends and blocked users).
#[derive(Clone, Debug, Serialize)]
pub struct ClientUser<Id: Snowflake = u128> {
    /// The public user info about the client.
    #[serde(flatten)]
    pub user: User<Id>,
    /// The associated email of the client's account.
    ///
    /// If the client is a bot, this is `None`.
    pub email: Option<String>,
    /// A list of guilds that the client is a member of. This is a list of partial guilds that
    /// include information such as the guild's ID, name, icon, and owner.
    pub guilds: Vec<()>, // TODO
    /// A list of server folders that the user organized their guilds in via the client UI.
    ///
    /// Guilds that were not placed in any folders have a path of `None`. Else, the path is a list
    /// of folders that the guild is nested in, beginning with the top-level folder, for example
    /// `[{"name": "Top Level Folder", ...}, {"name": "Nested Folder", ...}]`.
    ///
    /// This is ordered in the ordering of the user's folders and servers.
    ///
    /// If the client of this user is a bot, this is `None`.
    pub folders: Option<Vec<GuildFolder<Id>>>,
    /// A list of relationships that the client has with other users.
    pub relationships: Vec<Relationship<Id>>,
}

impl CastSnowflakes for ClientUser<u128> {
    type U128Result = Self;
    type StringResult = ClientUser<String>;

    fn into_u128_ids(self) -> Self::U128Result
    where
        Self: Sized,
    {
        self
    }

    fn into_string_ids(self) -> Self::StringResult
    where
        Self: Sized,
    {
        ClientUser {
            user: self.user.into_string_ids(),
            email: self.email,
            guilds: self.guilds,
            folders: self.folders.map(|folders| {
                folders
                    .into_iter()
                    .map(CastSnowflakes::into_string_ids)
                    .collect()
            }),
            relationships: self
                .relationships
                .into_iter()
                .map(CastSnowflakes::into_string_ids)
                .collect(),
        }
    }
}

impl CastSnowflakes for ClientUser<String> {
    type U128Result = ClientUser<u128>;
    type StringResult = Self;

    fn into_u128_ids(self) -> Self::U128Result
    where
        Self: Sized,
    {
        ClientUser {
            user: self.user.into_u128_ids(),
            email: self.email,
            guilds: self.guilds,
            folders: self.folders.map(|folders| {
                folders
                    .into_iter()
                    .map(CastSnowflakes::into_u128_ids)
                    .collect()
            }),
            relationships: self
                .relationships
                .into_iter()
                .map(CastSnowflakes::into_u128_ids)
                .collect(),
        }
    }

    fn into_string_ids(self) -> Self::StringResult
    where
        Self: Sized,
    {
        self
    }
}

/// Represents the type of relationship a user has with another user.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    /// The user is added as a friend.
    Friend,
    /// The user is blocked.
    Blocked,
}

#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct Relationship<Id: Snowflake = u128> {
    /// The ID of the user that this relationship is with.
    pub id: Id,
    /// The type of relationship this is.
    #[serde(rename = "type")]
    pub kind: RelationshipType,
}
