#[repr(u8)]
/// The model type, specifically for bits 64 to 71 of snowflakes.
pub enum ModelType {
    Guild = 0,
    User = 1,
    Channel = 2,
    Member = 3,
    Unknown = !0,
}