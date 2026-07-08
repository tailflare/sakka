use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, PartialEq, Encode, Decode)]
struct IgnoreDefault {
    first: u8,
    #[sakka(ignore)]
    ignored: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IgnoreValue {
    first: u8,
    #[sakka(ignore = 0xBEEFu16)]
    ignored: u16,
}

#[test]
fn round_trip_ignore_default() {
    let value = IgnoreDefault { first: 0x2A, ignored: 0xBEEF };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();
    let bytes = writer.finish();

    // ignored is not encoded, so only `first` is present
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes, vec![0x2A]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: IgnoreDefault = reader.read().unwrap();

    assert_eq!(decoded, IgnoreDefault { first: 0x2A, ignored: 0 });
}

#[test]
fn round_trip_ignore_value() {
    let value = IgnoreValue { first: 0x11, ignored: 0x1234 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();
    let bytes = writer.finish();

    // ignored is not encoded, so only `first` is present
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes, vec![0x11]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: IgnoreValue = reader.read().unwrap();

    assert_eq!(decoded, IgnoreValue { first: 0x11, ignored: 0xBEEF });
}
