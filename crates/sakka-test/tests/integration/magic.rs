use sakka::{Decode, Encode, Endian, Error, Reader, Writer};

const MAGIC_PREFIX: [u8; 2] = [0xCA, 0xFE];

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(magic = 0xABCD_u16)]
struct MagicStruct {
    value: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(magic = MAGIC_PREFIX)]
enum MagicEnum {
    A,
    B(u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(magic = b"MAGIC")]
struct MagicBinStruct {
    value: u8,
}

#[test]
fn magic_struct_writes_magic_prefix() {
    let value = MagicStruct { value: 7 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0xCD, 0xAB, 0x07]);
}

#[test]
fn magic_struct_decodes_when_magic_matches() {
    let bytes = [0xCD, 0xAB, 0x42];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let value: MagicStruct = reader.read().unwrap();
    assert_eq!(value, MagicStruct { value: 0x42 });
}

#[test]
fn magic_struct_decode_fails_when_magic_mismatches() {
    let bytes = [0x00, 0xAB, 0x42];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let err = reader.read::<MagicStruct>().unwrap_err();
    assert!(matches!(err, Error::InvalidMagic(_)));
}

#[test]
fn magic_enum_round_trip_with_array_magic() {
    let value = MagicEnum::B(9);

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0xCA, 0xFE, 0x01, 0x09]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: MagicEnum = reader.read().unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn magic_enum_decode_fails_when_magic_mismatches() {
    let bytes = [0xCA, 0x00, 0x00];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let err = reader.read::<MagicEnum>().unwrap_err();
    assert!(matches!(err, Error::InvalidMagic(_)));
}

#[test]
fn magic_bin_struct_writes_magic_prefix() {
    let value = MagicBinStruct { value: 7 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, b"MAGIC\x07");
}

#[test]
fn magic_bin_struct_decode_fails_when_magic_mismatches() {
    let bytes = b"MUGIC\x07";
    let mut reader = Reader::new(bytes, Endian::Little, ());

    let err = reader.read::<MagicBinStruct>().unwrap_err();
    assert!(matches!(err, Error::InvalidMagic(_)));
}
