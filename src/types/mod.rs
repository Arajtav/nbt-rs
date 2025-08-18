//! Definitions of the NBT tag types and related data structures.
//!
//! Provided types ensure the data can be serialized to a valid NBT by validating the data at creation time.

mod array;
mod compound;
mod list;
mod string;
mod tag;

pub use array::NbtArray;
pub use compound::NbtCompound;
pub use list::NbtList;
pub use string::NbtString;
pub use tag::NbtTag;
pub(crate) use tag::NbtTagId;
