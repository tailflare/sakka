use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct VecMissingCollection {
    data: Vec<u8>,
}

fn main() {}
