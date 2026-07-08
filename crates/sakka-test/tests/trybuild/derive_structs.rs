use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicStruct {
    magic: u32,
    version: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicWithNestedStruct {
    magic: u32,
    version: u16,
    nested: BasicStruct,
}

fn main() {}
