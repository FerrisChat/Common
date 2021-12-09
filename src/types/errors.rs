#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ErrorJson {
    /// JSON returned with a HTTP 500
    InternalServerError {
        reason: String,
        is_bug: bool,
        link: Option<String>,
    },

    /// JSON returned with a HTTP 429
    TooManyRequests {
        /// This many requests are allowed in `duration` seconds.
        count: u32,
        /// `count` requests are allowed in this many seconds.
        duration: u32,
        /// You can retry this request in this many seconds.
        ///
        /// This number could be the maximum for `i32` (2 to the 31st power minus 1) for some endpoints.
        /// That will rarely happen (as it stands, only for the Create User endpoint),
        /// but if it does, signals to just ignore this request and never retry,
        /// as a lifetime limit has been hit for this IP address.
        retry_after: i32,
    },

    /// A catch-all for all other errors.
    Message {
        #[serde(skip)]
        code: u16,
        message: String,
    },
}

impl ErrorJson {
    /// Create a new Internal Server Error object.
    #[inline]
    #[must_use]
    pub const fn new_500(reason: String, is_bug: bool, link: Option<String>) -> Self {
        Self::InternalServerError {
            reason,
            is_bug,
            link,
        }
    }

    /// Create a new Bad Request object
    #[inline]
    #[must_use]
    pub const fn new_400(message: String) -> Self {
        Self::Message { code: 400, message }
    }

    /// Create a new Not Found object
    #[inline]
    #[must_use]
    pub const fn new_404(message: String) -> Self {
        Self::Message { code: 400, message }
    }

    /// Create a new Too Many Requests object
    #[inline]
    #[must_use]
    pub const fn new_429(count: u32, duration: u32, retry_after: i32) -> Self {
        Self::TooManyRequests {
            count,
            duration,
            retry_after,
        }
    }

    /// Create a new Forbidden object
    #[inline]
    #[must_use]
    pub const fn new_403(message: String) -> Self {
        Self::Message { code: 403, message }
    }

    /// Create a new Unauthorized object
    #[inline]
    #[must_use]
    pub const fn new_401(message: String) -> Self {
        Self::Message { code: 401, message }
    }

    /// Create a new Conflict object
    #[inline]
    #[must_use]
    pub const fn new_409(message: String) -> Self {
        Self::Message { code: 409, message }
    }

    /// Create a new object with an arbitrary HTTP code
    #[inline]
    #[must_use]
    pub const fn new(message: String, code: u16) -> Self {
        Self::Message { code, message }
    }

    /// Get the HTTP status code of this object
    #[inline]
    #[must_use]
    pub const fn get_code(&self) -> u16 {
        match self {
            ErrorJson::InternalServerError { .. } => 500,
            ErrorJson::TooManyRequests { .. } => 429,
            ErrorJson::Message { code, .. } => *code,
        }
    }
}
