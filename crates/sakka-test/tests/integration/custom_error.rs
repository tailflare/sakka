use sakka::{Decode, Encode, Endian, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

#[derive(Debug)]
enum CustomCodecError {
    Core(Error),
    InvalidMarker(u8),
}

impl From<Error> for CustomCodecError {
    fn from(value: Error) -> Self {
        Self::Core(value)
    }
}

#[derive(Debug, PartialEq)]
struct CustomErrorRecord {
    marker: u8,
    value: u8,
}

impl<Ctx> Encode<Ctx> for CustomErrorRecord {
    type Error = CustomCodecError;

    fn encode(&self, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        if self.marker != 0xAB {
            return Err(CustomCodecError::InvalidMarker(self.marker));
        }

        writer.write_u8(self.marker)?;
        writer.write_u8(self.value)?;
        Ok(())
    }
}

impl<Ctx> Decode<Ctx> for CustomErrorRecord {
    type Error = CustomCodecError;

    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<Self, Self::Error> {
        let marker = reader.read_u8()?;
        if marker != 0xAB {
            return Err(CustomCodecError::InvalidMarker(marker));
        }

        let value = reader.read_u8()?;
        Ok(Self { marker, value })
    }
}

#[test]
fn manual_codec_round_trip_with_custom_error_type() {
    let value = CustomErrorRecord { marker: 0xAB, value: 0x42 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0xAB, 0x42]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CustomErrorRecord = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn manual_codec_decode_returns_domain_error_variant() {
    let bytes = [0xEE, 0x10];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let err = reader.read::<CustomErrorRecord>().unwrap_err();
    assert!(matches!(err, CustomCodecError::InvalidMarker(0xEE)));
}

#[test]
fn manual_codec_decode_converts_core_reader_errors() {
    let bytes = [0xAB];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let err = reader.read::<CustomErrorRecord>().unwrap_err();
    assert!(matches!(err, CustomCodecError::Core(Error::OutOfBounds)));
}
