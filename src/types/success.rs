#[derive(Deserialize, Serialize, Clone)]
pub struct SuccessJson {
    pub reason: String,
}

impl SuccessJson {
    /// Create a new object of `Self` with the provided reason.
    ///
    /// This exists mainly to make the code easier to read.
    #[inline(always)]
    pub const fn new(reason: String) -> Self {
        Self { reason }
    }
}
