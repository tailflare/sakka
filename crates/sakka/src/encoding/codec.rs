use crate::{Error, Reader, Writer};

pub trait Codec<T, Ctx = ()>
where
    Self::Error: From<Error>,
{
    type Error;

    fn encode(value: &T, writer: &mut Writer<Ctx>) -> Result<(), Self::Error>;

    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<T, Self::Error>;
}
