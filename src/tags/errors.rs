use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("The array is to long {0}/{max}", max = i32::MAX)]
    ArrayTooLong(usize),
    #[error("The string is to long {0}/{max}", max = u16::MAX)]
    StringTooLong(usize),
}
