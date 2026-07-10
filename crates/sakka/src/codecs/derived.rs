use core::marker::PhantomData;

use crate::{Codec, Decode, Encode, Error, Reader, Writer};

pub struct DerivedCodec<T>(PhantomData<T>);

impl<T, Ctx, E> Codec<T, Ctx> for DerivedCodec<T>
where
    T: Encode<Ctx, Error = E> + Decode<Ctx, Error = E>,
    E: From<Error>,
{
    type Error = E;

    fn encode(value: &T, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        value.encode(writer)
    }

    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<T, Self::Error> {
        T::decode(reader)
    }
}
