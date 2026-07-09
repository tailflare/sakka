use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalDupe {
    #[sakka(optional(bool), optional(eof))]
    value: Option<u16>,
}

fn main() {}
