use sakka::{Codec, Decode, Encode, Endian, Error, Reader, Writer};

struct CustomCodec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
struct CustomCodecStruct {
    #[sakka(codec = CustomCodec)]
    value: CustomCodecInnerStruct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CustomCodecInnerStruct {
    value1: u16,
    value2: u16,
}

impl<Ctx> Codec<CustomCodecInnerStruct, Ctx> for CustomCodec {
    type Error = Error;

    fn encode(value: &CustomCodecInnerStruct, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write(&value.value1)?;
        writer.write(&value.value2)?;
        Ok(())
    }

    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<CustomCodecInnerStruct, Self::Error> {
        let value1 = reader.read()?;
        let value2 = reader.read()?;
        Ok(CustomCodecInnerStruct { value1, value2 })
    }
}

#[test]
fn custom_codec_round_trip() {
    let value =
        CustomCodecStruct { value: CustomCodecInnerStruct { value1: 0x1122, value2: 0x3344 } };

    let mut writer = Writer::new(Endian::Little, ());
    writer.write(&value).unwrap();
    let bytes = writer.finish();

    assert_eq!(bytes, vec![0x22, 0x11, 0x44, 0x33]);

    let mut reader = Reader::new(&bytes, Endian::Little, ());
    let decoded: CustomCodecStruct = reader.read().unwrap();

    assert_eq!(decoded, value);
}
