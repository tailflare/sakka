use sakka::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(tag = u8)]
enum VariantCodecEnum {
    #[sakka(codec = ByteCodec)]
    Number(u32),
}

struct ByteCodec;

fn main() {}
