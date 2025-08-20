//! Helper macros.

/// Retrieves a field from an NBT compound, optionally performing type conversions.
///
/// # Parameters
/// - `input`: The `NbtCompound` (or a reference to one) from which to retrieve the field.
/// - `field`: One or more field names as string literals (e.g. `"Data"."Difficulty"`).
/// - `ty` (optional): A chain of conversion method names to call, e.g. `as_byte`, `as_compound.
///   Those methods can also be chained, like `as_list.as_compound` to convert to a list of compounds.
///
/// # Returns
/// An `Option` containing the field value, optionally converted according to `ty`.
/// Returns `None` if any of the fields does not exist or if any conversion in the chain fails.
///
/// # Examples
/// ```
/// use nbt_rs::{get_field, types::NbtTag, parse_nbt};
/// let compound = parse_nbt(include_bytes!("../tests/data/level_uncompressed.dat")).unwrap().1;
/// // Simply get one filed.
/// debug_assert!(get_field!(compound, "Data").is_some());
///
/// // Get a nested field.
/// let difficulty = get_field!(compound, "Data"."Difficulty");
/// assert_eq!(difficulty, Some(&NbtTag::Byte(2i8)));
///
/// // Get and convert a nested filed.
/// let list = get_field!(compound, "Data"."ServerBrands", as_list.as_string);
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
    ($input:ident, $first:literal $(. $rest:literal)* $(, $($ty:ident).*)? ) => {{
        $input.get($first)
        $(
            .and_then(|v| v.as_compound())
            .and_then(|v| v.get($rest))
        )*
        $(
            $(
                .and_then(|v| v.$ty())
            )*
        )?
    }};
}
