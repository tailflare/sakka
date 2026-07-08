use sakka::{Decode, Encode, Error, Reader, Writer};

fn encode_1<Ctx>(_writer: &mut Writer<Ctx>, _value: &u8) -> Result<(), Error> {
    Ok(())
}

fn encode_2<Ctx>(_writer: &mut Writer<Ctx>, _value: &u8) -> Result<(), Error> {
    Ok(())
}

fn decode_1<Ctx>(_reader: &mut Reader<'_, Ctx>) -> Result<u8, Error> {
    Ok(0)
}

fn decode_2<Ctx>(_reader: &mut Reader<'_, Ctx>) -> Result<u8, Error> {
    Ok(0)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicateEncodeWith {
    #[sakka(encode_with = encode_1, encode_with = encode_2)]
    value: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicateDecodeWith {
    #[sakka(decode_with = decode_1, decode_with = decode_2)]
    value: u8,
}

fn main() {}
