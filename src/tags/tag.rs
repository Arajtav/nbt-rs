use enum_as_inner::EnumAsInner;

use crate::tags::{Array, Compound, List, String};

/// An NBT Tag.
///
/// Represents all defined NBT tags.
#[derive(Debug, EnumAsInner, PartialEq)]
pub enum Tag {
    /// An empty tag.
    End,
    /// A signed byte.
    Byte(i8),
    /// A signed 2-byte integer.
    Short(i16),
    /// A signed 4-byte integer.
    Int(i32),
    /// A signed 8-byte integer.
    Long(i64),
    /// A 4-byte float.
    Float(f32),
    /// A 8-byte float.
    Double(f64),
    /// An NBT `Array` of signed bytes.
    ByteArray(Array<i8>),
    /// An NBT `String`.
    String(String),
    /// An NBT `List`.
    List(List),
    /// A `HashMap` of NBT `Tag`s, with NBT `String` as a key.
    Compound(Compound),
    /// An NBT `Array` of signed 4-byte integer.
    IntArray(Array<i32>),
    /// An NBT `Array` of signed 8-byte integer.
    LongArray(Array<i64>),
}

impl Tag {
    pub(crate) fn tag_id(&self) -> TagId {
        match self {
            Tag::End => TagId::End,
            Tag::Byte(_) => TagId::Byte,
            Tag::Short(_) => TagId::Short,
            Tag::Int(_) => TagId::Int,
            Tag::Long(_) => TagId::Long,
            Tag::Float(_) => TagId::Float,
            Tag::Double(_) => TagId::Double,
            Tag::ByteArray(_) => TagId::ByteArray,
            Tag::String(_) => TagId::String,
            Tag::List(_) => TagId::List,
            Tag::Compound(_) => TagId::Compound,
            Tag::IntArray(_) => TagId::IntArray,
            Tag::LongArray(_) => TagId::LongArray,
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Tag::End => 0,
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) | Tag::Float(_) => 4,
            Tag::Long(_) | Tag::Double(_) => 8,
            Tag::ByteArray(v) => 4 + v.len(),
            Tag::String(v) => 2 + v.len(),
            Tag::List(v) => v.size(),
            Tag::Compound(v) => v.size,
            Tag::IntArray(v) => 4 + v.len() * 4,
            Tag::LongArray(v) => 4 + v.len() * 4,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub(crate) enum TagId {
    End = 0,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl TryFrom<u8> for TagId {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::End),
            1 => Ok(Self::Byte),
            2 => Ok(Self::Short),
            3 => Ok(Self::Int),
            4 => Ok(Self::Long),
            5 => Ok(Self::Float),
            6 => Ok(Self::Double),
            7 => Ok(Self::ByteArray),
            8 => Ok(Self::String),
            9 => Ok(Self::List),
            10 => Ok(Self::Compound),
            11 => Ok(Self::IntArray),
            12 => Ok(Self::LongArray),
            _ => Err(()),
        }
    }
}
