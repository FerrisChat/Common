///! Utility functions for crates that rely on this library.
use chrono::{TimeZone, Utc};

/// The start of the Ferris Epoch in milliseconds since the Unix Epoch.
///
/// Retrieved from https://github.com/FerrisChat/SnowflakeGenerator/blob/master/src/lib.rs#L32.
pub const FERRIS_EPOCH: u128 = 1_640_995_200_000;

/// Returns a [`chrono::DateTime`] representing when the provided snowflake was created.
#[must_use]
pub fn snowflake_to_datetime(snowflake: u128) -> crate::Timestamp {
    let ms = (snowflake >> 64) + FERRIS_EPOCH;

    #[allow(clippy::cast_possible_truncation)]
    Utc.timestamp_millis(ms as i64)
}
