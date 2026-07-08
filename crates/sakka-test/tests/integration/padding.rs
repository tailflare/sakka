use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadBefore {
    #[sakka(pad_before = 2)]
    b: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadAfter {
    #[sakka(pad_after = 2)]
    b: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadBoth {
    #[sakka(pad_before = 1)]
    a: u8,
    #[sakka(pad_after = 1)]
    b: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadBothSameField {
    #[sakka(pad_before = 1, pad_after = 2)]
    b: u8,
}

#[test]
fn pad_before_only() {
    let value = PadBefore { b: 0x42 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x00, 0x00, 0x42]
    // 2 bytes padding, then the value
    assert_eq!(bytes.len(), 3);
    assert_eq!(&bytes[..2], &[0x00, 0x00]);
    assert_eq!(bytes[2], 0x42);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: PadBefore = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn pad_after_only() {
    let value = PadAfter { b: 0x42 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x42, 0x00, 0x00]
    // value, then 2 bytes padding
    assert_eq!(bytes.len(), 3);
    assert_eq!(bytes[0], 0x42);
    assert_eq!(&bytes[1..], &[0x00, 0x00]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: PadAfter = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn pad_both() {
    let value = PadBoth { a: 0x11, b: 0x22 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x00, 0x11, 0x22, 0x00]
    // pad_before for a (1), then a (1), then b (1), then pad_after for b (1)
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes[0], 0x00);
    assert_eq!(bytes[1], 0x11);
    assert_eq!(bytes[2], 0x22);
    assert_eq!(bytes[3], 0x00);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: PadBoth = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn pad_both_same_field() {
    let value = PadBothSameField { b: 0x55 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x00, 0x55, 0x00, 0x00]
    // pad_before (1), then b (1), then pad_after (2)
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes[0], 0x00);
    assert_eq!(bytes[1], 0x55);
    assert_eq!(&bytes[2..], &[0x00, 0x00]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: PadBothSameField = reader.read().unwrap();

    assert_eq!(decoded, value);
}
