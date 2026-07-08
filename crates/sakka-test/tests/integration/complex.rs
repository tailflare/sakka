use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, PartialEq, Encode, Decode)]
struct PaddingAndAlignment {
    #[sakka(pad_before = 1, align_before = 4, align_after = 4, pad_after = 1)]
    b: u8,
}

#[test]
fn padding_with_alignment() {
    let value = PaddingAndAlignment { b: 0xAA };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Structure should be:
    // pad_before (1) -> [0x00]
    // align_before (4) -> [0x00, 0x00, 0x00] (already at 1, need to reach 4)
    // value (1) -> [0xAA]
    // align_after (4) -> [0x00, 0x00, 0x00] (at 5, need to reach 8)
    // pad_after (1) -> [0x00]
    // Total: 9 bytes
    assert_eq!(bytes.len(), 9);
    assert_eq!(bytes[0], 0x00); // pad_before
    assert_eq!(&bytes[1..4], &[0x00, 0x00, 0x00]); // align_before padding
    assert_eq!(bytes[4], 0xAA); // value
    assert_eq!(&bytes[5..8], &[0x00, 0x00, 0x00]); // align_after padding
    assert_eq!(bytes[8], 0x00); // pad_after

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: PaddingAndAlignment = reader.read().unwrap();

    assert_eq!(decoded, value);
}
