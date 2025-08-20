#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

// Based on https://minecraft.wiki/w/NBT_format#Binary_format

pub mod error;
pub mod macros;
pub mod parser;
pub mod serializer;
pub mod traits;
pub mod types;

pub use parser::parse_nbt;
pub use serializer::serialize_nbt;
