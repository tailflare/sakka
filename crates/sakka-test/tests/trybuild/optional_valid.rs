use sakka::{Decode, Encode};

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

fn main() {}
