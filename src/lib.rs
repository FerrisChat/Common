//! A collection of common types used across FerrisChat's codebase.

#![allow(
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::wildcard_imports
)]

pub mod http;
pub mod models;
pub mod util;

/// A trait that is implemented on all types that can be used as a snowflake.
pub trait Snowflake: Clone + ToString + Send + Sync + Sized {
    /// Returns the ID as a u128.
    fn to_u128(&self) -> u128;
}

impl Snowflake for u128 {
    fn to_u128(&self) -> u128 {
        *self
    }
}

impl Snowflake for String {
    fn to_u128(&self) -> u128 {
        self.parse().unwrap()
    }
}

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

impl<T: Snowflake> CastSnowflakes for T {
    type U128Result = u128;
    type StringResult = String;

    fn into_u128_ids(self) -> Self::U128Result {
        self.to_u128()
    }

    fn into_string_ids(self) -> Self::StringResult {
        self.to_string()
    }
}

impl<T: CastSnowflakes> CastSnowflakes for Vec<T> {
    type U128Result = Vec<T::U128Result>;
    type StringResult = Vec<T::StringResult>;

    fn into_u128_ids(self) -> Self::U128Result {
        self.into_iter()
            .map(CastSnowflakes::into_u128_ids)
            .collect()
    }

    fn into_string_ids(self) -> Self::StringResult {
        self.into_iter()
            .map(CastSnowflakes::into_string_ids)
            .collect()
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

impl<T: CastSnowflakes> CastSnowflakes for Maybe<T> {
    type U128Result = Maybe<T::U128Result>;
    type StringResult = Maybe<T::StringResult>;

    fn into_u128_ids(self) -> Self::U128Result {
        self.map(CastSnowflakes::into_u128_ids)
    }

    fn into_string_ids(self) -> Self::StringResult {
        self.map(CastSnowflakes::into_string_ids)
    }
}

#[macro_export]
macro_rules! serde_for_bitflags {
    (u32: $t:ty) => {
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
    (i64: $t:ty) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i64(self.bits)
            }
        }

        impl<'de> serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self {
                    bits: deserializer.deserialize_i64($crate::I64Visitor)?,
                })
            }
        }
    };
}

macro_rules! visitor {
    ($name:ident, $t:ty, $m:ident, $bounds:literal) => {
        pub(crate) struct $name;

        impl serde::de::Visitor<'_> for $name {
            type Value = $t;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!("an integer between ", $bounds))
            }

            fn $m<E>(self, v: $t) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v)
            }
        }
    };
}

visitor!(U32Visitor, u32, visit_u32, "0 and 2^32 - 1");
visitor!(I64Visitor, i64, visit_i64, "-2^63 and 2^63 - 1");

/// A serde value that distinguishes between null and missing values.
#[derive(Clone, Debug, Default)]
pub enum Maybe<T> {
    /// The field is absent.
    #[default]
    Absent,
    /// The field is present but is set to `null`.
    Null,
    /// The field is present and is set to a value, `T`.
    Value(T),
}

impl<T> Maybe<T> {
    /// Returns `true` if the value is `Absent`.
    pub const fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }

    /// Maps the inner value of `Maybe` to a new value.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Maybe<U> {
        match self {
            Self::Value(v) => Maybe::Value(f(v)),
            Self::Null => Maybe::Null,
            Self::Absent => Maybe::Absent,
        }
    }
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(opt: Option<T>) -> Self {
        opt.map_or(Self::Null, Self::Value)
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(maybe: Maybe<T>) -> Self {
        match maybe {
            Maybe::Value(v) => Some(v),
            _ => None,
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for Maybe<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

impl<T: serde::Serialize> serde::Serialize for Maybe<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => v.serialize(serializer),
            Self::Absent => Err(serde::ser::Error::custom(
                "Maybe fields need to be annotated with \
                    `#[serde(default, skip_serializing_if = \"Maybe::is_Absent\")]`",
            )),
        }
    }
}

pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// A prelude of commonly used traits. This is usually used with a glob-import, i.e.
/// `use common::prelude::*;`.
pub mod prelude {
    pub use crate::{CastSnowflakes, Snowflake, Timestamp};
}

pub(crate) mod crate_prelude {
    pub use crate::{CastSnowflakes, Maybe, Snowflake};
    pub use macros::CastSnowflakes;
    pub use serde_for_bitflags;
}
