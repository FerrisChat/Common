//! A collection of common types used across FerrisChat's codebase.

#![allow(
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::wildcard_imports
)]

pub mod models;

/// A trait that is implemented on all types that can be used as a snowflake.
pub trait Snowflake: Clone {}
impl Snowflake for u128 {}
impl Snowflake for String {}

/// A model that can be converted between stringified snowflakes and u128 snowflakes.
pub trait CastSnowflakes {
    type U128Result: CastSnowflakes;
    type StringResult: CastSnowflakes;

    /// Convert this model into one with u128 snowflakes.
    fn into_u128_ids(self) -> Self::U128Result
    where
        Self: Sized;

    /// Convert this model into one with stringified snowflakes.
    fn into_string_ids(self) -> Self::StringResult
    where
        Self: Sized;
}

impl CastSnowflakes for Vec<u128> {
    type U128Result = Self;
    type StringResult = Vec<String>;

    fn into_u128_ids(self) -> Self::U128Result {
        self
    }

    fn into_string_ids(self) -> Self::StringResult {
        self.into_iter().map(|id| id.to_string()).collect()
    }
}

impl CastSnowflakes for Vec<String> {
    type U128Result = Vec<u128>;
    type StringResult = Self;

    fn into_u128_ids(self) -> Self::U128Result {
        self.into_iter().map(|id| id.parse().unwrap()).collect()
    }

    fn into_string_ids(self) -> Self::StringResult {
        self
    }
}

impl<T: CastSnowflakes> CastSnowflakes for Vec<T> {
    type U128Result = Vec<T::U128Result>;
    type StringResult = Vec<T::StringResult>;

    fn into_u128_ids(self) -> Self::U128Result {
        self.into_iter().map(CastSnowflakes::into_u128_ids).collect()
    }

    fn into_string_ids(self) -> Self::StringResult {
        self.into_iter().map(CastSnowflakes::into_string_ids).collect()
    }
}

impl<T: CastSnowflakes> CastSnowflakes for Option<T> {
    type U128Result = Option<T::U128Result>;
    type StringResult = Option<T::StringResult>;

    fn into_u128_ids(self) -> Self::U128Result {
        self.map(CastSnowflakes::into_u128_ids)
    }

    fn into_string_ids(self) -> Self::StringResult {
        self.map(CastSnowflakes::into_string_ids)
    }
}

#[macro_export]
macro_rules! serde_for_bitflags {
    ($t:ty) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u32(self.bits)
            }
        }

        impl<'de> serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self {
                    bits: deserializer.deserialize_u32($crate::U32Visitor)?,
                })
            }
        }
    };
}

pub(crate) struct U32Visitor;
impl serde::de::Visitor<'_> for U32Visitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 0 and 2^32 - 1")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// A prelude of commonly used traits. This is usually used with a glob-import, i.e.
/// `use common::prelude::*;`.
pub mod prelude {
    pub use crate::{CastSnowflakes, Snowflake, Timestamp};
}

pub(crate) mod crate_prelude {
    pub use crate::{CastSnowflakes, Snowflake};
    pub use macros::CastSnowflakes;
    pub use serde_for_bitflags;
}
