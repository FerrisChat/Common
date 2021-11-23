/// Pronouns are sorted alphabetically.
#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
#[repr(u8)]
pub enum Pronouns {
    /// he/him
    HeHim = 0,
    /// he/it
    HeIt = 1,
    /// he/she
    HeShe = 2,
    /// he/they
    HeThey = 3,
    /// it/him
    ItHim = 4,
    /// it/its
    ItIts = 5,
    /// it/she
    ItShe = 6,
    /// it/they
    ItThey = 7,
    /// she/he
    SheHe = 8,
    /// she/her
    SheHer = 9,
    /// she/it
    SheIt = 10,
    /// she/they
    SheThey = 11,
    /// they/he
    TheyHe = 12,
    /// they/it
    TheyIt = 13,
    /// they/she
    TheyShe = 14,
    /// they/them
    TheyThem = 15,
    /// any pronouns
    Any = 16,
    /// other pronouns/ask
    OtherAsk = 17,
    /// avoid pronouns/use name
    Avoid = 18,
}
