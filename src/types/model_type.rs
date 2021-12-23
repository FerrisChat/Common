#[repr(u8)]
/// The model type, specifically for bits 64 to 71 of snowflakes.
pub enum ModelType {
    Guild = 0,
    User = 1,
    Channel = 2,
    Message = 3,
    Role = 4,
    /// You should never get a snowflake with this model type,
    /// as these types are only used for cases where the database needs a ID but no other option applies.
    InternalUse = 5,
    DmChannel = 6,
    Bot = 7,
    Unknown = !0,
}
