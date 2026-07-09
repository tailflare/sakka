use sakka::{Codec, Decode, Encode, Error, Reader, Writer};

struct U16Codec;

impl<Ctx> Codec<u16, Ctx> for U16Codec {
    type Error = Error;

    fn encode(value: &u16, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write(value)
    }

    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<u16, Self::Error> {
        reader.read()
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CustomCodecValid {
    #[sakka(codec = U16Codec)]
    value: u16,
}

fn main() {}
