use thiserror::Error;

/// Represents the errors caused by trying to create an invalid NBT tag.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Indicates that the `Array` exceeds the maximum allowed length.
    #[error("The array is to long {0}/{max}", max = i32::MAX)]
    ArrayTooLong(usize),
    /// Indicates that the `String` exceeds the maximum allowed length.
    #[error("The string is to long {0}/{max}", max = u16::MAX)]
    StringTooLong(usize),
}
