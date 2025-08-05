//! Functions for serializing NBT data into bytes.

use std::collections::HashMap;

use crate::tags::{Array, List, String, Tag, TagId};

fn serialize_byte(data: &i8, buffer: &mut Vec<u8>) {
    buffer.push(*data as u8);
}

fn serialize_short(data: &i16, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&data.to_be_bytes());
}

fn serialize_int(data: &i32, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&data.to_be_bytes());
}

fn serialize_long(data: &i64, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&data.to_be_bytes());
}

fn serialize_float(data: &f32, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&data.to_be_bytes());
}

fn serialize_double(data: &f64, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&data.to_be_bytes());
}

fn serialize_byte_array(data: &Array<u8>, buffer: &mut Vec<u8>) {
    serialize_int(&(data.len() as i32), buffer);
    buffer.extend_from_slice(data);
}

fn serialize_string(data: &String, buffer: &mut Vec<u8>) {
    serialize_short(&(data.len() as u16 as i16), buffer);
    buffer.extend_from_slice(data.as_bytes());
}

fn serialize_int_array(data: &Array<i32>, buffer: &mut Vec<u8>) {
    serialize_int(&(data.len() as i32), buffer);
    for v in data.iter() {
        buffer.extend_from_slice(&v.to_be_bytes());
    }
}

fn serialize_long_array(data: &Array<i64>, buffer: &mut Vec<u8>) {
    serialize_int(&(data.len() as i32), buffer);
    for v in data.iter() {
        buffer.extend_from_slice(&v.to_be_bytes());
    }
}

fn serialize_list(list: &List, buffer: &mut Vec<u8>) {
    macro_rules! serialize {
        ($data:ident, $serializer:ident, $variant:ident) => {{
            buffer.push(TagId::$variant as u8);
            serialize_int(&($data.len() as i32), buffer);
            for value in $data.iter() {
                $serializer(value, buffer);
            }
        }};
    }

    match list {
        List::End => {
            buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00]);
        }
        List::Byte(data) => serialize!(data, serialize_byte, Byte),
        List::Short(data) => serialize!(data, serialize_short, Short),
        List::Int(data) => serialize!(data, serialize_int, Int),
        List::Long(data) => serialize!(data, serialize_long, Long),
        List::Float(data) => serialize!(data, serialize_float, Float),
        List::Double(data) => serialize!(data, serialize_double, Double),
        List::ByteArray(data) => serialize!(data, serialize_byte_array, ByteArray),
        List::String(data) => serialize!(data, serialize_string, String),
        List::List(data) => serialize!(data, serialize_list, List),
        List::Compound(data) => serialize!(data, serialize_compound, Compound),
        List::IntArray(data) => serialize!(data, serialize_int_array, IntArray),
        List::LongArray(data) => serialize!(data, serialize_long_array, LongArray),
    }
}

fn serialize_compound(data: &HashMap<String, Tag>, buffer: &mut Vec<u8>) {
    for (key, tag) in data {
        serialize_named_tag(key, tag, buffer);
    }
    buffer.push(0x00);
}

fn serialize_payload(data: &Tag, buffer: &mut Vec<u8>) {
    match data {
        Tag::End => {}
        Tag::Byte(data) => serialize_byte(data, buffer),
        Tag::Short(data) => serialize_short(data, buffer),
        Tag::Int(data) => serialize_int(data, buffer),
        Tag::Long(data) => serialize_long(data, buffer),
        Tag::Float(data) => serialize_float(data, buffer),
        Tag::Double(data) => serialize_double(data, buffer),
        Tag::ByteArray(data) => serialize_byte_array(data, buffer),
        Tag::String(data) => serialize_string(data, buffer),
        Tag::List(data) => serialize_list(data, buffer),
        Tag::Compound(data) => serialize_compound(data, buffer),
        Tag::IntArray(data) => serialize_int_array(data, buffer),
        Tag::LongArray(data) => serialize_long_array(data, buffer),
    }
}

fn serialize_named_tag(name: &String, tag: &Tag, buffer: &mut Vec<u8>) {
    buffer.push(tag.tag_id() as u8);
    serialize_string(name, buffer);
    serialize_payload(tag, buffer);
}

/// Serializes a named NBT Compound into a buffer of bytes.
/// This function is guaranteed not to fail, since all `Tag`s are validated at creation.
///
/// # Parameters
/// - `name`: The name of the root tag. Typically an empty string.
/// - `data`: A `HashMap` representing the contents of the compound tag.
pub fn serialize_nbt(name: &String, data: &HashMap<String, Tag>) -> Box<[u8]> {
    // would have just used serialize_named_tag, however i would need to clone for Tag::Compound()
    let mut buffer = Vec::new();
    buffer.push(TagId::Compound as u8);
    serialize_string(name, &mut buffer);
    serialize_compound(data, &mut buffer);
    buffer.into_boxed_slice()
}
