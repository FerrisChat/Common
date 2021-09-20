#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    /// The channel ID
    ///
    /// 128 bit unsigned integer
    pub id: u128,

    /// The channel's name
    ///
    /// String of max length 100 chars
    pub name: String,

    /// The guild ID
    ///
    /// 128 bit unsigned integer
    pub guild_id: u128,
}
