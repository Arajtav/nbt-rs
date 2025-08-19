/// Nbt Serialize Trait
///
/// A basic trait to serialize nbt payload.
/// Implemented for all the types nbt uses.
pub trait NbtSerialize {
    /// Serializes the value, extending the `buf`.
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>);
}

impl NbtSerialize for i8 {
    #[inline(always)]
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        (*self as u8).serialize_nbt_payload(buf);
    }
}

impl NbtSerialize for u8 {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        buf.push(*self);
    }
}

macro_rules! impl_nbt_serialize_numeric {
    ($ty:ty) => {
        impl NbtSerialize for $ty {
            fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
                buf.extend_from_slice(&self.to_be_bytes());
            }
        }
    };
}

// a bit better cache efficiency that way
macro_rules! impl_nbt_serialize_numeric_cast {
    ($ty:ty, $from:ty) => {
        impl NbtSerialize for $ty {
            #[inline(always)]
            fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
                (*self as $from).serialize_nbt_payload(buf)
            }
        }
    };
}

impl_nbt_serialize_numeric!(i16);
impl_nbt_serialize_numeric_cast!(u16, i16);
impl_nbt_serialize_numeric!(i32);
impl_nbt_serialize_numeric_cast!(u32, i32);
impl_nbt_serialize_numeric!(i64);
impl_nbt_serialize_numeric_cast!(u64, i64);
impl_nbt_serialize_numeric!(f32);
impl_nbt_serialize_numeric!(f64);
