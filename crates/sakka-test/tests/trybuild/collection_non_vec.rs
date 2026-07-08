use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionOnNonVec {
    #[sakka(collection(prefix = u32))]
    a: u32,
}

fn main() {}
