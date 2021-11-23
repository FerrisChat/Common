/// Pronouns are sorted alphabetically.
#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub enum Pronouns {
    /// he/him
    HeHim,
    /// he/it
    HeIt,
    /// he/she
    HeShe,
    /// he/they
    HeThey,
    /// it/him
    ItHim,
    /// it/its
    ItIts,
    /// it/she
    ItShe,
    /// it/they
    ItThey,
    /// she/he
    SheHe,
    /// she/her
    SheHer,
    /// she/it
    SheIt,
    /// she/they
    SheThey,
    /// they/he
    TheyHe,
    /// they/it
    TheyIt,
    /// they/she
    TheyShe,
    /// they/them
    TheyThem,
    /// any pronouns
    Any,
    /// other pronouns/ask
    OtherAsk,
    /// avoid pronouns/use name
    Avoid,
}
