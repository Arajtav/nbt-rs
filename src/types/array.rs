use std::{fmt, ops::Deref};

use bytemuck::cast_slice;

use crate::{
    error::{ParseError, ValidationError},
    traits::{NbtParse, NbtSerialize},
};

/// An NBT Array.
///
/// Contains a fixed number of items, up to `i32::MAX`.
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
// impl_nbt_serialize_array!(u32);
impl_nbt_serialize_array!(i64);
// impl_nbt_serialize_array!(u64);

impl NbtParse for NbtArray<i8> {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (len, data) = i32::try_parse_nbt_payload(data)?;
        if len < 0 {
            return Err(ParseError::NegativeLength(len));
        }

        let (data, rest) = data
            .split_at_checked(len as usize)
            .ok_or(ParseError::UnexpectedEndOfInput)?;
        Ok((
            NbtArray {
                items: cast_slice(data).into(),
            },
            rest,
        ))
    }
}
impl NbtParse for NbtArray<i32> {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (len, data) = i32::try_parse_nbt_payload(data)?;
        if len < 0 {
            return Err(ParseError::NegativeLength(len));
        }

        let (data, rest) = data
            .split_at_checked(len as usize * 4)
            .ok_or(ParseError::UnexpectedEndOfInput)?;
        let items = data
            .chunks_exact(4)
            .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        Ok((NbtArray { items }, rest))
    }
}

impl NbtParse for NbtArray<i64> {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (len, data) = i32::try_parse_nbt_payload(data)?;
        if len < 0 {
            return Err(ParseError::NegativeLength(len));
        }

        let (data, rest) = data
            .split_at_checked(len as usize * 8)
            .ok_or(ParseError::UnexpectedEndOfInput)?;
        let items = data
            .chunks_exact(8)
            .map(|chunk| i64::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        Ok((NbtArray { items }, rest))
    }
}
