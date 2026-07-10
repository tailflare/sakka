use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionFieldNotBefore {
    #[sakka(collection(field = len))]
    data: Vec<u8>,
    len: u8,
}

fn main() {}
