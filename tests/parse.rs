use nbt_rs::parser::parse_nbt;

#[test]
fn test_parse_level_dat() {
    let root = parse_nbt(include_bytes!("data/level_uncompressed.dat"));
    assert!(root.is_ok())
}

#[test]
fn test_parse_simple_compound() {
    let data = &[
        0x0A, 0x00, 0x00, 0x01, 0x00, 0x01, b'a', 0x00, 0x01, 0x00, 0x01, b'b', 0x01, 0x00,
    ];

    let root = parse_nbt(data).unwrap();
    assert!(root.0.is_empty());
    let a = *root.1.get("a").unwrap().as_byte().unwrap();
    let b = *root.1.get("b").unwrap().as_byte().unwrap();
    assert_eq!(a, 0x00);
    assert_eq!(b, 0x01);
}
