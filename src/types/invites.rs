#[derive(Serialize, Deserialize)]
pub struct Invite {
    /// The invite's unique code (string)
    pub code: String,

    /// The invite owner's user ID
    ///
    /// 128 bit unsigned integer
    pub owner_id: u128,
    
    /// The invite's guild ID
    ///
    /// 128 bit unsigned integer
    pub guild_id: u128,

    /// When the invite was created as a timezone-naive 
    pub created_at: time::PrimitiveDateTime,

    /// How many times the invite was used
    pub uses: i32,
      
    /// The maximum amount of times the invite can be used before being invalidated
    pub max_uses: Option<i16>,
    
    /// The amount of seconds from the time the invite was created until it will expire
    pub max_age: Option<i64>,    
}
