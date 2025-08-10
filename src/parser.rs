//! Functions for parsing NBT data.

use bytemuck::cast_slice;
use thiserror::Error;

use crate::tags::{Array, Compound, List, String, Tag, TagId};

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

fn parse_tag_id(cursor: &mut Cursor) -> Result<TagId> {
    let tag_id = cursor.read_byte()?;
    let tag_id = TagId::try_from(tag_id).map_err(|_| ParseError::InvalidTagId(tag_id))?;
    Ok(tag_id)
}

fn parse_byte(cursor: &mut Cursor) -> Result<i8> {
    let v = cursor.read_byte()? as i8;
    Ok(v)
}

fn parse_short(cursor: &mut Cursor) -> Result<i16> {
    let value = cursor.read_exact(2)?;
    let value = i16::from_be_bytes(value.try_into().unwrap());
    Ok(value)
}

fn parse_int(cursor: &mut Cursor) -> Result<i32> {
    let value = cursor.read_exact(4)?;
    let value = i32::from_be_bytes(value.try_into().unwrap());
    Ok(value)
}

fn parse_long(cursor: &mut Cursor) -> Result<i64> {
    let value = cursor.read_exact(8)?;
    let value = i64::from_be_bytes(value.try_into().unwrap());
    Ok(value)
}

fn parse_float(cursor: &mut Cursor) -> Result<f32> {
    let value = cursor.read_exact(4)?;
    let value = f32::from_be_bytes(value.try_into().unwrap());
    Ok(value)
}

fn parse_double(cursor: &mut Cursor) -> Result<f64> {
    let value = cursor.read_exact(8)?;
    let value = f64::from_be_bytes(value.try_into().unwrap());
    Ok(value)
}

fn parse_byte_array(cursor: &mut Cursor) -> Result<Array<i8>> {
    let len = parse_int(cursor)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let data = cursor.read_exact(len as usize)?;
    Ok(Array {
        items: cast_slice(data).into(),
    })
}

fn parse_string(cursor: &mut Cursor) -> Result<String> {
    let len = parse_short(cursor)?;
    let data = cursor.read_exact(len as u16 as usize)?;

    let data = std::string::String::from_utf8(data.into()).map_err(|_| ParseError::InvalidUtf8)?;
    Ok(String { str: data })
}

fn parse_int_array(cursor: &mut Cursor) -> Result<Array<i32>> {
    let len = parse_int(cursor)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let data = cursor.read_exact(len as usize * 4)?;
    let data = data
        .chunks_exact(4)
        .map(|chunk| i32::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok(Array { items: data })
}

fn parse_long_array(cursor: &mut Cursor) -> Result<Array<i64>> {
    let len = parse_int(cursor)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    let data = cursor.read_exact(len as usize * 8)?;
    let data = data
        .chunks_exact(8)
        .map(|chunk| i64::from_be_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok(Array { items: data })
}

fn parse_list(cursor: &mut Cursor) -> Result<List> {
    let tag_id = parse_tag_id(cursor)?;
    let len = parse_int(cursor)?;
    if len < 0 {
        return Err(ParseError::NegativeLength(len));
    }

    macro_rules! parse {
        ($parser:ident, $variant:ident) => {{
            let mut items = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let item = $parser(cursor)?;
                items.push(item);
            }
            List::$variant(Array {
                items: items.into(),
            })
        }};
    }

    Ok(match tag_id {
        TagId::End => List::End,
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

fn parse_payload(tag_id: TagId, cursor: &mut Cursor) -> Result<Tag> {
    macro_rules! parse {
        ($variant:ident, $parser:expr) => {
            $parser(cursor).map(|v| Tag::$variant(v))
        };
    }

    match tag_id {
        TagId::End => Ok(Tag::End),
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

fn parse_compound(cursor: &mut Cursor) -> Result<Compound> {
    let mut compound: Vec<(String, Tag)> = Vec::new();

    loop {
        let tag_id = parse_tag_id(cursor)?;
        if tag_id == TagId::End {
            return Ok(Compound { data: compound });
        }

        let name = parse_string(cursor)?;
        let tag = parse_payload(tag_id, cursor)?;

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
pub fn parse_nbt(data: &[u8]) -> Result<(String, Compound)> {
    let mut cursor = Cursor::new(data);
    let tag_id = parse_tag_id(&mut cursor)?;
    if tag_id != TagId::Compound {
        return Err(ParseError::NotNBT);
    }

    let name = parse_string(&mut cursor)?;
    let tag = parse_compound(&mut cursor)?;

    if !cursor.is_empty() {
        Err(ParseError::LeftoverData(cursor.remaining_len()))
    } else {
        Ok((name, tag))
    }
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn read_exact(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.pos + len > self.data.len() {
            return Err(ParseError::UnexpectedEndOfInput);
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        if self.pos + 1 > self.data.len() {
            return Err(ParseError::UnexpectedEndOfInput);
        }
        let out = self.data[self.pos];
        self.pos += 1;
        Ok(out)
    }

    pub fn remaining_len(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn is_empty(&self) -> bool {
        self.remaining_len() == 0
    }
}
