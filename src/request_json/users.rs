use crate::types::Pronouns;

/// JSON accepted for POST `/v0/users/` (create user)
#[derive(Serialize, Deserialize, Clone)]
pub struct UserCreateJson {
    /// User's preferred username.
    pub username: String,

    /// User's email.
    ///
    /// Note: no serverside validation is done on this string.
    /// This allows for anonymous signup, but means account recovery is
    /// impossible if a invalid email is given
    pub email: String,

    /// User's password.
    ///
    /// Password must be at least one Unicode codepoint.
    /// No restrictions are applied besides that.
    pub password: String,

    /// User's preferred set of pronouns.
    pub pronouns: Option<Pronouns>,
}

/// JSON accepted for PATCH `/v0/users/{user_id}`
#[derive(Serialize, Deserialize, Clone)]
pub struct UserUpdateJson {
    /// User's preferred username.
    pub username: Option<String>,

    /// User's email.
    ///
    /// Note: no serverside validation is done on this string.
    /// This allows for anonymous signup, but means account recovery is
    /// impossible if a invalid email is given
    pub email: Option<String>,

    /// User's avatar, base64 encoded.
    pub avatar: Option<String>,

    /// User's Password.
    ///
    /// Password must be at least one Unicode codepoint.
    /// No restrictions are applied besides that.
    pub password: Option<String>,

    /// User's preferred set of pronouns.
    pub pronouns: Option<Pronouns>,
}
