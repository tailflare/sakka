use alloc::vec;

use super::{ReadCollection, ReadPrimitive, Reader, WriteCollection, WritePrimitive, Writer};
use crate::Endian;

#[test]
fn basic_writer_reader_roundtrip() {
    let mut writer = Writer::new(Endian::Little, ());

    writer.write_u16(0x1234).unwrap();
    writer.write_bool_byte(true).unwrap();
    writer.write_prefixed_slice::<_, u8>(&[7u8, 8, 9]).unwrap();

    let bytes = writer.finish();

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let a = reader.read_u16().unwrap();
    let b = reader.read_bool_byte().unwrap();
    let c = reader.read_prefixed_vec::<u8, u8>().unwrap();

    assert_eq!(a, 0x1234);
    assert!(b);
    assert_eq!(c, vec![7, 8, 9]);
    assert!(reader.is_eof());
}
