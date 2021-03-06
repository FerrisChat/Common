use crate::types::{Guild, Pronouns};
use bitflags::bitflags;
use serde::ser::SerializeStruct;

#[derive(Deserialize, Clone)]
pub struct User {
    /// The user's ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The user's name
    ///
    /// String of max length 100 characters
    pub name: String,

    /// The user's avatar url
    pub avatar: Option<String>,

    /// The list of guilds the user is in
    ///
    /// Only sent to the user.
    pub guilds: Option<Vec<Guild>>,

    /// User flags
    ///
    /// Bitmask of user info
    pub flags: UserFlags,

    /// The user's discriminator
    ///
    /// 16 bit signed integer (will be 4 digits)
    pub discriminator: i16,

    /// User's preferred set of pronouns.
    pub pronouns: Option<Pronouns>,

    /// Is Bot
    ///
    /// Boolean. True if user is a bot
    pub is_bot: bool,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("User", 7)?;

        self_ser.serialize_field("id", &self.id)?;
        self_ser.serialize_field("id_string", &self.id.to_string())?;

        self_ser.serialize_field("name", &self.name)?;
        self_ser.serialize_field("avatar", &self.avatar)?;
        self_ser.serialize_field("guilds", &self.guilds)?;
        self_ser.serialize_field("flags", &self.flags)?;
        self_ser.serialize_field("discriminator", &self.discriminator)?;
        self_ser.serialize_field("pronouns", &self.pronouns)?;
        self_ser.serialize_field("is_bot", &self.is_bot)?;

        self_ser.end()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BotsOwnedByUser {
    /// Bots owned by the user.
    pub bots: Vec<User>,
}

bitflags! {
    #[derive(Default)]
    pub struct UserFlags: i64 {
        /// This account is a bot.
        const BOT_ACCOUNT =            1 << 0;
        /// This account is a verified scam.
        /// Verified is both verified by staff, and reported by a large amount of people.
        const VERIFIED_SCAM =          1 << 1;
        /// This account could possibly be a scam, as many users have reported it as such.
        const POSSIBLE_SCAM =          1 << 2;
        /// This account has had either its email address or token changed within the past 24 hours.
        /// It may not be controlled by its real owner, so take precautions when using mod actions against them.
        const COMPROMISED =            1 << 3;
        /// This account is a system account.
        const SYSTEM =                 1 << 4;
        /// This bot was one of the first 100 bots created on the platform.
        const EARLY_BOT =              1 << 5;
        /// This account is the owner of one of the first 100 bots created on the platform.
        const EARLY_BOT_DEV =          1 << 6;
        /// This account was one of the first 1,000 created on the platform.
        const EARLY_SUPPORTER =        1 << 7;
        /// This account is owned by someone who has donated to help keep the platform running, and support development.
        const DONATOR =                1 << 8;
        /// This account is owned by a maintainer of a API wrapper for the FerrisChat API in a language.
        const LIBRARY_DEV =            1 << 9;
        /// This account is owned by someone who has contributed to FerrisChat's codebase in some way.
        const CONTRIBUTOR =            1 << 10;
        /// This account is owned by a core developer/maintainer of FerrisChat itself.
        const MAINTAINER =             1 << 11;
        /// This account is owned by someone who has won an official FerrisChat event.
        const CHRISTMAS_EVENT_WINNER = 1 << 12;
        /// This account is owned by someone who has reported/discovered many important bugs in FerrisChat.
        const BUG_HUNTER             = 1 << 13;
    }
}

serde_for_bitflags!(UserFlags);
