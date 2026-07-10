use alloc::vec::Vec;
use core::marker::PhantomData;

use super::Codec;
use crate::{
    CollectionLength, Decode, Encode, Error, ReadCollection, Reader, WriteCollection, Writer,
};

pub struct VecPrefixCodec<L, C>(PhantomData<(L, C)>);

impl<L, C, T, Ctx> Codec<Vec<T>, Ctx> for VecPrefixCodec<L, C>
where
    L: Decode<Ctx, Error = Error> + Encode<Ctx, Error = Error> + CollectionLength,
    C: Codec<T, Ctx>,
{
    type Error = C::Error;

    #[inline]
    fn encode(value: &Vec<T>, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write_prefixed_slice_with::<_, L, _, _>(value, |w, item| C::encode(item, w))
    }

    #[inline]
    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<Vec<T>, Self::Error> {
        let len: L = reader.read()?;
        let len_usize = len.to_usize()?;
        reader.read_vec_with(len_usize, |r| C::decode(r))
    }
}
