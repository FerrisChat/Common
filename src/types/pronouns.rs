/// Pronouns are sorted alphabetically.
#[derive(Serialize, Deserialize, Clone, Copy)]
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

impl Pronouns {
    #[must_use]
    pub const fn from_i16(x: i16) -> Option<Self> {
        #[allow(clippy::enum_glob_use)]
        use Pronouns::*;
        match x {
            0 => Some(HeHim),
            1 => Some(HeIt),
            2 => Some(HeShe),
            3 => Some(HeThey),
            4 => Some(ItHim),
            5 => Some(ItIts),
            6 => Some(ItShe),
            7 => Some(ItThey),
            8 => Some(SheHe),
            9 => Some(SheHer),
            10 => Some(SheIt),
            11 => Some(SheThey),
            12 => Some(TheyHe),
            13 => Some(TheyIt),
            14 => Some(TheyShe),
            15 => Some(TheyThem),
            16 => Some(Any),
            17 => Some(OtherAsk),
            18 => Some(Avoid),
            _ => None,
        }
    }
}
