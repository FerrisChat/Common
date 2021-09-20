use crate::types::Guild;
use bitflags::bitflags;

#[derive(Serialize, Deserialize, Clone)]
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
    pub flags: i64,

    /// The user's discriminator
    ///
    /// 16 bit signed integer (will be 4 digits)
    pub discriminator: i16,
}

bitflags! {
    #[derive(Default)]
    pub struct UserFlags: i64 {
        /// This account is a bot.
        const BOT_ACCOUNT =     0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
        /// This account is a verified scam.
        /// Verified is both verified by staff, and reported by a large amount of people.
        const VERIFIED_SCAM =   0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
        /// This account could possibly be a scam, as many users have reported it as such.
        const POSSIBLE_SCAM =   0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100;
        /// This account has had either its email address or token changed within the past 24 hours.
        /// It may not be controlled by its real owner, so take precautions when using mod actions against them.
        const COMPROMISED =     0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000;
        /// This account is a system account.
        const SYSTEM =          0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000;
        /// This bot was one of the first 100 bots created on the platform.
        const EARLY_BOT =       0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000;
        /// This account is the owner of one of the first 100 bots created on the platform.
        const EARLY_BOT_DEV =   0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000;
        /// This account was one of the first 1,000 created on the platform.
        const EARLY_SUPPORTER = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000;
        /// This account is owned by someone who has donated to help keep the platform running, and support development.
        const DONATOR =         0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000;
        /// This account is owned by a maintainer of a API wrapper for the FerrisChat API in a language.
        const LIBRARY_DEV =     0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000;
        /// This account is owned by someone who has contributed to FerrisChat's codebase in some way.
        const CONTRIBUTOR =     0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000;
        /// This account is owned by a core developer/maintainer of FerrisChat itself.
        const MAINTAINER =      0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000;
    }
}
