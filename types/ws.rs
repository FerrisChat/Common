/// JSON returned by GET /api/v0/ws/info.
#[derive(Serialize, Deserialize)]
pub struct WsConnectionInfo {
    /// Where to establish a WebSocket connection to.
    pub url: String,
}
