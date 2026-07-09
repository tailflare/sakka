use sakka::Decode;

#[derive(Debug, PartialEq, Decode)]
struct DuplicateStore {
    #[sakka(store = version, store = version)]
    value: u8,
}

fn main() {}
