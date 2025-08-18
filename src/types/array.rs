use std::ops::Deref;

use bytemuck::cast_slice;

use crate::{error::ValidationError, traits::NbtSerialize};

/// An NBT Array.
///
/// Contains a fixed number of items, up to `i32::MAX`.
#[derive(Debug, PartialEq)]
pub struct NbtArray<T> {
    pub(crate) items: Vec<T>,
}

impl<T> TryFrom<Vec<T>> for NbtArray<T> {
    type Error = (ValidationError, Vec<T>);

    /// Attempts to create an `Array` from a `Vec`.
    ///
    /// # Errors
    /// Returns an error if the `Vec` has more than `i32::MAX` elements.
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
