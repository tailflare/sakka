use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalBoolCollection {
    #[sakka(optional(bool), collection(prefix = u16))]
    data: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalEofCollection {
    #[sakka(optional(eof), collection(count = 3))]
    data: Option<Vec<u8>>,
}

fn main() {}
