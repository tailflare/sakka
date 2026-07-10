use alloc::vec::Vec;
use core::marker::PhantomData;

use super::Codec;
use crate::{
    CollectionLength, Decode, Encode, Error, ReadCollection, Reader, WriteCollection, Writer,
};

pub struct VecPrefixCodec<L, C>(PhantomData<(L, C)>);

impl<C, T, L, Ctx> Codec<Vec<T>, Ctx> for VecPrefixCodec<L, C>
where
    C: Codec<T, Ctx>,
    L: Decode<Ctx, Error = Error> + Encode<Ctx, Error = Error> + CollectionLength,
{
    type Error = C::Error;

    #[inline]
    fn encode(value: &Vec<T>, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write_prefixed_slice_with::<_, L, _, _>(value, |w, item| C::encode(item, w))
    }

    #[inline]
    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<Vec<T>, Self::Error> {
        reader.read_prefixed_vec_with::<_, L, _, _>(|r| C::decode(r))
    }
}
