use core::marker::PhantomData;

use super::Codec;
use crate::{ReadOption, Reader, WriteOption, Writer};

pub struct OptionBoolCodec<C>(PhantomData<C>);

impl<C, T, Ctx> Codec<Option<T>, Ctx> for OptionBoolCodec<C>
where
    C: Codec<T, Ctx>,
{
    type Error = C::Error;

    #[inline]
    fn encode(value: &Option<T>, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write_option_with(value, |w, item| C::encode(item, w))
    }

    #[inline]
    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<Option<T>, Self::Error> {
        reader.read_option_with(|r| C::decode(r))
    }
}

pub struct OptionEofCodec<C>(PhantomData<C>);

impl<C, T, Ctx> Codec<Option<T>, Ctx> for OptionEofCodec<C>
where
    C: Codec<T, Ctx>,
{
    type Error = C::Error;

    #[inline]
    fn encode(value: &Option<T>, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        if let Some(item) = value {
            C::encode(item, writer)?;
        }

        Ok(())
    }

    #[inline]
    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<Option<T>, Self::Error> {
        if reader.is_eof() { Ok(None) } else { Ok(Some(C::decode(reader)?)) }
    }
}
