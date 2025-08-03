use nbt_rs::parser::parse_nbt;

#[test]
fn test_parse_level_dat() {
    let root = parse_nbt(include_bytes!("data/level_uncompressed.dat"));
    assert!(root.is_ok())
}
