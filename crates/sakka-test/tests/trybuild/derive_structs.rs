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

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicGenericStruct<T> {
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicGenericWithWhereStruct<T>
where
    T: Copy,
{
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicDoubleGenericStruct<A, B> {
    first: A,
    second: B,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BasicConstGenericStruct<T, const N: usize> {
    values: [T; N],
}

fn main() {}
