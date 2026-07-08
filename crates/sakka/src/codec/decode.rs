use crate::{Error, Reader};

/// A trait for types that can be decoded from a `Reader`.
pub trait Decode<Ctx = ()>: Sized
where
    Self::Error: From<Error>,
{
    type Error;

    /// Decodes a value of this type from the given reader.
    fn decode(r: &mut Reader<'_, Ctx>) -> Result<Self, Self::Error>;
}
