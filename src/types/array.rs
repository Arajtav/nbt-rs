use std::{fmt, ops::Deref};

use bytemuck::cast_slice;

use crate::{error::ValidationError, traits::NbtSerialize};

/// A wrapper around a `Vec<T>`, limiting its length to the maximum allowed in nbt.
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct NbtArray<T> {
    pub(crate) items: Vec<T>,
}

impl<T: Eq> Eq for NbtArray<T> {}

impl<T: PartialOrd> PartialOrd for NbtArray<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.items.partial_cmp(&other.items)
    }
}

impl<T: Ord> Ord for NbtArray<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.items.cmp(&other.items)
    }
}

impl<T: fmt::Display> fmt::Display for NbtArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.items
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<T> TryFrom<Vec<T>> for NbtArray<T> {
    type Error = (ValidationError, Vec<T>);

    /// Attempts to create an `NbtArray`.
    ///
    /// # Errors
    /// Will fail if the source vec has more than `i32::MAX` items.
    ///
    /// # Examples
    /// ```
    /// let nbt_array: nbt_rs::types::NbtArray<i8> = vec![0i8, 1i8, 2i8].try_into().unwrap();
    /// ```
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() > i32::MAX as usize {
            Err((ValidationError::ArrayTooLong(vec.len()), vec))
        } else {
            Ok(Self { items: vec })
        }
    }
}

impl<T> From<NbtArray<T>> for Vec<T> {
    fn from(array: NbtArray<T>) -> Self {
        array.items
    }
}

impl<T> Deref for NbtArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl NbtSerialize for NbtArray<i8> {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        (self.len() as i32).serialize_nbt_payload(buf);
        buf.extend_from_slice(cast_slice(&self.items));
    }
}

macro_rules! impl_nbt_serialize_array {
    ($ty:ty) => {
        impl NbtSerialize for NbtArray<$ty> {
            fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
                (self.len() as i32).serialize_nbt_payload(buf);
                for v in self.iter() {
                    buf.extend_from_slice(&v.to_be_bytes());
                }
            }
        }
    };
}

impl_nbt_serialize_array!(i32);
impl_nbt_serialize_array!(u32);
impl_nbt_serialize_array!(i64);
impl_nbt_serialize_array!(u64);
