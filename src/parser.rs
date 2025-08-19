//! Functions for parsing NBT data.

use crate::{
    error::ParseError,
    traits::NbtParse,
    types::{NbtCompound, NbtString, NbtTagId},
};

/// Parses a named NBT compound from a byte slice.
///
/// Expects the input to be a named NBT Compound, with no leftover data.
pub fn parse_nbt(data: &[u8]) -> Result<(NbtString, NbtCompound), ParseError> {
    let (tag_id, data) = NbtTagId::try_parse_nbt_payload(data)?;
    if tag_id != NbtTagId::Compound {
        return Err(ParseError::NotNBT);
    }

    let (name, data) = NbtString::try_parse_nbt_payload(data)?;
    let (tag, data) = NbtCompound::try_parse_nbt_payload(data)?;

    if !data.is_empty() {
        Err(ParseError::LeftoverData(data.len()))
    } else {
        Ok((name, tag))
    }
}
