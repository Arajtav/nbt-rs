//! Functions for parsing NBT data.

use std::collections::HashMap;

use thiserror::Error;

use crate::tags::{Array, List, String, Tag, TagId};

/// Errors that can occur while parsing NBT data.
#[derive(Debug, Error)]
pub enum ParseError {
    /// The encountered tag ID is not valid.
    #[error("Invalid tag ID: {0}")]
    InvalidTagId(u8),
    /// The input ended before the parser finished.
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    /// The string data is not valid UTF-8.
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
    /// The encountered length of an `Array` or a `List` is negative.
    #[error("Negative length encountered: {0}")]
    NegativeLength(i32),
    /// Extra bytes remaining after the parser finished.
    #[error("Leftover data: {0} bytes")]
    LeftoverData(usize),
    /// A non-unique tag name was encountered.
    #[error("Duplicate tag name")]
    DuplicateTagName(String),
    /// The data is not a valid NBT file.
    #[error("Not an NBT file")]
    NotNBT,
}

/// A shorthand for `Result<T, ParseError>`.
pub type Result<T> = std::result::Result<T, ParseError>;

fn parse_tag_id(data: &[u8]) -> Result<(TagId, &[u8])> {
    let (&tag_id, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
    let tag_id = TagId::try_from(tag_id).map_err(|_| ParseError::InvalidTagId(tag_id))?;
    Ok((tag_id, rest))
}

fn parse_byte(data: &[u8]) -> Result<(i8, &[u8])> {
    let (&v, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok((v as i8, rest))
}

fn parse_short(data: &[u8]) -> Result<(i16, &[u8])> {
    let (value, rest) = data
        .split_at_checked(2)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let value = i16::from_be_bytes(value.try_into().unwrap());
    Ok((value, rest))
}

fn parse_int(data: &[u8]) -> Result<(i32, &[u8])> {
    let (value, rest) = data
        .split_at_checked(4)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let value = i32::from_be_bytes(value.try_into().unwrap());
    Ok((value, rest))
}

fn parse_long(data: &[u8]) -> Result<(i64, &[u8])> {
    let (value, rest) = data
        .split_at_checked(8)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let value = i64::from_be_bytes(value.try_into().unwrap());
    Ok((value, rest))
}

fn parse_float(data: &[u8]) -> Result<(f32, &[u8])> {
    let (value, rest) = data
        .split_at_checked(4)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let value = f32::from_be_bytes(value.try_into().unwrap());
    Ok((value, rest))
}

fn parse_double(data: &[u8]) -> Result<(f64, &[u8])> {
    let (value, data) = data
        .split_at_checked(8)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let value = f64::from_be_bytes(value.try_into().unwrap());
    Ok((value, data))
}

fn parse_byte_array(data: &[u8]) -> Result<(Array<u8>, &[u8])> {
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let (data, rest) = data
        .split_at_checked(len as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok((Array { items: data.into() }, rest))
}

fn parse_string(data: &[u8]) -> Result<(String, &[u8])> {
    let (len, data) = parse_short(data)?;
    let (data, rest) = data
        .split_at_checked(len as u16 as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;

    let data = std::string::String::from_utf8(data.into()).map_err(|_| ParseError::InvalidUtf8)?;
    Ok((String { str: data }, rest))
}

fn parse_int_array(data: &[u8]) -> Result<(Array<i32>, &[u8])> {
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let (data, rest) = data
        .split_at_checked(len as usize * 4)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let data = data
        .chunks_exact(4)
        .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok((Array { items: data }, rest))
}

fn parse_long_array(data: &[u8]) -> Result<(Array<i64>, &[u8])> {
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let (data, rest) = data
        .split_at_checked(len as usize * 8)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let data = data
        .chunks_exact(8)
        .map(|chunk| i64::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok((Array { items: data }, rest))
}

fn parse_list(data: &[u8]) -> Result<(List, &[u8])> {
    let (tag_id, data) = parse_tag_id(data)?;
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    macro_rules! parse {
        ($parser:ident, $variant:ident) => {{
            let mut data = data;
            let mut items = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let (item, rest) = $parser(data)?;
                items.push(item);
                data = rest;
            }
            (
                List::$variant(Array {
                    items: items.into(),
                }),
                data,
            )
        }};
    }

    Ok(match tag_id {
        TagId::End => (List::End, data),
        TagId::Byte => parse!(parse_byte, Byte),
        TagId::Short => parse!(parse_short, Short),
        TagId::Int => parse!(parse_int, Int),
        TagId::Long => parse!(parse_long, Long),
        TagId::Float => parse!(parse_float, Float),
        TagId::Double => parse!(parse_double, Double),
        TagId::ByteArray => parse!(parse_byte_array, ByteArray),
        TagId::String => parse!(parse_string, String),
        TagId::List => parse!(parse_list, List),
        TagId::Compound => parse!(parse_compound, Compound),
        TagId::IntArray => parse!(parse_int_array, IntArray),
        TagId::LongArray => parse!(parse_long_array, LongArray),
    })
}

fn parse_payload(tag_id: TagId, data: &[u8]) -> Result<(Tag, &[u8])> {
    macro_rules! parse {
        ($variant:ident, $parser:expr) => {
            $parser(data).map(|(v, rest)| (Tag::$variant(v), rest))
        };
    }

    match tag_id {
        TagId::End => Ok((Tag::End, data)),
        TagId::Byte => parse!(Byte, parse_byte),
        TagId::Short => parse!(Short, parse_short),
        TagId::Int => parse!(Int, parse_int),
        TagId::Long => parse!(Long, parse_long),
        TagId::Float => parse!(Float, parse_float),
        TagId::Double => parse!(Double, parse_double),
        TagId::ByteArray => parse!(ByteArray, parse_byte_array),
        TagId::String => parse!(String, parse_string),
        TagId::List => parse!(List, parse_list),
        TagId::Compound => parse!(Compound, parse_compound),
        TagId::IntArray => parse!(IntArray, parse_int_array),
        TagId::LongArray => parse!(LongArray, parse_long_array),
    }
}

fn parse_compound(mut data: &[u8]) -> Result<(HashMap<String, Tag>, &[u8])> {
    let mut compound = HashMap::new();

    loop {
        let (tag_id, rest) = parse_tag_id(data)?;
        if tag_id == TagId::End {
            return Ok((compound, rest));
        }

        let (name, rest) = parse_string(rest)?;
        let (tag, rest) = parse_payload(tag_id, rest)?;

        if compound.insert(name.clone(), tag).is_some() {
            return Err(ParseError::DuplicateTagName(name));
        }
        data = rest;
    }
}

/// Parses a named NBT compound from a byte slice.
///
/// Expects the input to be a named NBT Compound, with no leftover data.
pub fn parse_nbt(data: &[u8]) -> Result<(String, HashMap<String, Tag>)> {
    let (tag_id, data) = parse_tag_id(data)?;
    if tag_id != TagId::Compound {
        return Err(ParseError::NotNBT);
    }

    let (name, data) = parse_string(data)?;
    let (tag, data) = parse_compound(data)?;

    if !data.is_empty() {
        Err(ParseError::LeftoverData(data.len()))
    } else {
        Ok((name, tag))
    }
}
