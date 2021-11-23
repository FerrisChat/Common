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

impl From<i16> for Pronouns {
    fn from(x: i16) -> Self {
        use Pronouns::*;
        match x {
            0 => HeHim,
            1 => HeIt,
            2 => HeShe,
            3 => HeThey,
            4 => ItHim,
            5 => ItIts,
            6 => ItShe,
            7 => ItThey,
            8 => SheHe,
            9 => SheHer,
            10 => SheIt,
            11 => SheThey,
            12 => TheyHe,
            13 => TheyIt,
            14 => TheyShe,
            15 => TheyThem,
            16 => Any,
            17 => OtherAsk,
            18 => Avoid,
            _ => panic!("got {}, out of range for pronoun conversion", x),
        }
    }
}
