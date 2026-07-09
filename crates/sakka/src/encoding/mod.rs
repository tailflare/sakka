mod codec;
mod collection;
#[allow(clippy::module_inception)]
mod decode;
mod encode;
mod primitive;
#[cfg(test)]
mod tests;

pub use self::{codec::Codec, decode::Decode, encode::Encode};
