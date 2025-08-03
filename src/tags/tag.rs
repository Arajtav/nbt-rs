use std::collections::HashMap;

use enum_as_inner::EnumAsInner;

use crate::tags::{Array, List, String};

#[derive(Debug, EnumAsInner, PartialEq)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Array<u8>),
    String(String),
    List(List),
    Compound(HashMap<String, Tag>),
    IntArray(Array<i32>),
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
