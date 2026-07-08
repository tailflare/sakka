use sakka::{Decode, Encode};

#[derive(Debug, PartialEq)]
struct BasicNotDerivedStruct {
    magic: u32,
    version: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicWithNestedStruct {
    magic: u32,
    version: u16,
    nested: BasicNotDerivedStruct,
}

fn main() {}
