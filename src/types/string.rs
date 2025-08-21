use core::fmt;
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{error::ValidationError, traits::NbtSerialize};

/// A wrapper around a `String` making sure it is a valid nbt string.
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

    /// Attempts to create an `NbtString`
    ///
    /// # Errors
    /// Will fail if the source string is longer than `u16::MAX` bytes.
    ///
    /// # Examples
    /// ```
    /// let nbt_string: nbt_rs::types::NbtString = "AnNbtString".to_owned().try_into().unwrap();
    /// ```
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

impl PartialEq<&str> for NbtString {
    fn eq(&self, other: &&str) -> bool {
        self.str == *other
    }
}

impl PartialEq<NbtString> for &str {
    fn eq(&self, other: &NbtString) -> bool {
        *self == other.str
    }
}
