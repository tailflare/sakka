use sakka::Decode;

#[derive(Debug, PartialEq, Decode)]
struct IgnoreDefault {
    first: u8,
    #[sakka(ignore)]
    ignored: u16,
}

#[derive(Debug, PartialEq, Decode)]
struct IgnoreValue {
    first: u8,
    #[sakka(ignore = 0xCAFEu16)]
    ignored: u16,
}

fn main() {}
