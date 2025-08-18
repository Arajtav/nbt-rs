use enum_as_inner::EnumAsInner;

use crate::{
    serializer::NbtSerialize,
    tags::{Array, Compound, String, TagId},
};

/// An NBT List.
///
/// Represents all possible NBT lists.
/// Uses NBT `Array` underneath as it is the same thing really.
#[derive(Debug, EnumAsInner, PartialEq)]
pub enum List {
    /// An empty `List` with no type.
    End,
    /// A `List` of signed bytes.
    Byte(Array<i8>),
    /// A `List` of signed 2-byte integers.
    Short(Array<i16>),
    /// A `List` of signed 4-byte integers.
    Int(Array<i32>),
    /// A `List` of signed 8-byte integers.
    Long(Array<i64>),
    /// A `List` of 4-byte floats.
    Float(Array<f32>),
    /// A `List` of 8-byte floats.
    Double(Array<f64>),
    /// A `List` of NBT Byte Arrays.
    ByteArray(Array<Array<i8>>),
    /// A `List` of NBT `String`s.
    String(Array<String>),
    /// A `List` of NBT `List`s.
    List(Array<List>),
    /// A `List` of NBT Compounds.
    Compound(Array<Compound>),
    /// A `List` of NBT Int Arrays.
    IntArray(Array<Array<i32>>),
    /// A `List` of NBT Long Arrays.
    LongArray(Array<Array<i64>>),
}

impl NbtSerialize for List {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        macro_rules! serialize {
            ($data:ident, $variant:ident) => {{
                buf.push(TagId::$variant as u8);
                ($data.len() as i32).serialize_nbt_payload(buf);
                for value in $data.iter() {
                    value.serialize_nbt_payload(buf);
                }
            }};
        }

        match self {
            List::End => {
                // type and length must be 0
                buf.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]);
            }
            List::Byte(data) => serialize!(data, Byte),
            List::Short(data) => serialize!(data, Short),
            List::Int(data) => serialize!(data, Int),
            List::Long(data) => serialize!(data, Long),
            List::Float(data) => serialize!(data, Float),
            List::Double(data) => serialize!(data, Double),
            List::ByteArray(data) => serialize!(data, ByteArray),
            List::String(data) => serialize!(data, String),
            List::List(data) => serialize!(data, List),
            List::Compound(data) => serialize!(data, Compound),
            List::IntArray(data) => serialize!(data, IntArray),
            List::LongArray(data) => serialize!(data, LongArray),
        }
    }
}
