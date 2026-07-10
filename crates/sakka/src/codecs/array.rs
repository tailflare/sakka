use core::marker::PhantomData;

use super::Codec;
use crate::{ReadCollection, Reader, WriteCollection, Writer};

pub struct ArrayCodec<C>(PhantomData<C>);

impl<C, T, Ctx, const N: usize> Codec<[T; N], Ctx> for ArrayCodec<C>
where
    C: Codec<T, Ctx>,
{
    type Error = C::Error;

    #[inline]
    fn encode(value: &[T; N], writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write_slice_with(value, |w, item| C::encode(item, w))
    }

    #[inline]
    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<[T; N], Self::Error> {
        reader.read_array_with(|r| C::decode(r))
    }
}
