//! Common data structures used by FerrisChat.

pub mod guild;
pub mod permissions;
pub mod user;

pub use guild::*;
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
