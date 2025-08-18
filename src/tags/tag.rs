use enum_as_inner::EnumAsInner;

use crate::{
    serializer::NbtSerialize,
    tags::{Array, Compound, List, String},
};

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

impl NbtSerialize for Tag {
    #[inline(always)]
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        match self {
            Tag::End => {}
            Tag::Byte(data) => data.serialize_nbt_payload(buf),
            Tag::Short(data) => data.serialize_nbt_payload(buf),
            Tag::Int(data) => data.serialize_nbt_payload(buf),
            Tag::Long(data) => data.serialize_nbt_payload(buf),
            Tag::Float(data) => data.serialize_nbt_payload(buf),
            Tag::Double(data) => data.serialize_nbt_payload(buf),
            Tag::ByteArray(data) => data.serialize_nbt_payload(buf),
            Tag::String(data) => data.serialize_nbt_payload(buf),
            Tag::List(data) => data.serialize_nbt_payload(buf),
            Tag::Compound(data) => data.serialize_nbt_payload(buf),
            Tag::IntArray(data) => data.serialize_nbt_payload(buf),
            Tag::LongArray(data) => data.serialize_nbt_payload(buf),
        }
    }
}
