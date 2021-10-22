#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    /// The authorization token
    pub token: String,
}
