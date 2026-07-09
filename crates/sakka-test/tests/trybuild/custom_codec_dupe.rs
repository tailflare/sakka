use sakka::Encode;

struct CodecA;
struct CodecB;

#[derive(Debug, PartialEq, Encode)]
struct DuplicateCodecAttribute {
    #[sakka(codec = CodecA, codec = CodecB)]
    value: u8,
}

fn main() {}
