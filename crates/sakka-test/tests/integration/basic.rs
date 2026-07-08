use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicStruct {
    magic: u32,
    version: u16,
    valid: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicWithNestedStruct {
    magic: u32,
    version: u16,
    nested: BasicStruct,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicTupleStruct(u32, u16, bool);

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicUnitStruct;

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicGenericStruct<T> {
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicGenericWithWhereStruct<T>
where
    T: Copy,
{
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicDoubleGenericStruct<A, B> {
    first: A,
    second: B,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicConstGenericStruct<T, const N: usize> {
    values: [T; N],
}

#[test]
fn round_trip_basic_struct() {
    let value = BasicStruct { magic: 0x1234_5678, version: 42, valid: true };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicStruct = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_basic_with_nested_struct() {
    let nested = BasicStruct { magic: 0x1234_5678, version: 42, valid: true };
    let value = BasicWithNestedStruct { magic: 0xDEAD_BEEF, version: 99, nested };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicWithNestedStruct = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_basic_tuple_struct() {
    let value = BasicTupleStruct(0x1234_5678, 42, true);

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicTupleStruct = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_basic_unit_struct() {
    let value = BasicUnitStruct;

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicUnitStruct = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_basic_generic_struct() {
    let value = BasicGenericStruct { value: 42 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicGenericStruct<i32> = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_basic_generic_with_where_struct() {
    let value = BasicGenericWithWhereStruct { value: 42 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicGenericWithWhereStruct<i32> = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_basic_double_generic_struct() {
    let value = BasicDoubleGenericStruct { first: 42, second: 99 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicDoubleGenericStruct<i32, i32> = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_basic_const_generic_struct() {
    let value = BasicConstGenericStruct { values: [1, 2, 3, 4, 5] };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicConstGenericStruct<i32, 5> = reader.read().unwrap();

    assert_eq!(decoded, value);
}
