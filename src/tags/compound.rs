use std::collections::HashMap;

use crate::tags::{String, Tag};

/// An NBT Compound.
///
/// Ensures no items have duplicate keys.
#[derive(Debug, PartialEq)]
pub struct Compound {
    pub(crate) data: Vec<(String, Tag)>,
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
