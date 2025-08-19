use core::fmt;
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{
    error::{ParseError, ValidationError},
    traits::{NbtParse, NbtSerialize},
};

/// An NBT String.
///
/// Wrapper around a `String`, limiting its length to `u16:MAX` bytes.
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct NbtString {
    pub(crate) str: String,
}

impl fmt::Display for NbtString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl NbtSerialize for NbtString {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        (bytes.len() as u16 as i16).serialize_nbt_payload(buf);
        buf.extend_from_slice(bytes);
    }
}

impl Hash for NbtString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
    }
}

impl TryFrom<String> for NbtString {
    type Error = (ValidationError, String);

    /// Attempts to create an `NbtString` from a `String`.
    ///
    /// # Errors
    /// Returns an error if the `String` is longer than `u16::MAX`.
    fn try_from(str: String) -> Result<Self, Self::Error> {
        if str.len() > u16::MAX as usize {
            Err((ValidationError::StringTooLong(str.len()), str))
        } else {
            Ok(Self { str })
        }
    }
}

impl Deref for NbtString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.str
    }
}

impl Borrow<str> for NbtString {
    fn borrow(&self) -> &str {
        &self.str
    }
}

impl PartialEq<str> for NbtString {
    fn eq(&self, other: &str) -> bool {
        self.str == other
    }
}

impl PartialEq<NbtString> for str {
    fn eq(&self, other: &NbtString) -> bool {
        self == other.str
    }
}

impl NbtParse for NbtString {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (len, data) = i16::try_parse_nbt_payload(data)?;
        let (data, rest) = data
            .split_at_checked(len as u16 as usize)
            .ok_or(ParseError::UnexpectedEndOfInput)?;

        let data =
            std::string::String::from_utf8(data.into()).map_err(|_| ParseError::InvalidUtf8)?;
        Ok((NbtString { str: data }, rest))
    }
}
