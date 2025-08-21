//! Functions for serializing NBT data into bytes.

use crate::{
    traits::NbtSerialize,
    types::{NbtCompound, NbtString, NbtTagId},
};

/// Serializes a named `NbtCompound` into a `Vec` of bytes.
/// This function is guaranteed never to fail nor to generate any invalid nbt data.
///
/// # Examples
/// ```
/// let raw = include_bytes!("../tests/data/level_uncompressed.dat");
/// let (name, compound) = nbt_rs::parse_nbt(raw).unwrap();
/// let serialized = nbt_rs::serialize_nbt(&name, &compound);
/// assert_eq!(serialized, raw);
/// ```
pub fn serialize_nbt(name: &NbtString, data: &NbtCompound) -> Vec<u8> {
    // 4KiB is small enough not to matter compared to how much
    // does it speed thing up for an average nbt file.
    let mut buffer = Vec::with_capacity(4096);
    buffer.push(NbtTagId::Compound as u8);
    name.serialize_nbt_payload(&mut buffer);
    data.serialize_nbt_payload(&mut buffer);
    buffer
}
