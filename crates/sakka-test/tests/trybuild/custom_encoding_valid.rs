use sakka::{Decode, Encode, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

fn encode_u16<Ctx>(writer: &mut Writer<Ctx>, value: &u16) -> Result<(), Error> {
    writer.write_u16(*value)
}

fn decode_u16<Ctx>(reader: &mut Reader<'_, Ctx>) -> Result<u16, Error> {
    reader.read_u16()
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CustomEncodingValid {
    #[sakka(encode_with = encode_u16, decode_with = decode_u16)]
    value: u16,
}

fn main() {}
