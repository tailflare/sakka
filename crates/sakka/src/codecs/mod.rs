mod array;
mod codec;
mod option;
#[cfg(test)]
mod tests;
mod vec;

pub use self::{
    array::ArrayCodec,
    codec::Codec,
    option::{OptionBoolCodec, OptionEofCodec},
    vec::VecPrefixCodec,
};
