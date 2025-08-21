//! Functions for parsing NBT data.

use bytemuck::cast_slice;

use crate::{
    error::ParseError,
    types::{NbtArray, NbtCompound, NbtList, NbtString, NbtTag, NbtTagId},
};

type Result<T> = std::result::Result<T, ParseError>;

#[inline(always)]
fn split_1(mut data: &[u8]) -> Result<(u8, &[u8])> {
    let &a = data
        .split_off_first()
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok((a, data))
}

macro_rules! impl_splt {
    ($name:ident, $width:expr) => {
        #[inline(always)]
        fn $name(data: &[u8]) -> Result<([u8; $width], &[u8])> {
            let (&a, data) = data
                .split_first_chunk::<$width>()
                .ok_or(ParseError::UnexpectedEndOfInput)?;
            Ok((a, data))
        }
    };
}

impl_splt!(split_2, 2);
impl_splt!(split_4, 4);
impl_splt!(split_8, 8);

fn parse_tag_id(data: &[u8]) -> Result<(NbtTagId, &[u8])> {
    let (tag_id, rest) = split_1(data)?;
    let tag_id = NbtTagId::try_from(tag_id).map_err(|_| ParseError::InvalidTagId(tag_id))?;
    Ok((tag_id, rest))
}

#[inline(always)]
fn parse_byte(data: &[u8]) -> Result<(i8, &[u8])> {
    let (v, rest) = split_1(data)?;
    Ok((v as i8, rest))
}

macro_rules! impl_parse_numeric {
    ($name:ident, $ty:ty, $split:ident) => {
        #[inline(always)]
        fn $name(data: &[u8]) -> Result<($ty, &[u8])> {
            let (bytes, rest) = $split(data)?;
            Ok((<$ty>::from_be_bytes(bytes), rest))
        }
    };
}

impl_parse_numeric!(parse_short, i16, split_2);
impl_parse_numeric!(parse_int, i32, split_4);
impl_parse_numeric!(parse_long, i64, split_8);
impl_parse_numeric!(parse_float, f32, split_4);
impl_parse_numeric!(parse_double, f64, split_8);

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
    match tag_id {
        NbtTagId::Byte => {
            return parse_byte_array(data).map(|p| (NbtList::Byte(p.0), p.1));
        }
        NbtTagId::Int => {
            return parse_int_array(data).map(|p| (NbtList::Int(p.0), p.1));
        }
        NbtTagId::Long => {
            return parse_long_array(data).map(|p| (NbtList::Long(p.0), p.1));
        }
        _ => {}
    }
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

/// Parses a named `NbtCompound` from a byte slice.
///
/// # Errors
/// Will fail if the nbt data is invalid, or if there is any leftover data after parsing.
///
/// # Examples
/// ```
/// use nbt_rs::{parse_nbt, get_field};
/// let data = &[
///     0x0a, 0x00, 0x00, 0x01, 0x00, 0x01, b'a', 0x00, 0x01, 0x00, 0x01, b'b', 0x01, 0x00,
/// ];
///
/// let (name, root) = parse_nbt(data).unwrap();
/// assert!(name.is_empty());
/// assert_eq!(get_field!(root, "a", as_byte).unwrap(), &0x00);
/// assert_eq!(get_field!(root, "b", as_byte).unwrap(), &0x01);
/// ```
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
