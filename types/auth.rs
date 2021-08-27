#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    /// The authorization token
    pub token: String,
}
