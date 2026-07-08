use sakka::{Decode, Encode, Error, Reader, Writer};

fn encode_u8<Ctx>(_writer: &mut Writer<Ctx>, _value: &u8) -> Result<(), Error> {
    Ok(())
}

fn decode_u8<Ctx>(_reader: &mut Reader<'_, Ctx>) -> Result<u8, Error> {
    Ok(0)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IgnoreWithEncodeWith {
    #[sakka(ignore, encode_with = encode_u8)]
    value: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IgnoreWithDecodeWith {
    #[sakka(ignore, decode_with = decode_u8)]
    value: u8,
}

fn main() {}
