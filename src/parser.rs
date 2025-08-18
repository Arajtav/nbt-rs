//! Functions for parsing NBT data.

use bytemuck::cast_slice;

use crate::{
    error::ParseError,
    types::{NbtArray, NbtCompound, NbtList, NbtString, NbtTag, NbtTagId},
};

/// A shorthand for `Result<T, ParseError>`.
pub type Result<T> = std::result::Result<T, ParseError>;

fn parse_tag_id(data: &[u8]) -> Result<(NbtTagId, &[u8])> {
    let (&tag_id, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
    let tag_id = NbtTagId::try_from(tag_id).map_err(|_| ParseError::InvalidTagId(tag_id))?;
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

fn parse_byte_array(data: &[u8]) -> Result<(NbtArray<i8>, &[u8])> {
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let (data, rest) = data
        .split_at_checked(len as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok((
        NbtArray {
            items: cast_slice(data).into(),
        },
        rest,
    ))
}

fn parse_string(data: &[u8]) -> Result<(NbtString, &[u8])> {
    let (len, data) = parse_short(data)?;
    let (data, rest) = data
        .split_at_checked(len as u16 as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;

    let data = std::string::String::from_utf8(data.into()).map_err(|_| ParseError::InvalidUtf8)?;
    Ok((NbtString { str: data }, rest))
}

fn parse_int_array(data: &[u8]) -> Result<(NbtArray<i32>, &[u8])> {
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let (data, rest) = data
        .split_at_checked(len as usize * 4)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let items = data
        .chunks_exact(4)
        .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok((NbtArray { items }, rest))
}

fn parse_long_array(data: &[u8]) -> Result<(NbtArray<i64>, &[u8])> {
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let (data, rest) = data
        .split_at_checked(len as usize * 8)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    let items = data
        .chunks_exact(8)
        .map(|chunk| i64::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok((NbtArray { items }, rest))
}

fn parse_list(data: &[u8]) -> Result<(NbtList, &[u8])> {
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
                NbtList::$variant(NbtArray {
                    items: items.into(),
                }),
                data,
            )
        }};
    }

    Ok(match tag_id {
        NbtTagId::End => (NbtList::End, data),
        NbtTagId::Byte => parse!(parse_byte, Byte),
        NbtTagId::Short => parse!(parse_short, Short),
        NbtTagId::Int => parse!(parse_int, Int),
        NbtTagId::Long => parse!(parse_long, Long),
        NbtTagId::Float => parse!(parse_float, Float),
        NbtTagId::Double => parse!(parse_double, Double),
        NbtTagId::ByteArray => parse!(parse_byte_array, ByteArray),
        NbtTagId::String => parse!(parse_string, String),
        NbtTagId::List => parse!(parse_list, List),
        NbtTagId::Compound => parse!(parse_compound, Compound),
        NbtTagId::IntArray => parse!(parse_int_array, IntArray),
        NbtTagId::LongArray => parse!(parse_long_array, LongArray),
    })
}

fn parse_payload(tag_id: NbtTagId, data: &[u8]) -> Result<(NbtTag, &[u8])> {
    macro_rules! parse {
        ($variant:ident, $parser:expr) => {
            $parser(data).map(|(v, rest)| (NbtTag::$variant(v), rest))
        };
    }

    match tag_id {
        NbtTagId::End => Ok((NbtTag::End, data)),
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

fn parse_compound(mut data: &[u8]) -> Result<(NbtCompound, &[u8])> {
    let mut compound: Vec<(NbtString, NbtTag)> = Vec::new();

    loop {
        let (tag_id, rest) = parse_tag_id(data)?;
        if tag_id == NbtTagId::End {
            return Ok((NbtCompound { data: compound }, rest));
        }

        let (name, rest) = parse_string(rest)?;
        let (tag, rest) = parse_payload(tag_id, rest)?;

        if compound
            .iter()
            .any(|(existing_name, _)| existing_name.eq(&name))
        {
            return Err(ParseError::DuplicateTagName(name));
        }
        compound.push((name, tag));
        data = rest;
    }
}

/// Parses a named NBT compound from a byte slice.
///
/// Expects the input to be a named NBT Compound, with no leftover data.
pub fn parse_nbt(data: &[u8]) -> Result<(NbtString, NbtCompound)> {
    let (tag_id, data) = parse_tag_id(data)?;
    if tag_id != NbtTagId::Compound {
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
