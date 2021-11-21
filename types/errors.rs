use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone)]
pub struct InternalServerErrorJson {
    pub reason: String,
    pub is_bug: bool,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BadRequestJson {
    pub reason: String,
    pub location: Option<BadRequestJsonLocation>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BadRequestJsonLocation {
    pub line: u32,
    pub character: u32,
}

#[derive(Deserialize, Clone)]
/// JSON returned along with a HTTP 429 Too Many Requests
pub struct TooManyRequestsJson {
    /// This many requests are allowed in `duration` seconds.
    pub count: u32,
    /// `count` requests are allowed in this many seconds.
    pub duration: u32,
    /// You can retry this request in this many seconds.
    ///
    /// This number could be the maximum for `u128` (2 to the 128th power minus 1) for some endpoints.
    /// That will rarely happen (as it stands, only for the Create User endpoint),
    /// but if it does, signals to just ignore this request and never retry,
    /// as a lifetime limit has been hit for this IP address.
    pub retry_after: u128,
}

impl Serialize for TooManyRequestsJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut self_ser = serializer.serialize_struct("TooManyRequestsJson", 4)?;

        self_ser.serialize_field("count", &self.count)?;
        self_ser.serialize_field("duration", &self.duration)?;

        self_ser.serialize_field("retry_after", &self.retry_after)?;
        self_ser.serialize_field("retry_after_string", &self.retry_after.to_string())?;
        self_ser.end()
    }
}

#[derive(Serialize, Deserialize, Clone)]
// JSON returned along with HTTP 404 Not Found.
pub struct NotFoundJson {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Json {
    pub message: String,
}
