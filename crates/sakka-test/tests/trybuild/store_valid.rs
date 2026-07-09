use sakka::{Decode, Encode};

#[derive(Default)]
struct Ctx {
    version: u8,
    count: u16,
}

#[derive(Encode, Decode)]
#[sakka(context = Ctx)]
struct StoreValid {
    #[sakka(store = version)]
    version: u8,
    #[sakka(store = count)]
    count: u16,
}

fn main() {}
