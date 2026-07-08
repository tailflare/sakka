use sakka::{Decode, Encode};

#[derive(Encode, Decode)]
#[sakka(error = crate::E1, error = crate::E2)]
struct StructErrorDupe {
    value: u8,
}

fn main() {}
