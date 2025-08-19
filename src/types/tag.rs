use core::fmt;

use enum_as_inner::EnumAsInner;

use crate::{
    error::ParseError,
    traits::{NbtParse, NbtSerialize},
    types::{NbtArray, NbtCompound, NbtList, NbtString},
};

/// An NBT Tag.
///
/// Represents all defined NBT tags.
#[derive(Debug, EnumAsInner, PartialEq, PartialOrd, Clone)]
pub enum NbtTag {
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
    ByteArray(NbtArray<i8>),
    /// An NBT `String`.
    String(NbtString),
    /// An NBT `List`.
    List(NbtList),
    /// A `HashMap` of NBT `Tag`s, with NBT `String` as a key.
    Compound(NbtCompound),
    /// An NBT `Array` of signed 4-byte integer.
    IntArray(NbtArray<i32>),
    /// An NBT `Array` of signed 8-byte integer.
    LongArray(NbtArray<i64>),
}

impl fmt::Display for NbtTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NbtTag::End => write!(f, "END"),
            NbtTag::Byte(v) => write!(f, "{v}b"),
            NbtTag::Short(v) => write!(f, "{v}s"),
            NbtTag::Int(v) => write!(f, "{v}"),
            NbtTag::Long(v) => write!(f, "{v}l"),
            NbtTag::Float(v) => write!(f, "{v}f"),
            NbtTag::Double(v) => write!(f, "{v}d"),
            NbtTag::ByteArray(v) => write!(f, "[B; {v}]"),
            NbtTag::String(v) => write!(f, "{:?}", v.to_string()),
            NbtTag::List(v) => write!(f, "{v}"),
            NbtTag::Compound(v) => write!(f, "{v}"),
            NbtTag::IntArray(v) => write!(f, "[I; {v}]"),
            NbtTag::LongArray(v) => write!(f, "[L; {v}]"),
        }
    }
}

impl NbtTag {
    pub(crate) fn tag_id(&self) -> NbtTagId {
        match self {
            NbtTag::End => NbtTagId::End,
            NbtTag::Byte(_) => NbtTagId::Byte,
            NbtTag::Short(_) => NbtTagId::Short,
            NbtTag::Int(_) => NbtTagId::Int,
            NbtTag::Long(_) => NbtTagId::Long,
            NbtTag::Float(_) => NbtTagId::Float,
            NbtTag::Double(_) => NbtTagId::Double,
            NbtTag::ByteArray(_) => NbtTagId::ByteArray,
            NbtTag::String(_) => NbtTagId::String,
            NbtTag::List(_) => NbtTagId::List,
            NbtTag::Compound(_) => NbtTagId::Compound,
            NbtTag::IntArray(_) => NbtTagId::IntArray,
            NbtTag::LongArray(_) => NbtTagId::LongArray,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub(crate) enum NbtTagId {
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

impl TryFrom<u8> for NbtTagId {
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

impl NbtSerialize for NbtTag {
    #[inline(always)]
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        match self {
            NbtTag::End => {}
            NbtTag::Byte(data) => data.serialize_nbt_payload(buf),
            NbtTag::Short(data) => data.serialize_nbt_payload(buf),
            NbtTag::Int(data) => data.serialize_nbt_payload(buf),
            NbtTag::Long(data) => data.serialize_nbt_payload(buf),
            NbtTag::Float(data) => data.serialize_nbt_payload(buf),
            NbtTag::Double(data) => data.serialize_nbt_payload(buf),
            NbtTag::ByteArray(data) => data.serialize_nbt_payload(buf),
            NbtTag::String(data) => data.serialize_nbt_payload(buf),
            NbtTag::List(data) => data.serialize_nbt_payload(buf),
            NbtTag::Compound(data) => data.serialize_nbt_payload(buf),
            NbtTag::IntArray(data) => data.serialize_nbt_payload(buf),
            NbtTag::LongArray(data) => data.serialize_nbt_payload(buf),
        }
    }
}

impl NbtParse for NbtTagId {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (&tag_id, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
        let tag_id = NbtTagId::try_from(tag_id).map_err(|_| ParseError::InvalidTagId(tag_id))?;
        Ok((tag_id, rest))
    }
}
