use nbt_rs::parser::{ParseError, parse_nbt};

#[test]
fn test_parse_invalid_tag_id() {
    let data = &[0x0A, 0x00, 0x00, 0xff, 0x00];

    let err = parse_nbt(data).unwrap_err();
    assert!(matches!(err, ParseError::InvalidTagId(0xff)));
}

#[test]
fn test_parse_invalid_tag_id_list() {
    let data = &[
        0x0A, 0x00, 0x00, 0x09, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00,
    ];

    let err = parse_nbt(data).unwrap_err();
    assert!(matches!(err, ParseError::InvalidTagId(0xff)));
}

#[test]
fn test_parse_too_short() {
    let data = &[0x0A, 0x00, 0x00, 0x01, 0x00, 0x00];

    let err = parse_nbt(data).unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedEndOfInput));
}

#[test]
fn test_parse_invalid_utf8() {
    let data = &[0x0A, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x80, 0x00];

    let err = parse_nbt(data).unwrap_err();
    assert!(matches!(err, ParseError::InvalidUtf8));
}

#[test]
fn test_parse_leftover_data() {
    let data = &[0x0A, 0x00, 0x00, 0x00, 0xff];

    let err = parse_nbt(data).unwrap_err();
    assert!(matches!(err, ParseError::LeftoverData(1)));
}

#[test]
fn test_parse_array_negative_lengths() {
    // byte array
    let mut data = [
        0x0A, 0x00, 0x00, 0x07, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00,
    ];

    let err = parse_nbt(&data).unwrap_err();
    assert!(matches!(err, ParseError::NegativeLength(-1)));

    // int array
    data[3] = 0x0b;
    let err = parse_nbt(&data).unwrap_err();
    assert!(matches!(err, ParseError::NegativeLength(-1)));

    // long array
    data[3] = 0x0c;
    let err = parse_nbt(&data).unwrap_err();
    assert!(matches!(err, ParseError::NegativeLength(-1)));
}

#[test]
fn test_parse_duplicate_tag_names() {
    let data = &[
        0x0A, 0x00, 0x00, 0x01, 0x00, 0x01, b'a', 0x00, 0x01, 0x00, 0x01, b'a', 0x01, 0x00,
    ];

    let err = parse_nbt(data).unwrap_err();
    assert!(matches!(err, ParseError::DuplicateTagName(ref s) if s == "a"));
}

#[test]
fn test_parse_non_compound() {
    let data = &[0x00];

    let err = parse_nbt(data).unwrap_err();
    assert!(matches!(err, ParseError::NotNBT));
}
