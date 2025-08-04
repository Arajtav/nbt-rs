use std::collections::HashMap;

use enum_as_inner::EnumAsInner;

use crate::tags::{Array, String, Tag};

#[derive(Debug, EnumAsInner, PartialEq)]
pub enum List {
    End,
    Byte(Array<i8>),
    Short(Array<i16>),
    Int(Array<i32>),
    Long(Array<i64>),
    Float(Array<f32>),
    Double(Array<f64>),
    ByteArray(Array<Array<u8>>),
    String(Array<String>),
    List(Array<List>),
    Compound(Array<HashMap<String, Tag>>),
    IntArray(Array<Array<i32>>),
    LongArray(Array<Array<i64>>),
}
