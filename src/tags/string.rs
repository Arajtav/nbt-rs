use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::tags::ValidationError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct String {
    pub(crate) str: std::string::String,
}

impl Hash for String {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
    }
}

impl TryFrom<std::string::String> for String {
    type Error = (ValidationError, std::string::String);
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
