use enum_as_inner::EnumAsInner;

use crate::{
    traits::NbtSerialize,
    types::{NbtArray, NbtCompound, NbtString, tag::NbtTagId},
};

/// An NBT List.
///
/// Represents all possible NBT lists.
/// Uses NBT `Array` underneath as it is the same thing really.
#[derive(Debug, EnumAsInner, PartialEq)]
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
