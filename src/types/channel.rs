#[derive(Serialize, Deserialize)]
pub struct Channel {
    /// The channel ID
    ///
    /// 64 bit signed integer
    pub id: i64,

    /// The user's name
    ///
    /// String of max length 100 chars
    pub name: String,
}
