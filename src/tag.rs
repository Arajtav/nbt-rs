use std::collections::HashMap;

use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
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
    List(List),
    Compound(Compound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug)]
pub struct Compound {
    data: HashMap<String, Tag>,
}

impl Default for Compound {
    fn default() -> Self {
        Self::new()
    }
}

impl Compound {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<Tag> {
        self.data.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&Tag> {
        self.data.get(key)
    }

    pub fn insert(&mut self, key: String, value: Tag) -> Option<Tag> {
        assert!(!matches!(value, Tag::End));
        self.data.insert(key, value)
    }
}

#[derive(Debug, EnumAsInner)]
pub enum List {
    End,
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<u8>>),
    String(Vec<String>),
    List(Vec<List>),
    Compound(Vec<Compound>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum TagId {
    End = 0,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl From<TagId> for u8 {
    fn from(tag: TagId) -> Self {
        tag as u8
    }
}

impl TryFrom<u8> for TagId {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::End),
            1 => Ok(Self::Byte),
            2 => Ok(Self::Short),
            3 => Ok(Self::Int),
            4 => Ok(Self::Long),
            5 => Ok(Self::Float),
            6 => Ok(Self::Double),
            7 => Ok(Self::ByteArray),
            8 => Ok(Self::String),
            9 => Ok(Self::List),
            10 => Ok(Self::Compound),
            11 => Ok(Self::IntArray),
            12 => Ok(Self::LongArray),
            _ => Err(()),
        }
    }
}
