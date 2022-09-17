pub mod auth;
pub mod guild;
pub mod user;

pub use auth::*;
pub use guild::*;
pub use user::*;

/// An error response.
#[derive(Debug, serde::Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Error {
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
}
