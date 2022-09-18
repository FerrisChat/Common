pub mod auth;
pub mod channel;
pub mod guild;
pub mod user;

pub use auth::*;
pub use channel::*;
pub use guild::*;
pub use user::*;

use crate::crate_prelude::*;
use serde::{Deserialize, Serialize};

/// An error response.
#[derive(CastSnowflakes, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Error<Id: Snowflake = u128> {
    /// Internal server error occured, this is likely a bug.
    InternalError {
        /// What caused the error. `None` if unknown.
        what: Option<&'static str>,
        /// The error message.
        message: String,
        /// A debug version of the error, or `None` if there is no debug version.
        debug: Option<String>,
    },
    /// Could not resolve a plausible IP address from the request.
    MalformedIp {
        /// The error message.
        message: &'static str,
    },
    /// You are sending requests too quickly are you are being rate limited.
    Ratelimited {
        /// How long you should wait before sending another request, in whole seconds.
        retry_after: f32,
        /// The IP address that is being rate limited.
        ip: String,
        /// The ratelimited message.
        message: String,
    },
    /// The entity was not found.
    NotFound {
        /// The type of item that couldn't be found.
        entity: &'static str,
        /// The error message.
        message: String,
    },
    /// Tried authorizing a bot account with anything but an authentication token.
    UnsupportedAuthMethod {
        /// The error message.
        message: &'static str,
    },
    /// Invalid login credentials were provided, i.e. an invalid password.
    InvalidCredentials {
        /// Which credential was invalid.
        what: &'static str,
        /// The error message.
        message: &'static str,
    },
    /// Something was already taken, e.g. a username or email.
    AlreadyTaken {
        /// What was already taken.
        what: &'static str,
        /// The error message.
        message: String,
    },
    /// The request required a valid authentication token, but one of the following happened:
    ///
    /// * The token was not provided.
    /// * The token was malformed, i.e. a non-UTF-8 string.
    /// * The token does not exist or is invalid.
    InvalidToken {
        /// The error message.
        message: &'static str,
    },
    /// Validation error for a field.
    ValidationError {
        /// The field that failed validation.
        field: &'static str,
        /// The error message.
        message: String,
    },
    /// You tried accessing or modifying data of a guild the user is not a member of.
    NotMember {
        /// The ID of the guild you are not a member of.
        guild_id: Id,
        /// The error message.
        message: &'static str,
    },
    /// Missing permissions to perform an action.
    MissingPermissions {
        /// The error message.
        message: &'static str,
    },
    /// You must be the owner of the guild to perform this action.
    NotOwner {
        /// The ID of the guild you are not the owner of.
        guild_id: Id,
        /// The error message.
        message: &'static str,
    },
    /// You are missing the request body in an endpoint that requires it. This is commonly JSON or
    /// MsgPack.
    MissingBody {
        /// The error message.
        message: &'static str,
    },
    /// You are missing a required field in the request body.
    MissingField {
        /// The name of the missing field.
        field: &'static str,
        /// The error message.
        message: &'static str,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HybridSnowflake {
    U128(u128),
    String(String),
}

impl Snowflake for HybridSnowflake {
    fn to_u128(&self) -> u128 {
        match self {
            Self::U128(id) => *id,
            Self::String(id) => id.parse().unwrap(),
        }
    }
}

impl ToString for HybridSnowflake {
    fn to_string(&self) -> String {
        match self {
            Self::U128(id) => id.to_string(),
            Self::String(id) => id.clone(),
        }
    }
}
