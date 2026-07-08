use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicatePadBefore {
    #[sakka(pad_before = 1, pad_before = 2)]
    a: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicatePadAfter {
    #[sakka(pad_after = 1, pad_after = 2)]
    a: u8,
}

fn main() {}
