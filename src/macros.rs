//! Helper macros.

/// Retrieves a field from an NBT compound, optionally performing type conversions.
///
/// # Parameters
/// - `input`: The `NbtCompound` (or a reference to one) from which to retrieve the field.
/// - `field`: The name of the field to retrieve, as a `&str`.
/// - `ty` (optional): A chain of conversion method names to call, e.g. `as_byte`, `as_compound.
///   Those methods can also be chained, like `as_list.as_compound` to convert to a list of compounds.
///
/// # Returns
/// An `Option` containing the field value, optionally converted according to `ty`.
/// Returns `None` if the field does not exist or if any conversion in the chain fails.
///
/// # Examples
///
/// Basic usage: get the raw field value
/// ```rust
/// use nbt_rs::{get_field, types::NbtTag, parse_nbt};
/// let compound = parse_nbt(include_bytes!("../tests/data/level_uncompressed.dat")).unwrap().1;
/// let data = get_field!(compound, "Data", as_compound).unwrap();
///
/// let difficulty = get_field!(data, "Difficulty");
/// assert_eq!(difficulty, Some(&NbtTag::Byte(2i8)));
///
/// let list = get_field!(data, "ServerBrands", as_list.as_string);
/// assert_eq!(
///     list,
///     Some(
///         &vec!["fabric".to_owned().try_into().unwrap()]
///             .try_into()
///             .unwrap()
///     )
/// );
/// ```
#[macro_export]
macro_rules! get_field {
    ($input:ident, $field:literal $(, $($ty:ident).*)? ) => {{
        $input.get($field)
        $(
            $(
                .and_then(|v| v.$ty())
            )*
        )?
    }};
}
