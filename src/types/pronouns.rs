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

impl TryFrom<i16> for Pronouns {
    type Error = ();

    fn try_from(x: i16) -> Result<Self, Self::Error> {
        use Pronouns::*;
        match x {
            0 => Ok(HeHim),
            1 => Ok(HeIt),
            2 => Ok(HeShe),
            3 => Ok(HeThey),
            4 => Ok(ItHim),
            5 => Ok(ItIts),
            6 => Ok(ItShe),
            7 => Ok(ItThey),
            8 => Ok(SheHe),
            9 => Ok(SheHer),
            10 => Ok(SheIt),
            11 => Ok(SheThey),
            12 => Ok(TheyHe),
            13 => Ok(TheyIt),
            14 => Ok(TheyShe),
            15 => Ok(TheyThem),
            16 => Ok(Any),
            17 => Ok(OtherAsk),
            18 => Ok(Avoid),
            _ => Err(()),
        }
    }
}
