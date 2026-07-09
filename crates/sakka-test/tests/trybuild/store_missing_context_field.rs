use sakka::{Decode, Encode};

#[derive(Default)]
struct Ctx {
    present: u8,
}

#[derive(Encode, Decode)]
#[sakka(context = Ctx)]
struct StoreMissingContextField {
    #[sakka(store = missing)]
    value: u8,
}

fn main() {}
