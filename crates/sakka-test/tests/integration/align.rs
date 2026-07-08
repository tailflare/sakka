use sakka::{Decode, Encode, Endian, Reader, Writer};

const ALIGN_VALUE: usize = 4;
const ALIGN_8: usize = 8;

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignBefore {
    a: u8,
    #[sakka(align_before = 4)]
    b: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignAfter {
    a: u32,
    #[sakka(align_after = 4)]
    b: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignBothDifferentFields {
    a: u8,
    #[sakka(align_before = 4)]
    b: u32,
    #[sakka(align_after = 8)]
    c: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignBothSameField {
    a: u8,
    #[sakka(align_before = 4, align_after = 16)]
    b: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignWithConst {
    a: u8,
    #[sakka(align_before = ALIGN_VALUE)]
    b: u32,
    #[sakka(align_after = ALIGN_8)]
    c: u8,
}

#[test]
fn round_trip_align_before() {
    let value = AlignBefore { a: 0x12, b: 0x3456_7890 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x12, 0x00, 0x00, 0x00, 0x90, 0x78, 0x56, 0x34]
    // a=1 byte, 3 bytes padding, b=4 bytes
    assert_eq!(bytes.len(), 8);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: AlignBefore = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_align_after() {
    let value = AlignAfter { a: 0x1234_5678, b: 0xAB };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x78, 0x56, 0x34, 0x12, 0xAB, 0x00, 0x00, 0x00]
    // a=4 bytes, b=1 byte, 3 bytes padding to align after
    assert_eq!(bytes.len(), 8);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: AlignAfter = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_align_both() {
    let value = AlignBothDifferentFields { a: 0x12, b: 0x3456_7890, c: 0xCD };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // a=1 byte, 3 bytes padding before b, b=4 bytes, c=1 byte, 7 bytes padding after c
    // Total: 1 + 3 + 4 + 1 + 7 = 16
    assert_eq!(bytes.len(), 16);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: AlignBothDifferentFields = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_align_both_same_field() {
    let value = AlignBothSameField { a: 0x12, b: 0x3456_7890 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // a=1 byte, 3 bytes padding before b (align to 4), b=4 bytes (now at pos 8), 8 bytes padding after b (align to 16)
    // Total: 1 + 3 + 4 + 8 = 16
    assert_eq!(bytes.len(), 16);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: AlignBothSameField = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_align_with_const() {
    let value = AlignWithConst { a: 0x12, b: 0x3456_7890, c: 0xCD };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // a=1 byte, 3 bytes padding before b (align to 4), b=4 bytes, c=1 byte, 7 bytes padding after c (align to 8)
    // Total: 1 + 3 + 4 + 1 + 7 = 16
    assert_eq!(bytes.len(), 16);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: AlignWithConst = reader.read().unwrap();

    assert_eq!(decoded, value);
}
