use super::user::User;
use crate::crate_prelude::*;
use serde::Serialize;

/// Represents a member of a guild. Members are user objects associated with a guild.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct Member<Id: Snowflake = u128> {
    /// The user associated with this member.
    #[serde(flatten)]
    pub user: User<Id>,
    /// The ID of the guild this member is in.
    pub guild_id: Id,
    /// The nickname of the member. `None` if the member has no nickname.
    pub nick: Option<String>,
    /// A list of IDs of the roles that the member has.
    pub roles: Vec<Id>,
    /// The time that the member joined the guild.
    pub joined_at: (), // TODO: decide a type for timestamps
}

impl<Id: Snowflake> Member<Id> {
    /// The display name of the member. This is the nickname if the member has one,
    /// else the username.
    pub fn display_name(&self) -> &str {
        self.nick.as_deref().unwrap_or(&self.user.username)
    }
}

/// Represents a guild with partial information, sometimes referred to as a server.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct PartialGuild<Id: Snowflake = u128> {
    /// The snowflake ID of the guild.
    pub id: Id,
    /// The name of the guild.
    pub name: String,
    /// The URL of the icon of the guild.
    pub icon: Option<String>,
    /// The ID of the owner of the guild.
    pub owner_id: Id,
}

/// Represents a guild with all information, sometimes referred to as a server.
#[derive(Clone, Debug, Serialize)]
pub struct Guild<Id: Snowflake = u128> {
    /// The information available to partial guilds, including the name and ID.
    #[serde(flatten)]
    pub partial: PartialGuild<Id>,
    /// The resolved owner as a user object. This is only available when fetching the guild directly
    /// or when the client receives a ready event containing all guild data through the gateway.
    pub owner: Option<User<Id>>,
    /// A list of resolved members in the guild. This is only available when fetching the guild
    /// directly or when the client receives a ready event containing all guild data through the
    /// gateway.
    pub members: Option<Vec<Member<Id>>>,
}

impl<Id: Snowflake> Guild<Id> {
    /// Returns a reference to the snowflake ID of the guild.
    pub const fn id(&self) -> &Id {
        &self.partial.id
    }

    /// Returns a string slice of the name of the guild.
    pub fn name(&self) -> &str {
        &self.partial.name
    }

    /// Returns the URL of the icon of the guild.
    pub fn icon(&self) -> Option<&str> {
        self.partial.icon.as_deref()
    }

    /// Returns a reference to the snowflake ID of the owner of the guild.
    pub const fn owner_id(&self) -> &Id {
        &self.partial.owner_id
    }
}

impl CastSnowflakes for Guild<u128> {
    type U128Result = Self;
    type StringResult = Guild<String>;

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
        Guild {
            partial: self.partial.into_string_ids(),
            owner: self.owner.map(CastSnowflakes::into_string_ids),
            members: self.members.map(|members| {
                members
                    .into_iter()
                    .map(CastSnowflakes::into_string_ids)
                    .collect()
            }),
        }
    }
}

impl CastSnowflakes for Guild<String> {
    type U128Result = Guild<u128>;
    type StringResult = Self;

    fn into_u128_ids(self) -> Self::U128Result
    where
        Self: Sized,
    {
        Guild {
            partial: self.partial.into_u128_ids(),
            owner: self.owner.map(CastSnowflakes::into_u128_ids),
            members: self.members.map(|members| {
                members
                    .into_iter()
                    .map(CastSnowflakes::into_u128_ids)
                    .collect()
            }),
        }
    }

    fn into_string_ids(self) -> Self::StringResult
    where
        Self: Sized,
    {
        self
    }
}
