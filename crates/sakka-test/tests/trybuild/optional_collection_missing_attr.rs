use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalCollectionMissingAttr {
    #[sakka(optional(bool))]
    data: Option<Vec<u8>>,
}

fn main() {}
