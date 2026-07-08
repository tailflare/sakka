use crate::{Error, Writer};

/// A trait for types that can be encoded into a `Writer`.
pub trait Encode<Ctx = ()> {
    /// Encodes a value of this type into the given writer.
    fn encode(&self, w: &mut Writer<Ctx>) -> Result<(), Error>;
}
