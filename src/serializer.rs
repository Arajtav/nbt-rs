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
pub fn serialize_nbt(name: &NbtString, data: &NbtCompound) -> Vec<u8> {
    // 4KiB is small enough to not matter compared how much
    // does it speed thing up for an average nbt file
    let mut buffer = Vec::with_capacity(4096);
    buffer.push(NbtTagId::Compound as u8);
    name.serialize_nbt_payload(&mut buffer);
    data.serialize_nbt_payload(&mut buffer);
    buffer
}
