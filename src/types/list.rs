use core::fmt;

use enum_as_inner::EnumAsInner;

use crate::{
    error::ParseError,
    traits::{NbtParse, NbtSerialize},
    types::{NbtArray, NbtCompound, NbtString, tag::NbtTagId},
};

/// An NBT List.
///
/// Represents all possible NBT lists.
/// Uses NBT `Array` underneath as it is the same thing really.
#[derive(Debug, EnumAsInner, PartialEq, PartialOrd, Clone)]
pub enum NbtList {
    /// An empty `List` with no type.
    End,
    /// A `List` of signed bytes.
    Byte(NbtArray<i8>),
    /// A `List` of signed 2-byte integers.
    Short(NbtArray<i16>),
    /// A `List` of signed 4-byte integers.
    Int(NbtArray<i32>),
    /// A `List` of signed 8-byte integers.
    Long(NbtArray<i64>),
    /// A `List` of 4-byte floats.
    Float(NbtArray<f32>),
    /// A `List` of 8-byte floats.
    Double(NbtArray<f64>),
    /// A `List` of NBT Byte Arrays.
    ByteArray(NbtArray<NbtArray<i8>>),
    /// A `List` of NBT `String`s.
    String(NbtArray<NbtString>),
    /// A `List` of NBT `List`s.
    List(NbtArray<NbtList>),
    /// A `List` of NBT Compounds.
    Compound(NbtArray<NbtCompound>),
    /// A `List` of NBT Int Arrays.
    IntArray(NbtArray<NbtArray<i32>>),
    /// A `List` of NBT Long Arrays.
    LongArray(NbtArray<NbtArray<i64>>),
}

impl fmt::Display for NbtList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NbtList::End => write!(f, "End"),
            NbtList::Byte(arr) => write!(f, "Byte({arr})"),
            NbtList::Short(arr) => write!(f, "Short({arr})"),
            NbtList::Int(arr) => write!(f, "Int({arr})"),
            NbtList::Long(arr) => write!(f, "Long({arr})"),
            NbtList::Float(arr) => write!(f, "Float({arr})"),
            NbtList::Double(arr) => write!(f, "Double({arr})"),
            NbtList::ByteArray(arr) => write!(f, "ByteArray({arr})"),
            NbtList::String(arr) => write!(f, "String({arr})"),
            NbtList::List(arr) => write!(f, "List({arr})"),
            NbtList::Compound(arr) => write!(f, "Compound({arr})"),
            NbtList::IntArray(arr) => write!(f, "IntArray({arr})"),
            NbtList::LongArray(arr) => write!(f, "LongArray({arr})"),
        }
    }
}

impl NbtSerialize for NbtList {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        macro_rules! serialize {
            ($data:ident, $variant:ident) => {{
                buf.push(NbtTagId::$variant as u8);
                ($data.len() as i32).serialize_nbt_payload(buf);
                for value in $data.iter() {
                    value.serialize_nbt_payload(buf);
                }
            }};
        }

        match self {
            NbtList::End => {
                // type and length must be 0
                buf.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]);
            }
            NbtList::Byte(data) => {
                buf.push(NbtTagId::Byte as u8);
                data.serialize_nbt_payload(buf);
            }
            NbtList::Short(data) => serialize!(data, Short),
            NbtList::Int(data) => {
                buf.push(NbtTagId::Int as u8);
                data.serialize_nbt_payload(buf);
            }
            NbtList::Long(data) => {
                buf.push(NbtTagId::Long as u8);
                data.serialize_nbt_payload(buf);
            }
            NbtList::Float(data) => serialize!(data, Float),
            NbtList::Double(data) => serialize!(data, Double),
            NbtList::ByteArray(data) => serialize!(data, ByteArray),
            NbtList::String(data) => serialize!(data, String),
            NbtList::List(data) => serialize!(data, List),
            NbtList::Compound(data) => serialize!(data, Compound),
            NbtList::IntArray(data) => serialize!(data, IntArray),
            NbtList::LongArray(data) => serialize!(data, LongArray),
        }
    }
}

impl NbtParse for NbtList {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (tag_id, data) = NbtTagId::try_parse_nbt_payload(data)?;
        // that kinda makes the code way uglier, however i am almost sure reusing those methods improves caching
        match tag_id {
            NbtTagId::Byte => {
                return <NbtArray<i8>>::try_parse_nbt_payload(data)
                    .map(|e| (NbtList::Byte(e.0), e.1));
            }
            NbtTagId::Int => {
                return <NbtArray<i32>>::try_parse_nbt_payload(data)
                    .map(|e| (NbtList::Int(e.0), e.1));
            }
            NbtTagId::Long => {
                return <NbtArray<i64>>::try_parse_nbt_payload(data)
                    .map(|e| (NbtList::Long(e.0), e.1));
            }
            _ => {}
        }
        let (len, data) = i32::try_parse_nbt_payload(data)?;
        if len < 0 {
            return Err(ParseError::NegativeLength(len));
        }

        macro_rules! parse {
            ($type:ty, $variant:ident) => {{
                let mut data = data;
                let mut items = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    let (item, rest) = <$type>::try_parse_nbt_payload(data)?;
                    items.push(item);
                    data = rest;
                }
                (NbtList::$variant(NbtArray { items: items }), data)
            }};
        }

        Ok(match tag_id {
            NbtTagId::End => (NbtList::End, data),
            NbtTagId::Byte => unreachable!(),
            NbtTagId::Short => parse!(i16, Short),
            NbtTagId::Int => unreachable!(),
            NbtTagId::Long => unreachable!(),
            NbtTagId::Float => parse!(f32, Float),
            NbtTagId::Double => parse!(f64, Double),
            NbtTagId::ByteArray => parse!(NbtArray<i8>, ByteArray),
            NbtTagId::String => parse!(NbtString, String),
            NbtTagId::List => parse!(NbtList, List),
            NbtTagId::Compound => parse!(NbtCompound, Compound),
            NbtTagId::IntArray => parse!(NbtArray<i32>, IntArray),
            NbtTagId::LongArray => parse!(NbtArray<i64>, LongArray),
        })
    }
}
