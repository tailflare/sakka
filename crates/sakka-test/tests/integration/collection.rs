use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionPrefix {
    #[sakka(collection(prefix = u32))]
    b: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionCount {
    #[sakka(collection(count = 5))]
    b: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionArray {
    b: [u8; 5],
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionFieldCount {
    count: u8,
    #[sakka(collection(field = count))]
    b: Vec<u8>,
}

#[test]
fn round_trip_collection_with_prefix() {
    let value = CollectionPrefix { b: vec![0x12, 0x34, 0x56, 0x78, 0xAB] };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x05, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, 0xAB]
    // u32 prefix (5) followed by 5 bytes
    assert_eq!(bytes.len(), 9);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CollectionPrefix = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_collection_with_count() {
    let value = CollectionCount { b: vec![0x12, 0x34, 0x56, 0x78, 0xAB] };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x12, 0x34, 0x56, 0x78, 0xAB]
    // No prefix, just 5 bytes (count is compile-time constant)
    assert_eq!(bytes.len(), 5);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CollectionCount = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_array() {
    let value = CollectionArray { b: [0x12, 0x34, 0x56, 0x78, 0xAB] };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Should be: [0x12, 0x34, 0x56, 0x78, 0xAB]
    // Fixed-size array, no prefix
    assert_eq!(bytes.len(), 5);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CollectionArray = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_collection_with_field_count() {
    let value = CollectionFieldCount { count: 5, b: vec![0x12, 0x34, 0x56, 0x78, 0xAB] };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    // Count is stored in its own field, so the collection data is not length-prefixed.
    assert_eq!(bytes.len(), 6);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CollectionFieldCount = reader.read().unwrap();

    assert_eq!(decoded, value);
}
