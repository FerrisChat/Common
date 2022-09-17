use super::PermissionPair;
use crate::crate_prelude::*;
use serde::{Deserialize, Serialize};

/// Represents either a solid role color or a gradient of role colors.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RoleColor {
    /// A solid role color. This is an integer between 0 and 16777215.
    Solid {
        /// The color of the role.
        value: u32,
    },
    /// A left-to-right gradient of colors. The value is represented as an array of two-element
    /// tuples, where the first element is the color and the second element is the position of the
    /// color in the gradient represented as a whole number of percent between 0 and 100.
    ///
    /// For example, `{"type": "gradient", "value": [[0, 0], [16777215, 100]]}` would be a gradient
    /// from black to white.
    Gradient {
        /// The gradient of the role.
        value: Vec<(u32, u8)>,
    },
}

/// A role in a guild.
#[derive(CastSnowflakes, Clone, Debug, Serialize)]
pub struct Role<Id: Snowflake = u128> {
    /// The snowflake ID of the role.
    pub id: Id,
    /// The ID of the guild this role belongs to.
    pub guild_id: Id,
    /// The name of the role.
    pub name: String,
    /// The color of the role. Could be a solid color or a gradient. THis is `None` if the color is
    /// the default color.
    pub color: Option<RoleColor>,
    /// The permissions users with this role have.
    pub permissions: PermissionPair,
    /// The position of this role in the role hierarchy. The lower the number, the lower the role.
    /// The default role always has a position of 0.
    ///
    /// The backend will try its best to keep all role positions unique, but on the event two
    /// collide due to something such as a data race, then the true position of these roles will
    /// not be predictable, and will like be in the order of model creation.
    pub position: u16,
    /// A bitmask of flags representing extra metadata about the role.
    pub flags: RoleFlags,
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct RoleFlags: u32 {
        /// Whether the role is hoisted, or shown separately, in member list.
        const HOISTED = 1 << 0;
        /// Whether the role is managed. Managed roles cannot be edited or deleted.
        const MANAGED = 1 << 1;
        /// Whether the role is mentionable.
        const MENTIONABLE = 1 << 2;
        /// Whether the role is the default role for everyone.
        const DEFAULT = 1 << 3;
    }
}

serde_for_bitflags!(u32: RoleFlags);
