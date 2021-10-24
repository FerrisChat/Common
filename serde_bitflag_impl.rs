macro_rules! serde_for_bitflags {
    ($tt:ident) => {
        use serde::{Serialize, Serializer};
        impl Serialize for $tt {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_i64(self.bits)
            }
        }

        use serde::{Deserialize, Deserializer};
        use $crate::serde_bitflag_impl::I64Visitor;
        impl<'de> Deserialize<'de> for $tt {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Self {
                    bits: deserializer.deserialize_i64(I64Visitor)?,
                })
            }
        }
    };
}

use serde::de::Visitor;
use std::fmt::Formatter;

pub struct I64Visitor;
impl Visitor<'_> for I64Visitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^63 and 2^63")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}
