use std::collections::HashMap;

use nbt_rs::{
    parse_nbt,
    types::{NbtString, NbtTag},
};

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

    let (name, root) = parse_nbt(data).unwrap();
    let root: HashMap<NbtString, NbtTag> = root.into();
    assert!(name.is_empty());
    let a = *root.get("a").unwrap().as_byte().unwrap();
    let b = *root.get("b").unwrap().as_byte().unwrap();
    assert_eq!(a, 0x00);
    assert_eq!(b, 0x01);
}
