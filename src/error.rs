//! Definitions of the error types.

use thiserror::Error;

use crate::types::NbtString;

/// Errors that can occur while parsing NBT data.
#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
    /// The encountered tag ID is not valid.
    #[error("Invalid tag ID: {0}")]
    InvalidTagId(u8),
    /// The input ended before the parser finished.
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    /// The string data is not valid UTF-8.
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
    /// The encountered length of an `Array` or a `List` is negative.
    #[error("Negative length encountered: {0}")]
    NegativeLength(i32),
    /// Extra bytes remaining after the parser finished.
    #[error("Leftover data: {0} bytes")]
    LeftoverData(usize),
    /// A non-unique tag name was encountered.
    #[error("Duplicate tag name")]
    DuplicateTagName(NbtString),
    /// The data is not a valid NBT file.
    #[error("Not an NBT file")]
    NotNBT,
}

/// Represents the errors caused by trying to create an invalid NBT tag.
#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationError {
    /// Indicates that the `Array` exceeds the maximum allowed length.
    #[error("The array is to long {0}/{max}", max = i32::MAX)]
    ArrayTooLong(usize),
    /// Indicates that the `String` exceeds the maximum allowed length.
    #[error("The string is to long {0}/{max}", max = u16::MAX)]
    StringTooLong(usize),
}
