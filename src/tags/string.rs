use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{serializer::NbtSerialize, tags::ValidationError};

/// An NBT String.
///
/// Wrapper around a `String`, limiting its length to `u16:MAX` bytes.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct String {
    pub(crate) str: std::string::String,
}

impl NbtSerialize for String {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        (bytes.len() as u16).serialize_nbt_payload(buf);
        buf.extend_from_slice(bytes);
    }
}

impl Hash for String {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
    }
}

impl TryFrom<std::string::String> for String {
    type Error = (ValidationError, std::string::String);

    /// Attempts to create a `String` from an `std::string::String`.
    ///
    /// # Errors
    /// Returns an error if the `std::string::String` is longer than `u16::MAX`.
    fn try_from(str: std::string::String) -> Result<Self, Self::Error> {
        if str.len() > u16::MAX as usize {
            Err((ValidationError::StringTooLong(str.len()), str))
        } else {
            Ok(Self { str })
        }
    }
}

impl Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.str
    }
}

impl Borrow<str> for String {
    fn borrow(&self) -> &str {
        &self.str
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.str == other
    }
}

impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        self == other.str
    }
}
