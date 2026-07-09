use sakka::{Decode, Encode};

#[derive(Encode, Decode)]
#[sakka(context = crate::C1, context = crate::C2)]
struct StructContextDupe {
    value: u8,
}

fn main() {}
