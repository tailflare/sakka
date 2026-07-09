use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalOnNonOption {
    #[sakka(optional(bool))]
    value: u16,
}

fn main() {}
