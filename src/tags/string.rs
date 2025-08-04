use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::tags::ValidationError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct String {
    data: std::string::String,
}

impl Hash for String {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl String {
    pub(crate) fn new_unchecked(data: std::string::String) -> Self {
        Self { data }
    }
}

impl TryFrom<std::string::String> for String {
    type Error = ValidationError;
    fn try_from(str: std::string::String) -> Result<Self, Self::Error> {
        if str.len() > u16::MAX as usize {
            Err(ValidationError::StringTooLong(str.len()))
        } else {
            Ok(Self { data: str })
        }
    }
}

impl From<String> for std::string::String {
    fn from(str: String) -> Self {
        str.data
    }
}

impl String {
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl Deref for String {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data.as_bytes()
    }
}

impl Borrow<str> for String {
    fn borrow(&self) -> &str {
        &self.data
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.data == other
    }
}

impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        self == other.data
    }
}
