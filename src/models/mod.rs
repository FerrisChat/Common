//! Common data structures used by FerrisChat.

pub mod channel;
pub mod guild;
pub mod message;
pub mod permissions;
pub mod role;
pub mod user;

pub use channel::*;
pub use guild::*;
pub use message::*;
pub use permissions::*;
pub use role::*;
pub use user::*;

/// An enumeration for the type of a model, used in a snowflake.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ModelType {
    /// The model is a guild.
    Guild = 0,
    /// The model is a user account.
    User = 1,
    /// The model is a channel.
    Channel = 2,
    /// The model is a message.
    Message = 3,
    /// The model is a role.
    Role = 4,
    /// The model is used internally, e.g. a nonce.
    Internal = 5,
    /// Unknown model.
    Unknown = !0,
}

impl ModelType {
    /// Returns the corresponding model type for the given integer.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Guild,
            1 => Self::User,
            2 => Self::Channel,
            3 => Self::Message,
            4 => Self::Role,
            5 => Self::Internal,
            _ => Self::Unknown,
        }
    }
}
