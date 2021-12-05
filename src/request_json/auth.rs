/// JSON accepted for POST /v0/auth
#[derive(Serialize, Deserialize, Clone)]
pub struct AuthJson {
    pub email: String,
    pub password: String,
}
