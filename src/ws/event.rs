#[derive(Serialize, Deserialize)]
#[serde(tag = "c", content = "d")]
pub enum WsEvent {
    Identify {
        token: String,
        intents: u64,
    },
    Resume {
        token: String,
        session_id: String,
        sequence: u64,
    },
}
