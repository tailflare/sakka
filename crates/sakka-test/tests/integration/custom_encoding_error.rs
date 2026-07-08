use sakka::{Decode, Encode, Endian, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

#[derive(Debug)]
enum CustomEncodingError {
    Core(Error),
    ValueTooLarge(u8),
    InvalidEncodedValue(u8),
}

impl From<Error> for CustomEncodingError {
    fn from(value: Error) -> Self {
        Self::Core(value)
    }
}

fn encode_checked_u8<Ctx>(writer: &mut Writer<Ctx>, value: &u8) -> Result<(), CustomEncodingError> {
    if *value > 0x7F {
        return Err(CustomEncodingError::ValueTooLarge(*value));
    }

    writer.write_u8(value.wrapping_add(1))?;
    Ok(())
}

fn decode_checked_u8<Ctx>(reader: &mut Reader<'_, Ctx>) -> Result<u8, CustomEncodingError> {
    let encoded = reader.read_u8()?;
    if encoded == 0 {
        return Err(CustomEncodingError::InvalidEncodedValue(encoded));
    }

    Ok(encoded.wrapping_sub(1))
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(error = CustomEncodingError)]
struct CustomEncodingWithError {
    #[sakka(encode_with = encode_checked_u8, decode_with = decode_checked_u8)]
    value: u8,
}

#[test]
fn custom_struct_error_and_custom_encoding_round_trip() {
    let value = CustomEncodingWithError { value: 0x2A };

    let mut writer = Writer::new(Endian::Little, ());
    let write_result: Result<(), CustomEncodingError> = writer.write(&value);
    assert!(write_result.is_ok());

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0x2B]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: Result<CustomEncodingWithError, CustomEncodingError> = reader.read();

    assert_eq!(decoded.unwrap(), value);
}

#[test]
fn custom_encode_with_returns_custom_error() {
    let value = CustomEncodingWithError { value: 0xFE };
    let mut writer = Writer::new(Endian::Little, ());

    let err = writer.write(&value).unwrap_err();
    assert!(matches!(err, CustomEncodingError::ValueTooLarge(0xFE)));
}

#[test]
fn custom_decode_with_returns_custom_error() {
    let bytes = [0x00u8];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let err = reader.read::<CustomEncodingWithError>().unwrap_err();
    assert!(matches!(err, CustomEncodingError::InvalidEncodedValue(0x00)));
}

#[test]
fn custom_decode_with_converts_core_error() {
    let bytes: [u8; 0] = [];
    let mut reader = Reader::new(&bytes, Endian::Little, ());

    let err = reader.read::<CustomEncodingWithError>().unwrap_err();
    assert!(matches!(err, CustomEncodingError::Core(Error::OutOfBounds)));
}
