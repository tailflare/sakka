use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadBefore {
    #[sakka(pad_before = 2)]
    a: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadAfter {
    #[sakka(pad_after = 2)]
    b: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadBoth {
    #[sakka(pad_before = 1, pad_after = 1)]
    c: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PadWithAlignment {
    #[sakka(pad_before = 1, align_before = 4)]
    d: u16,
}

fn main() {}
