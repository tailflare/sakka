use super::{Codec, Decode, Encode};
use crate::{Endian, Error, Reader, Writer};

#[derive(Debug)]
enum InvertedBoolCodecError {
    Core(Error),
    InvalidDiscriminant(u8),
}

impl From<Error> for InvertedBoolCodecError {
    fn from(value: Error) -> Self {
        Self::Core(value)
    }
}

struct InvertedBoolCodec;

impl Codec<bool> for InvertedBoolCodec {
    type Error = InvertedBoolCodecError;

    fn encode(value: &bool, writer: &mut Writer<()>) -> Result<(), Self::Error> {
        let encoded = if *value { 0u8 } else { 1u8 };
        writer.write(&encoded)?;
        Ok(())
    }

    fn decode(reader: &mut Reader<'_, ()>) -> Result<bool, Self::Error> {
        match reader.read::<u8>()? {
            0 => Ok(true),
            1 => Ok(false),
            raw => Err(InvertedBoolCodecError::InvalidDiscriminant(raw)),
        }
    }
}

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

#[test]
fn custom_codec_encodes_decodes_and_surfaces_errors() {
    let mut writer = Writer::new(Endian::Big, ());
    InvertedBoolCodec::encode(&true, &mut writer).unwrap();
    assert_eq!(writer.finish(), [0x00]);

    let mut reader = Reader::new(&[0x01], Endian::Big, ());
    let decoded = InvertedBoolCodec::decode(&mut reader).unwrap();
    assert!(!decoded);

    let mut invalid = Reader::new(&[0xFF], Endian::Big, ());
    let invalid_err = InvertedBoolCodec::decode(&mut invalid).unwrap_err();
    assert!(matches!(invalid_err, InvertedBoolCodecError::InvalidDiscriminant(0xFF)));

    let mut truncated = Reader::new(&[], Endian::Big, ());
    let core_err = InvertedBoolCodec::decode(&mut truncated).unwrap_err();
    assert!(matches!(core_err, InvertedBoolCodecError::Core(Error::OutOfBounds)));
}
