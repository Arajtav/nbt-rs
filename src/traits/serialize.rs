/// Nbt Serialize Trait
///
/// A basic trait to serialize nbt payload.
/// Implemented for all the types nbt uses.
pub trait NbtSerialize {
    /// Serializes the value, extending the `buf`.
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>);
}

impl NbtSerialize for i8 {
    fn serialize_nbt_payload(&self, buf: &mut Vec<u8>) {
        buf.push(*self as u8);
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

impl_nbt_serialize_numeric!(i16);
impl_nbt_serialize_numeric!(u16);
impl_nbt_serialize_numeric!(i32);
impl_nbt_serialize_numeric!(u32);
impl_nbt_serialize_numeric!(i64);
impl_nbt_serialize_numeric!(u64);
impl_nbt_serialize_numeric!(f32);
impl_nbt_serialize_numeric!(f64);
