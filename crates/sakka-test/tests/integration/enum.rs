use sakka::{Decode, Encode, Endian, Error, Reader, Writer};

struct ShiftedU32Codec;

impl<Ctx> sakka::Codec<u32, Ctx> for ShiftedU32Codec {
    type Error = Error;

    fn encode(value: &u32, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write(&value.wrapping_add(1))
    }

    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<u32, Self::Error> {
        Ok(reader.read::<u32>()?.wrapping_sub(1))
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(tag = u8)]
#[repr(u8)]
enum NamedCodecEnum {
    Config {
        #[sakka(codec = ShiftedU32Codec)]
        slot: u32,
    } = 0,
    Other(u8) = 1,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BasicEnum {
    Unit,
    Tuple(u16, bool),
    Named { value: u8, flag: bool },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ExplicitEnum {
    A = 10,
    B,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(tag = u16)]
enum WideTagEnum {
    Small,
    Big = 300,
}

#[test]
fn round_trip_unit_variant() {
    let value = BasicEnum::Unit;

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicEnum = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_tuple_variant() {
    let value = BasicEnum::Tuple(0xBEEF, true);

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![1, 0xEF, 0xBE, 1]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicEnum = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn round_trip_named_variant() {
    let value = BasicEnum::Named { value: 7, flag: false };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![2, 7, 0]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: BasicEnum = reader.read().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn explicit_discriminants_are_used() {
    let explicit = ExplicitEnum::A;
    let after = ExplicitEnum::B;

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&explicit).unwrap();
    writer.write(&after).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![10, 11]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded_explicit: ExplicitEnum = reader.read().unwrap();
    let decoded_after: ExplicitEnum = reader.read().unwrap();

    assert_eq!(decoded_explicit, explicit);
    assert_eq!(decoded_after, after);
}

#[test]
fn invalid_discriminant_errors() {
    let mut reader = Reader::new(&[250], Endian::Little, ());
    let err = reader.read::<BasicEnum>().unwrap_err();

    assert!(matches!(err, Error::InvalidEnumDiscriminant));
}

#[test]
fn wide_enum_tag_writes_and_reads_u16_discriminants() {
    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&WideTagEnum::Small).unwrap();
    writer.write(&WideTagEnum::Big).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0x00, 0x00, 0x2C, 0x01]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let small: WideTagEnum = reader.read().unwrap();
    let big: WideTagEnum = reader.read().unwrap();

    assert_eq!(small, WideTagEnum::Small);
    assert_eq!(big, WideTagEnum::Big);
}

#[test]
fn named_variant_field_codec_controls_payload_encoding() {
    let value = NamedCodecEnum::Config { slot: 0x1234_5678 };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();

    let bytes = writer.finish();
    assert_eq!(bytes, vec![0, 0x79, 0x56, 0x34, 0x12]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: NamedCodecEnum = reader.read().unwrap();

    assert_eq!(decoded, value);
}
