//! Definitions of the NBT tag types and related data structures.
//!
//! Provided types ensure the data can be serialized to a valid NBT by validating the data at creation time.

mod array;
mod errors;
mod list;
mod string;
mod tag;

pub use array::Array;
pub use errors::ValidationError;
pub use list::List;
pub use string::String;
pub use tag::Tag;
pub(crate) use tag::TagId;
