use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicateCollectionAttr {
    #[sakka(collection(prefix = u32), collection(count = 5))]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DuplicateCollectionInner {
    #[sakka(collection(prefix = u32, prefix = u16))]
    data: Vec<u8>,
}

fn main() {}
