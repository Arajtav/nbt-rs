use crate::error::ParseError;

/// Nbt Parse Trait
///
/// A basic trait to parse nbt payloads.
/// Implemented for all the types nbt uses.
pub trait NbtParse: Sized {
    /// Tries to parse the slice, returns the value and the slice without the consumed bytes.
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError>;
}

impl NbtParse for i8 {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (&out, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
        Ok((out as i8, rest))
    }
}

impl NbtParse for u8 {
    fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (&out, rest) = data.split_first().ok_or(ParseError::UnexpectedEndOfInput)?;
        Ok((out, rest))
    }
}

macro_rules! impl_nbt_parse_numeric {
    ($ty:ty, $width:expr) => {
        impl NbtParse for $ty {
            fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
                let (&out, rest) = data
                    // there should be a way to infer that tbh?
                    .split_first_chunk::<$width>()
                    .ok_or(ParseError::UnexpectedEndOfInput)?;
                return Ok((<$ty>::from_be_bytes(out), rest));
            }
        }
    };
}

// macro_rules! impl_nbt_parse_numeric {
//     ($ty:ty, $width:expr) => {
//         impl NbtParse for $ty {
//             fn try_parse_nbt_payload(data: &[u8]) -> Result<(Self, &[u8]), ParseError> {
//                 let (out, rest) = data
//                     .split_at_checked($width)
//                     .ok_or(ParseError::UnexpectedEndOfInput)?;
//                 return Ok((<$ty>::from_be_bytes(out.try_into().unwrap()), rest));
//             }
//         }
//     };
// }

impl_nbt_parse_numeric!(i16, 2);
// impl_nbt_parse_numeric!(u16, 2);
impl_nbt_parse_numeric!(i32, 4);
// impl_nbt_parse_numeric!(u32, 4);
impl_nbt_parse_numeric!(i64, 8);
// impl_nbt_parse_numeric!(u64, 8);
impl_nbt_parse_numeric!(f32, 4);
impl_nbt_parse_numeric!(f64, 8);
