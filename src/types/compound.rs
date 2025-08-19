use core::fmt;
use std::collections::HashMap;

use crate::{
    traits::NbtSerialize,
    types::{NbtString, NbtTag},
};

/// An NBT Compound.
///
/// Ensures no items have duplicate keys.
#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct NbtCompound {
    pub(crate) data: Vec<(NbtString, NbtTag)>,
}

impl NbtCompound {
    /// Gets an element by name.
    pub fn get(&self, str: &str) -> Option<&NbtTag> {
        self.data.iter().find(|e| e.0 == str).map(|e| &e.1)
    }
}

impl fmt::Display for NbtCompound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.data
                .iter()
                .map(|(k, v)| format!("{:?}: {}", k.to_string(), v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl NbtSerialize for NbtCompound {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        for (key, tag) in &self.data {
            buf.push(tag.tag_id() as u8);
            key.serialize_nbt_payload(buf);
            tag.serialize_nbt_payload(buf);
        }
        buf.push(0x00);
    }
}

impl From<NbtCompound> for Vec<(NbtString, NbtTag)> {
    fn from(val: NbtCompound) -> Self {
        val.data
    }
}

impl From<NbtCompound> for HashMap<NbtString, NbtTag> {
    fn from(val: NbtCompound) -> Self {
        val.data.into_iter().collect()
    }
}

impl From<HashMap<NbtString, NbtTag>> for NbtCompound {
    fn from(map: HashMap<NbtString, NbtTag>) -> Self {
        let data = map.into_iter().collect();
        NbtCompound { data }
    }
}
