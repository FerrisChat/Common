#[derive(Serialize, Deserialize)]
#[serde(tag = "c", content = "d")]
pub enum WsEvent {
    Identify { token: String, intents: u64 },
}
