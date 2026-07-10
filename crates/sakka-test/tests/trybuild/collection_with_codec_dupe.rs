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

#[derive(Encode, Decode)]
struct CollectionWithCodecDupe {
    #[sakka(codec = U8Codec, collection(count = 2))]
    value: u8,
}

fn main() {}
