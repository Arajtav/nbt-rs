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
