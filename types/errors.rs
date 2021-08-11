#[derive(Serialize, Deserialize)]
pub struct InternalServerErrorJson {
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct BadRequestJson {
    pub reason: String,
    pub location: Option<BadRequestJsonLocation>,
}

#[derive(Serialize, Deserialize)]
pub struct BadRequestJsonLocation {
    pub line: u32,
    pub character: u32,
}

#[derive(Serialize, Deserialize)]
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
