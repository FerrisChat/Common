///! Utility functions for crates that rely on this library.
use crate::models::{ModelType, PermissionOverwrite, Permissions, Role};
use chrono::{TimeZone, Utc};

/// The start of the Ferris Epoch in milliseconds since the Unix Epoch.
///
/// Retrieved from https://github.com/FerrisChat/SnowflakeGenerator/blob/master/src/lib.rs#L32.
pub const FERRIS_EPOCH: u128 = 1_640_995_200_000;

/// Returns a [`chrono::DateTime`] representing when the provided snowflake was created.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn snowflake_to_datetime(snowflake: u128) -> crate::Timestamp {
    Utc.timestamp_millis(((snowflake >> 64) + FERRIS_EPOCH) as i64)
}

/// Returns the model type of the provided snowflake.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn snowflake_to_model_type(snowflake: u128) -> ModelType {
    ModelType::from_u8((snowflake >> 56) as u8)
}

/// Calculates the permissions after applying all role permissions and channel overwrites.
///
/// # Parameters
/// * `user_id` - The ID of the user to calculate permissions for.
/// * `roles` - The roles the user has.
/// * `overwrites` - The channel overwrites, or `None` to apply no overwrites.
#[must_use]
pub fn calculate_permissions(
    user_id: u128,
    roles: &mut [Role],
    overwrites: Option<&[PermissionOverwrite]>,
) -> Permissions {
    let base = Permissions::empty();
    roles.sort_by_key(|r| r.position);

    let mut perms = roles
        .iter()
        .fold(base, |acc, role| acc | role.permissions.allow);
    perms &= !roles
        .iter()
        .fold(base, |acc, role| acc | role.permissions.deny);

    // currently, administrator acts after denied perms, meaning administrator does *not* take
    // precedence when a higher role denies the administrator permission. this could change in the
    // future
    if perms.contains(Permissions::ADMINISTRATOR) {
        return Permissions::all();
    }

    if let Some(overwrites) = overwrites {
        let mut role_overwrites = overwrites
            .iter()
            .filter_map(|o| roles.iter().find(|r| r.id == o.id).map(|r| (o, r.position)))
            .collect::<Vec<_>>();

        role_overwrites.sort_by_key(|(_, pos)| *pos);

        for (overwrite, _) in role_overwrites {
            perms |= overwrite.permissions.allow;
            perms &= !overwrite.permissions.deny;
        }

        if let Some(o) = overwrites.iter().find(|o| o.id == user_id) {
            perms |= o.permissions.allow;
            perms &= !o.permissions.deny;
        }
    }

    perms
}
