use sakka::{Decode, Encode, Endian, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

fn encode_plus_one<Ctx>(writer: &mut Writer<Ctx>, value: &u8) -> Result<(), Error> {
    writer.write_u8(value.wrapping_add(1))
}

fn decode_minus_one<Ctx>(reader: &mut Reader<'_, Ctx>) -> Result<u8, Error> {
    Ok(reader.read_u8()?.wrapping_sub(1))
}

fn decode_plus_ten<Ctx>(reader: &mut Reader<'_, Ctx>) -> Result<u8, Error> {
    Ok(reader.read_u8()?.wrapping_add(10))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CustomEncodeOnly {
    #[sakka(encode_with = encode_plus_one)]
    value: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CustomDecodeOnly {
    #[sakka(decode_with = decode_plus_ten)]
    value: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CustomEncodeDecode {
    #[sakka(encode_with = encode_plus_one, decode_with = decode_minus_one)]
    value: u8,
}

#[test]
fn encode_with_is_used_for_encoding() {
    let value = CustomEncodeOnly { value: 0x2A };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();
    let bytes = writer.finish();

    // encode_with writes value + 1
    assert_eq!(bytes, vec![0x2B]);
}

#[test]
fn decode_with_is_used_for_decoding() {
    let bytes = [0x05u8];

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CustomDecodeOnly = reader.read().unwrap();

    // decode_with reads value and adds 10
    assert_eq!(decoded, CustomDecodeOnly { value: 0x0F });
}

#[test]
fn encode_with_and_decode_with_round_trip() {
    let value = CustomEncodeDecode { value: 0x44 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();
    let bytes = writer.finish();

    // Encoded byte is transformed by encode_with
    assert_eq!(bytes, vec![0x45]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CustomEncodeDecode = reader.read().unwrap();

    assert_eq!(decoded, value);
}
