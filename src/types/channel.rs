#[derive(Serialize, Deserialize)]
pub struct Channel {
    /// The channel ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The user's name
    ///
    /// String of max length 100 chars
    pub name: String,
}
