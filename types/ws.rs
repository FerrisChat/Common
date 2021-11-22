/// JSON returned by GET /v0/ws/info.
#[derive(Serialize, Deserialize, Clone)]
pub struct WsConnectionInfo {
    /// Where to establish a WebSocket connection to.
    pub url: String,
}
