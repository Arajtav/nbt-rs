use std::ops::Deref;

use crate::tags::ValidationError;

/// An NBT Array.
///
/// Contains a fixed number of items, up to `i32::MAX`.
#[derive(Debug, PartialEq)]
pub struct Array<T> {
    pub(crate) items: Box<[T]>,
}

impl<T> TryFrom<Vec<T>> for Array<T> {
    type Error = (ValidationError, Vec<T>);

    /// Attempts to create an `Array` from a `Vec`.
    ///
    /// # Errors
    /// Returns an error if the `Vec` has more than `i32::MAX` elements.
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() > i32::MAX as usize {
            Err((ValidationError::ArrayTooLong(vec.len()), vec))
        } else {
            Ok(Self {
                items: vec.into_boxed_slice(),
            })
        }
    }
}

impl<T> From<Array<T>> for Vec<T> {
    fn from(array: Array<T>) -> Self {
        array.items.into_vec()
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
