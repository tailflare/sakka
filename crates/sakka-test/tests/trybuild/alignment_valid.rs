use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignBefore {
    a: u8,
    #[sakka(align_before = 4)]
    b: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignAfter {
    a: u32,
    #[sakka(align_after = 4)]
    b: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignBoth {
    a: u8,
    #[sakka(align_before = 4, align_after = 8)]
    b: u32,
}

const ALIGN_VALUE: usize = 16;

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlignWithConst {
    #[sakka(align_before = ALIGN_VALUE)]
    a: u32,
}

fn main() {}
