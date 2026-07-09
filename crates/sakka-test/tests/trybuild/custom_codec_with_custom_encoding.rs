use sakka::{Codec, Decode, Encode, Error, Reader, Writer};

struct U8Codec;

impl<Ctx> Codec<u8, Ctx> for U8Codec {
    type Error = Error;

    fn encode(value: &u8, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write(value)
    }

    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<u8, Self::Error> {
        reader.read()
    }
}

fn encode_u8<Ctx>(writer: &mut Writer<Ctx>, value: &u8) -> Result<(), Error> {
    writer.write(value)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CodecWithEncodeWith {
    #[sakka(codec = U8Codec, encode_with = encode_u8)]
    value: u8,
}

fn main() {}
