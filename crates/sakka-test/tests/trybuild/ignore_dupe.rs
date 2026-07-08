use sakka::Decode;

#[derive(Debug, PartialEq, Decode)]
struct DuplicateIgnore {
    #[sakka(ignore, ignore)]
    value: u8,
}

#[derive(Debug, PartialEq, Decode)]
struct DuplicateIgnoreValue {
    #[sakka(ignore = 1, ignore = 2)]
    value: u8,
}

fn main() {}
