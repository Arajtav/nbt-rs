use std::collections::HashMap;

use crate::{
    serializer::NbtSerialize,
    tags::{String, Tag},
};

/// An NBT Compound.
///
/// Ensures no items have duplicate keys.
#[derive(Debug, PartialEq)]
pub struct Compound {
    pub(crate) data: Vec<(String, Tag)>,
}

impl NbtSerialize for Compound {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        for (key, tag) in &self.data {
            buf.push(tag.tag_id() as u8);
            key.serialize_nbt_payload(buf);
            tag.serialize_nbt_payload(buf);
        }
        buf.push(0x00);
    }
}

impl From<Compound> for Vec<(String, Tag)> {
    fn from(val: Compound) -> Self {
        val.data
    }
}

impl From<Compound> for HashMap<String, Tag> {
    fn from(val: Compound) -> Self {
        val.data.into_iter().collect()
    }
}

impl From<HashMap<String, Tag>> for Compound {
    fn from(map: HashMap<String, Tag>) -> Self {
        let data = map.into_iter().collect();
        Compound { data }
    }
}
