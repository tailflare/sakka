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

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicTupleStruct(u32, u16, bool);

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicUnitStruct;

fn main() {}
