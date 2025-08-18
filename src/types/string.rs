use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{error::ValidationError, traits::NbtSerialize};

/// An NBT String.
///
/// Wrapper around a `String`, limiting its length to `u16:MAX` bytes.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NbtString {
    pub(crate) str: String,
}

impl NbtSerialize for NbtString {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        (bytes.len() as u16).serialize_nbt_payload(buf);
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

    /// Attempts to create a `NbtString` from an `String`.
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
