//! Functions for serializing NBT data into bytes.

use crate::{
    serializer::NbtSerialize,
    tags::{Compound, String, TagId},
};

/// Serializes a named NBT Compound into a buffer of bytes.
/// This function is guaranteed not to fail, since all `Tag`s are validated at creation.
///
/// # Parameters
/// - `name`: The name of the root tag. Typically an empty string.
/// - `data`: The compound with data to serialize.
pub fn serialize_nbt(name: &String, data: &Compound) -> Box<[u8]> {
    // would have just used serialize_named_tag, however i would need to clone for Tag::Compound()
    let mut buffer = Vec::new();
    buffer.push(TagId::Compound as u8);
    name.serialize_nbt_payload(&mut buffer);
    data.serialize_nbt_payload(&mut buffer);
    buffer.into_boxed_slice()
}
