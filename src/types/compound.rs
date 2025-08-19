use core::fmt;
use std::collections::HashMap;

use crate::{
    error::ParseError,
    traits::{NbtParse, NbtSerialize},
    types::{NbtArray, NbtList, NbtString, NbtTag, NbtTagId},
};

/// An NBT Compound.
///
/// Ensures no items have duplicate keys.
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct NbtCompound {
    pub(crate) data: Vec<(NbtString, NbtTag)>,
}

impl fmt::Display for NbtCompound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.data
                .iter()
                .map(|(k, v)| format!("{:?}: {}", k.to_string(), v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl NbtSerialize for NbtCompound {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        for (key, tag) in &self.data {
            buf.push(tag.tag_id() as u8);
            key.serialize_nbt_payload(buf);
            tag.serialize_nbt_payload(buf);
        }
        buf.push(0x00);
    }
}

impl From<NbtCompound> for Vec<(NbtString, NbtTag)> {
    fn from(val: NbtCompound) -> Self {
        val.data
    }
}

impl From<NbtCompound> for HashMap<NbtString, NbtTag> {
    fn from(val: NbtCompound) -> Self {
        val.data.into_iter().collect()
    }
}

impl From<HashMap<NbtString, NbtTag>> for NbtCompound {
    fn from(map: HashMap<NbtString, NbtTag>) -> Self {
        let data = map.into_iter().collect();
        NbtCompound { data }
    }
}

fn parse_payload(tag_id: NbtTagId, data: &[u8]) -> Result<(NbtTag, &[u8]), ParseError> {
    macro_rules! parse {
        ($variant:ident, $type:ty) => {
            <$type>::try_parse_nbt_payload(data).map(|(v, rest)| (NbtTag::$variant(v), rest))
        };
    }

    match tag_id {
        NbtTagId::End => Ok((NbtTag::End, data)),
        NbtTagId::Byte => parse!(Byte, i8),
        NbtTagId::Short => parse!(Short, i16),
        NbtTagId::Int => parse!(Int, i32),
        NbtTagId::Long => parse!(Long, i64),
        NbtTagId::Float => parse!(Float, f32),
        NbtTagId::Double => parse!(Double, f64),
        NbtTagId::ByteArray => parse!(ByteArray, NbtArray<i8>),
        NbtTagId::String => parse!(String, NbtString),
        NbtTagId::List => parse!(List, NbtList),
        NbtTagId::Compound => parse!(Compound, NbtCompound),
        NbtTagId::IntArray => parse!(IntArray, NbtArray<i32>),
        NbtTagId::LongArray => parse!(LongArray, NbtArray<i64>),
    }
}

impl NbtParse for NbtCompound {
    fn try_parse_nbt_payload(mut data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut compound: Vec<(NbtString, NbtTag)> = Vec::new();

        loop {
            let (tag_id, rest) = NbtTagId::try_parse_nbt_payload(data)?;
            if tag_id == NbtTagId::End {
                return Ok((NbtCompound { data: compound }, rest));
            }

            let (name, rest) = NbtString::try_parse_nbt_payload(rest)?;
            let (tag, rest) = parse_payload(tag_id, rest)?;

            if compound
                .iter()
                .any(|(existing_name, _)| existing_name.eq(&name))
            {
                return Err(ParseError::DuplicateTagName(name));
            }
            compound.push((name, tag));
            data = rest;
        }
    }
}
