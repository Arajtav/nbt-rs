//! Functions for parsing NBT data.

use bytemuck::cast_slice;

use crate::{
    error::ParseError,
    types::{NbtArray, NbtCompound, NbtList, NbtString, NbtTag, NbtTagId},
};

/// A shorthand for `Result<T, ParseError>`.
pub type Result<T> = std::result::Result<T, ParseError>;

fn parse_tag_id(data: &mut &[u8]) -> Result<NbtTagId> {
    let &tag_id = data
        .split_off_first()
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    NbtTagId::try_from(tag_id)
}

fn parse_byte(data: &mut &[u8]) -> Result<i8> {
    let &v = data
        .split_off_first()
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok(v as i8)
}

fn parse_short(data: &mut &[u8]) -> Result<i16> {
    let bytes = data
        .split_off(..2)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok(i16::from_be_bytes(bytes.try_into().unwrap()))
}

fn parse_int(data: &mut &[u8]) -> Result<i32> {
    let bytes = data
        .split_off(..4)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok(i32::from_be_bytes(bytes.try_into().unwrap()))
}

fn parse_long(data: &mut &[u8]) -> Result<i64> {
    let bytes = data
        .split_off(..8)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok(i64::from_be_bytes(bytes.try_into().unwrap()))
}

fn parse_float(data: &mut &[u8]) -> Result<f32> {
    let bytes = data
        .split_off(..4)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok(f32::from_be_bytes(bytes.try_into().unwrap()))
}

fn parse_double(data: &mut &[u8]) -> Result<f64> {
    let bytes = data
        .split_off(..8)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok(f64::from_be_bytes(bytes.try_into().unwrap()))
}

fn parse_byte_array(data: &mut &[u8]) -> Result<NbtArray<i8>> {
    let len = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let items = data
        .split_off(..len as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok(NbtArray {
        items: cast_slice(items).into(),
    })
}

fn parse_string(data: &mut &[u8]) -> Result<NbtString> {
    let len = parse_short(data)?;
    let str = data
        .split_off(..len as u16 as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;

    let str = std::string::String::from_utf8(str.into()).map_err(|_| ParseError::InvalidUtf8)?;
    Ok(NbtString { str })
}

fn parse_int_array(data: &mut &[u8]) -> Result<NbtArray<i32>> {
    let len = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let items = data
        .split_off(..len as usize * 4)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let items = items
        .chunks_exact(4)
        .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok(NbtArray { items })
}

fn parse_long_array(data: &mut &[u8]) -> Result<NbtArray<i64>> {
    let len = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let items = data
        .split_off(..len as usize * 8)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let items = items
        .chunks_exact(8)
        .map(|chunk| i64::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok(NbtArray { items })
}

fn parse_list(data: &mut &[u8]) -> Result<NbtList> {
    let tag_id = parse_tag_id(data)?;
    match tag_id {
        NbtTagId::Byte => {
            return parse_byte_array(data).map(NbtList::Byte);
        }
        NbtTagId::Int => {
            return parse_int_array(data).map(NbtList::Int);
        }
        NbtTagId::Long => {
            return parse_long_array(data).map(NbtList::Long);
        }
        _ => {}
    }

    let len = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    macro_rules! parse {
        ($parser:ident, $variant:ident) => {{
            let mut items = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let item = $parser(data)?;
                items.push(item);
            }
            NbtList::$variant(NbtArray {
                items: items.into(),
            })
        }};
    }

    Ok(match tag_id {
        NbtTagId::End => NbtList::End,
        NbtTagId::Short => parse!(parse_short, Short),
        NbtTagId::Float => parse!(parse_float, Float),
        NbtTagId::Double => parse!(parse_double, Double),
        NbtTagId::ByteArray => parse!(parse_byte_array, ByteArray),
        NbtTagId::String => parse!(parse_string, String),
        NbtTagId::List => parse!(parse_list, List),
        NbtTagId::Compound => parse!(parse_compound, Compound),
        NbtTagId::IntArray => parse!(parse_int_array, IntArray),
        NbtTagId::LongArray => parse!(parse_long_array, LongArray),
        NbtTagId::Byte | NbtTagId::Int | NbtTagId::Long => unreachable!(),
    })
}

fn parse_payload(tag_id: NbtTagId, data: &mut &[u8]) -> Result<NbtTag> {
    macro_rules! parse {
        ($variant:ident, $parser:expr) => {
            $parser(data).map(|v| NbtTag::$variant(v))
        };
    }

    match tag_id {
        NbtTagId::End => Ok(NbtTag::End),
        NbtTagId::Byte => parse!(Byte, parse_byte),
        NbtTagId::Short => parse!(Short, parse_short),
        NbtTagId::Int => parse!(Int, parse_int),
        NbtTagId::Long => parse!(Long, parse_long),
        NbtTagId::Float => parse!(Float, parse_float),
        NbtTagId::Double => parse!(Double, parse_double),
        NbtTagId::ByteArray => parse!(ByteArray, parse_byte_array),
        NbtTagId::String => parse!(String, parse_string),
        NbtTagId::List => parse!(List, parse_list),
        NbtTagId::Compound => parse!(Compound, parse_compound),
        NbtTagId::IntArray => parse!(IntArray, parse_int_array),
        NbtTagId::LongArray => parse!(LongArray, parse_long_array),
    }
}

fn parse_compound(data: &mut &[u8]) -> Result<NbtCompound> {
    let mut compound: Vec<(NbtString, NbtTag)> = Vec::new();

    loop {
        let tag_id = parse_tag_id(data)?;
        if tag_id == NbtTagId::End {
            return Ok(NbtCompound { data: compound });
        }

        let name = parse_string(data)?;
        let tag = parse_payload(tag_id, data)?;

        if compound
            .iter()
            .any(|(existing_name, _)| existing_name.eq(&name))
        {
            return Err(ParseError::DuplicateTagName(name));
        }
        compound.push((name, tag));
    }
}

/// Parses a named NBT compound from a byte slice.
///
/// Expects the input to be a named NBT Compound, with no leftover data.
pub fn parse_nbt(mut data: &[u8]) -> Result<(NbtString, NbtCompound)> {
    let data = &mut data;
    let tag_id = parse_tag_id(data)?;
    if tag_id != NbtTagId::Compound {
        return Err(ParseError::NotNBT);
    }

    let name = parse_string(data)?;
    let tag = parse_compound(data)?;

    if !data.is_empty() {
        Err(ParseError::LeftoverData(data.len()))
    } else {
        Ok((name, tag))
    }
}
