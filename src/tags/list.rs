use enum_as_inner::EnumAsInner;

use crate::tags::{Array, Compound, String};

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

impl List {
    pub(crate) fn size(&self) -> usize {
        5 + match self {
            List::End => 0,
            List::Byte(v) => v.len(),
            List::Short(v) => v.len() * 2,
            List::Int(v) => v.len() * 4,
            List::Long(v) => v.len() * 8,
            List::Float(v) => v.len() * 4,
            List::Double(v) => v.len() * 8,
            List::ByteArray(v) => v.iter().map(|ar| ar.len()).sum::<usize>() + 4 * v.len(),
            List::String(v) => v.iter().map(|str| str.len()).sum::<usize>() + 2 * v.len(),
            List::List(v) => v.iter().map(|list| list.size()).sum(),
            List::Compound(v) => v.iter().map(|compound| compound.size).sum(),
            List::IntArray(v) => (v.iter().map(|ar| ar.len()).sum::<usize>() + v.len()) * 4,
            List::LongArray(v) => 8 * v.iter().map(|ar| ar.len()).sum::<usize>() + 4 * v.len(),
        }
    }
}
