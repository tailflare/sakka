use super::{Decode, Encode};
use crate::{Endian, Error, Reader, Writer};

#[derive(Debug, PartialEq, Eq)]
struct TestRecord {
    id: u16,
    tag: u8,
    active: bool,
}

impl Encode for TestRecord {
    type Error = Error;

    fn encode(&self, w: &mut Writer<()>) -> Result<(), Self::Error> {
        w.write(&self.id)?;
        w.write(&self.tag)?;
        w.write(&self.active)?;
        Ok(())
    }
}

impl Decode for TestRecord {
    type Error = Error;

    fn decode(r: &mut Reader<'_, ()>) -> Result<Self, Self::Error> {
        Ok(Self { id: r.read()?, tag: r.read()?, active: r.read()? })
    }
}

#[test]
fn custom_struct_roundtrips_with_encode_and_decode() {
    let value = TestRecord { id: 0xBEEF, tag: 7, active: true };

    let mut writer = Writer::new(Endian::Big, ());
    writer.write(&value).unwrap();
    let bytes = writer.finish();
    assert_eq!(bytes, [0xBE, 0xEF, 0x07, 0x01]);

    let mut reader = Reader::new(&bytes, Endian::Big, ());
    let decoded: TestRecord = reader.read().unwrap();

    assert_eq!(decoded, value);
    assert!(reader.is_eof());
}
