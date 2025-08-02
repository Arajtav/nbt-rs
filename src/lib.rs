// Based on https://minecraft.wiki/w/NBT_format#Binary_format

use thiserror::Error;

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

#[derive(Debug)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Tag>),
    Compound(Vec<NamedTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug)]
pub struct NamedTag {
    pub name: String,
    pub tag: Tag,
}

fn parse_named_tag(data: &[u8]) -> Result<(NamedTag, &[u8])> {
    let (&tag_id, data) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
    if tag_id == 0 {
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

fn parse_payload(tag_id: u8, data: &[u8]) -> Result<(Tag, &[u8])> {
    match tag_id {
        0 => parse_end(data),
        1 => parse_byte(data).map(|(v, rest)| (Tag::Byte(v), rest)),
        2 => parse_short(data).map(|(v, rest)| (Tag::Short(v), rest)),
        3 => parse_int(data).map(|(v, rest)| (Tag::Int(v), rest)),
        4 => parse_long(data).map(|(v, rest)| (Tag::Long(v), rest)),
        5 => parse_float(data).map(|(v, rest)| (Tag::Float(v), rest)),
        6 => parse_double(data).map(|(v, rest)| (Tag::Double(v), rest)),
        7 => parse_byte_array(data).map(|(v, rest)| (Tag::ByteArray(v), rest)),
        8 => parse_string(data).map(|(v, rest)| (Tag::String(v), rest)),
        9 => parse_list(data),
        10 => parse_compound(data),
        11 => parse_int_array(data).map(|(v, rest)| (Tag::IntArray(v), rest)),
        12 => parse_long_array(data).map(|(v, rest)| (Tag::LongArray(v), rest)),
        _ => Err(ParseError::InvalidTagId(tag_id)),
    }
}

fn parse_end(data: &[u8]) -> Result<(Tag, &[u8])> {
    Ok((Tag::End, data))
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

fn parse_list(data: &[u8]) -> Result<(Tag, &[u8])> {
    let (&element_type, data) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
    let (len, mut data) = parse_int(data)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let mut items = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (tag, rest) = parse_payload(element_type, data)?;
        items.push(tag);
        data = rest;
    }

    Ok((Tag::List(items), data))
}

fn parse_compound(mut data: &[u8]) -> Result<(Tag, &[u8])> {
    let mut tags = Vec::new();

    loop {
        let (&tag_id, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
        if tag_id == 0 {
            return Ok((Tag::Compound(tags), rest));
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
