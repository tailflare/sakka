use sakka::{Decode, Encode, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

fn encode_u16<Ctx>(writer: &mut Writer<Ctx>, value: &u16) -> Result<(), Error> {
    writer.write_u16(*value)
}

fn decode_u16<Ctx>(reader: &mut Reader<'_, Ctx>) -> Result<u16, Error> {
    reader.read_u16()
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalBool {
    #[sakka(optional(bool))]
    value: Option<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalEof {
    #[sakka(optional(eof))]
    value: Option<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalBoolEq {
    #[sakka(optional = bool)]
    value: Option<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalEofEq {
    #[sakka(optional = eof)]
    value: Option<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalBoolCustom {
    #[sakka(optional(bool), encode_with = encode_u16, decode_with = decode_u16)]
    value: Option<u16>,
}

fn main() {}
