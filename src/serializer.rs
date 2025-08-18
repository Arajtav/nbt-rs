//! Functions for serializing NBT data into bytes.

use crate::{
    traits::NbtSerialize,
    types::{NbtCompound, NbtString, NbtTagId},
};

/// Serializes a named NBT Compound into a buffer of bytes.
/// This function is guaranteed not to fail, since all `Tag`s are validated at creation.
///
/// # Parameters
/// - `name`: The name of the root tag. Typically an empty string.
/// - `data`: The compound with data to serialize.
pub fn serialize_nbt(name: &NbtString, data: &NbtCompound) -> Box<[u8]> {
    let mut buffer = Vec::with_capacity(data.size as usize + 3 + name.len());
    buffer.push(NbtTagId::Compound as u8);
    name.serialize_nbt_payload(&mut buffer);
    data.serialize_nbt_payload(&mut buffer);
    buffer.into_boxed_slice()
}
