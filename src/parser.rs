use thiserror::Error;

use crate::tag::{List, NamedTag, Tag, TagId};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid tag ID: {0}")]
    InvalidTagId(u8),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
    #[error("Negative length encountered: {0}")]
    NegativeLength(i32),
    #[error("Leftover data: {0} bytes")]
    LeftoverData(usize),
}

pub type Result<T> = std::result::Result<T, ParseError>;

fn parse_tag_id(data: &[u8]) -> Result<(TagId, &[u8])> {
    let (&tag_id, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
    let tag_id = TagId::try_from(tag_id).map_err(|_| ParseError::InvalidTagId(tag_id))?;
    Ok((tag_id, rest))
}

fn parse_named_tag(data: &[u8]) -> Result<(NamedTag, &[u8])> {
    let (tag_id, data) = parse_tag_id(data)?;
    if tag_id == TagId::End {
        return Ok((
            NamedTag {
                name: "".to_string(),
                tag: Tag::End,
            },
            data,
        ));
    }

    let (name, data) = parse_string(data)?;
    let (tag, data) = parse_payload(tag_id, data)?;
    Ok((NamedTag { name, tag }, data))
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

fn parse_byte_array(data: &[u8]) -> Result<(Vec<u8>, &[u8])> {
    let (len, data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let (data, rest) = data
        .split_at_checked(len as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;
    Ok((Vec::from(data), rest))
}

fn parse_string(data: &[u8]) -> Result<(String, &[u8])> {
    let (len, data) = parse_short(data)?;
    let (data, rest) = data
        .split_at_checked(len as u16 as usize)
        .ok_or(ParseError::UnexpectedEndOfInput)?;

    let data = String::from_utf8(data.into()).map_err(|_| ParseError::InvalidUtf8)?;
    Ok((data, rest))
}

fn parse_int_array(data: &[u8]) -> Result<(Vec<i32>, &[u8])> {
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

    Ok((data, rest))
}

fn parse_long_array(data: &[u8]) -> Result<(Vec<i64>, &[u8])> {
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

    Ok((data, rest))
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
            (List::$variant(items), data)
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

fn parse_compound(mut data: &[u8]) -> Result<(Vec<NamedTag>, &[u8])> {
    let mut tags = Vec::new();

    loop {
        let (tag_id, rest) = parse_tag_id(data)?;
        if tag_id == TagId::End {
            return Ok((tags, rest));
        }

        let (tag, rest) = parse_named_tag(data)?;
        tags.push(tag);
        data = rest;
    }
}

pub fn parse_nbt(data: &[u8]) -> Result<NamedTag> {
    let (tag, data) = parse_named_tag(data)?;
    if !data.is_empty() {
        Err(ParseError::LeftoverData(data.len()))
    } else {
        Ok(tag)
    }
}
