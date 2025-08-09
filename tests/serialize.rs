use nbt_rs::{parser::parse_nbt, serializer::serialize_nbt};

#[test]
fn test_reparse_level_dat() {
    // i guess this test depends on whether the parsing works correctly, which is not optimal. it is
    // however the best i can do, as the Compound/HashMap doesn't care about the order of tags,
    // making comparison between hand written binary, and serialized object impossible.
    let data = include_bytes!("data/level_uncompressed.dat");
    let (name, parsed) = parse_nbt(data).unwrap();
    let serialized = serialize_nbt(&name, &parsed);
    let (new_name, new_parsed) = parse_nbt(&serialized).unwrap();
    // the amount of data should stay the same though.
    assert_eq!(data.len(), serialized.len());
    assert_eq!(name, new_name);
    assert_eq!(parsed, new_parsed);
}
