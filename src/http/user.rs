use crate::crate_prelude::*;
use serde::{Deserialize, Serialize};

/// Payload sent to create a new user.
#[derive(Clone, Debug, Deserialize)]
pub struct CreateUserPayload {
    /// The username of the user. Must be between 2 and 32 characters.
    pub username: String,
    /// The email of the user. Must be a valid email address.
    pub email: String,
    /// The password of the user. Must be between 8 and 32 characters.
    pub password: String,
}

/// Data returned when creating a new user.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct CreateUserResponse<Id: Snowflake = u128> {
    /// The ID of the user.
    pub id: Id,
    /// The token to use for authentication.
    pub token: String,
}
