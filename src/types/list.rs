use core::fmt;

use enum_as_inner::EnumAsInner;

use crate::{
    traits::NbtSerialize,
    types::{NbtArray, NbtCompound, NbtString, tag::NbtTagId},
};

/// Represents all possible NBT lists.
/// Uses `NbtArray` underneath as it is the same thing really.
#[derive(Debug, EnumAsInner, PartialEq, PartialOrd, Clone)]
pub enum NbtList {
    /// An empty list with no type.
    End,
    /// A list of signed bytes.
    Byte(NbtArray<i8>),
    /// A list of signed 2-byte integers.
    Short(NbtArray<i16>),
    /// A list of signed 4-byte integers.
    Int(NbtArray<i32>),
    /// A list of signed 8-byte integers.
    Long(NbtArray<i64>),
    /// A list of 4-byte floats.
    Float(NbtArray<f32>),
    /// A list of 8-byte floats.
    Double(NbtArray<f64>),
    /// A list of nbt Byte Arrays.
    ByteArray(NbtArray<NbtArray<i8>>),
    /// A list of `NbtString`s.
    String(NbtArray<NbtString>),
    /// A list of `NbtList`s.
    List(NbtArray<NbtList>),
    /// A list of `NbtCompound`s.
    Compound(NbtArray<NbtCompound>),
    /// A list of nbt Int Arrays.
    IntArray(NbtArray<NbtArray<i32>>),
    /// A list of nbt Long Arrays.
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
            NbtList::Byte(data) => serialize!(data, Byte),
            NbtList::Short(data) => serialize!(data, Short),
            NbtList::Int(data) => serialize!(data, Int),
            NbtList::Long(data) => serialize!(data, Long),
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
