use std::ops::Deref;

use crate::tags::ValidationError;

#[derive(Debug, PartialEq)]
pub struct Array<T> {
    data: Vec<T>,
}

impl<T> Array<T> {
    pub(crate) fn new_unchecked(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> TryFrom<Vec<T>> for Array<T> {
    type Error = ValidationError;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() > i32::MAX as usize {
            Err(ValidationError::ArrayTooLong(vec.len()))
        } else {
            Ok(Self { data: vec })
        }
    }
}

impl<T> From<Array<T>> for Vec<T> {
    fn from(array: Array<T>) -> Self {
        array.data
    }
}

impl<T> Array<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
