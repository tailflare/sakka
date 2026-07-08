use sakka::{Decode, Encode, Error};

fn bad_encode_with(_value: &u8) -> Result<(), Error> {
    Ok(())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WrongEncodeWithSignature {
    #[sakka(encode_with = bad_encode_with)]
    value: u8,
}

fn main() {}
