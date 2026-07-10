use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalBool {
    #[sakka(optional(bool))]
    value: Option<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalEof {
    #[sakka(optional(eof))]
    value: Option<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalBoolCollection {
    #[sakka(optional(bool), collection(prefix = u8))]
    value: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalEofCollection {
    #[sakka(optional(eof), collection(prefix = u8))]
    value: Option<Vec<u8>>,
}

#[test]
fn round_trip_optional_bool_some_and_none() {
    let some = OptionalBool { value: Some(0xBEEF) };
    let none = OptionalBool { value: None };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&some).unwrap();
    writer.write(&none).unwrap();
    let bytes = writer.finish();

    assert_eq!(bytes, vec![1, 0xEF, 0xBE, 0]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded_some: OptionalBool = reader.read().unwrap();
    let decoded_none: OptionalBool = reader.read().unwrap();

    assert_eq!(decoded_some, some);
    assert_eq!(decoded_none, none);
}

#[test]
fn optional_eof_reads_none_at_end_of_input() {
    let bytes = [0x34u8, 0x12];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let first: OptionalEof = reader.read().unwrap();
    let second: OptionalEof = reader.read().unwrap();

    assert_eq!(first, OptionalEof { value: Some(0x1234) });
    assert_eq!(second, OptionalEof { value: None });
}

#[test]
fn optional_eof_writer_only_writes_some() {
    let some = OptionalEof { value: Some(0x1234) };
    let none = OptionalEof { value: None };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&some).unwrap();
    writer.write(&none).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0x34, 0x12]);
}

#[test]
fn optional_bool_collection_round_trip() {
    let some = OptionalBoolCollection { value: Some(vec![0x10, 0x20, 0x30]) };
    let none = OptionalBoolCollection { value: None };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&some).unwrap();
    writer.write(&none).unwrap();
    let bytes = writer.finish();

    assert_eq!(bytes, vec![1, 3, 0x10, 0x20, 0x30, 0]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded_some: OptionalBoolCollection = reader.read().unwrap();
    let decoded_none: OptionalBoolCollection = reader.read().unwrap();

    assert_eq!(decoded_some, some);
    assert_eq!(decoded_none, none);
}

#[test]
fn optional_eof_collection_round_trip_and_eof_none() {
    let some = OptionalEofCollection { value: Some(vec![0xAA, 0xBB]) };
    let none = OptionalEofCollection { value: None };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&some).unwrap();
    writer.write(&none).unwrap();
    let bytes = writer.finish();

    assert_eq!(bytes, vec![2, 0xAA, 0xBB]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded_some: OptionalEofCollection = reader.read().unwrap();
    let decoded_none: OptionalEofCollection = reader.read().unwrap();

    assert_eq!(decoded_some, some);
    assert_eq!(decoded_none, none);
}
