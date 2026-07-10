use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionWithPrefix {
    #[sakka(collection(prefix = u32))]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionWithCount {
    #[sakka(collection(count = 10))]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionWithCountExpr {
    #[sakka(collection(count = 5 + 3))]
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultipleCollections {
    #[sakka(collection(prefix = u32))]
    a: Vec<u8>,
    #[sakka(collection(count = 4))]
    b: Vec<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionWithField {
    len: u8,
    #[sakka(collection(field = len))]
    data: Vec<u8>,
}

fn main() {}
