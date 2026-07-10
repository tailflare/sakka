use sakka::{Decode, Encode};

#[derive(Default)]
struct Ctx {
    version: u8,
    count: u16,
    size: usize,
}

#[derive(Encode, Decode)]
#[sakka(context = Ctx)]
struct StoreValid {
    #[sakka(store(version = value))]
    version: u8,
    #[sakka(store(count = value.len() as u16))]
    payload: [u8; 3],
    #[sakka(store(size = value.len()))]
    trailer: [u8; 2],
}

#[derive(Encode, Decode)]
#[sakka(context = Ctx)]
struct StoreValidLegacy {
    #[sakka(store = version)]
    version: u8,
    #[sakka(store = count)]
    count: u16,
}

fn main() {}
