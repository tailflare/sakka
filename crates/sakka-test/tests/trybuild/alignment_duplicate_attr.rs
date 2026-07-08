use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicateAlignBefore {
    #[sakka(align_before = 4, align_before = 8)]
    a: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicateAlignAfter {
    #[sakka(align_after = 4, align_after = 8)]
    a: u32,
}

fn main() {}
