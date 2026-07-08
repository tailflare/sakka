use alloc::vec::Vec;
use core::convert::TryInto;

use crate::{Decode, Encode, Error, Reader, Writer, common::array};

/// A trait for types that can represent the length of a collection.
pub trait CollectionLength {
    /// Converts the collection length to a `usize`.
    fn to_usize(self) -> Result<usize, Error>;

    /// Tries to create a collection length from a `usize`.
    fn try_from_usize(len: usize) -> Result<Self, Error>
    where
        Self: Sized;
}

/// A trait for reading collections of data from a reader.
pub trait ReadCollection<Ctx> {
    /// Reads a vector of elements from the reader using the provided function.
    fn read_vec_with<T, F>(&mut self, len: usize, f: F) -> Result<Vec<T>, Error>
    where
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>;

    /// Reads a vector of elements from the reader using the [Decode] trait.
    #[inline]
    fn read_vec<T>(&mut self, len: usize) -> Result<Vec<T>, Error>
    where
        T: Decode<Ctx>,
    {
        self.read_vec_with(len, T::decode)
    }

    /// Reads an array of elements from the reader using the provided function.
    fn read_array_with<T, const N: usize, F>(&mut self, f: F) -> Result<[T; N], Error>
    where
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>;

    /// Reads an array of elements from the reader using the [Decode] trait.
    #[inline]
    fn read_array<T, const N: usize>(&mut self) -> Result<[T; N], Error>
    where
        T: Decode<Ctx>,
    {
        self.read_array_with(T::decode)
    }

    /// Reads a vector of elements from the reader with a length prefix, using the provided
    /// function to read each element.
    fn read_prefixed_vec_with<T, L, F>(&mut self, f: F) -> Result<Vec<T>, Error>
    where
        L: Decode<Ctx> + CollectionLength,
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>;

    /// Reads a vector of elements from the reader with a length prefix, using the [Decode] trait to read each element.
    #[inline]
    fn read_prefixed_vec<T, L>(&mut self) -> Result<Vec<T>, Error>
    where
        T: Decode<Ctx>,
        L: Decode<Ctx> + CollectionLength,
    {
        self.read_prefixed_vec_with::<T, L, _>(T::decode)
    }
}

/// A trait for writing collections of data to a writer.
pub trait WriteCollection<Ctx> {
    /// Writes a slice of elements to the writer using the provided function.
    fn write_slice_with<T, F>(&mut self, slice: &[T], f: F) -> Result<(), Error>
    where
        F: FnMut(&mut Writer<Ctx>, &T) -> Result<(), Error>;

    /// Writes a slice of elements to the writer using the [Encode] trait.
    #[inline]
    fn write_slice<T>(&mut self, slice: &[T]) -> Result<(), Error>
    where
        T: Encode<Ctx>,
    {
        self.write_slice_with(slice, |writer, value| T::encode(value, writer))
    }

    /// Writes a slice of elements to the writer with a length prefix, using the provided function
    /// to write each element.
    fn write_prefixed_slice_with<T, L, F>(&mut self, slice: &[T], f: F) -> Result<(), Error>
    where
        L: Encode<Ctx> + CollectionLength,
        F: FnMut(&mut Writer<Ctx>, &T) -> Result<(), Error>;

    /// Writes a slice of elements to the writer with a length prefix, using the [Encode] trait to
    /// write each element.
    #[inline]
    fn write_prefixed_slice<T, L>(&mut self, slice: &[T]) -> Result<(), Error>
    where
        T: Encode<Ctx>,
        L: Encode<Ctx> + CollectionLength,
    {
        self.write_prefixed_slice_with::<T, L, _>(slice, |writer, value| T::encode(value, writer))
    }
}

impl<'a, Ctx> ReadCollection<Ctx> for Reader<'a, Ctx> {
    fn read_vec_with<T, F>(&mut self, len: usize, mut f: F) -> Result<Vec<T>, Error>
    where
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>,
    {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(f(self)?);
        }
        Ok(vec)
    }

    #[inline]
    fn read_array_with<T, const N: usize, F>(&mut self, mut f: F) -> Result<[T; N], Error>
    where
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>,
    {
        array::try_array_from_fn(|_| f(self))
    }

    fn read_prefixed_vec_with<T, L, F>(&mut self, f: F) -> Result<Vec<T>, Error>
    where
        L: Decode<Ctx> + CollectionLength,
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>,
    {
        let len: L = self.read()?;
        let len_usize = len.to_usize()?;
        self.read_vec_with(len_usize, f)
    }
}

impl<Ctx> WriteCollection<Ctx> for Writer<Ctx> {
    fn write_slice_with<T, F>(&mut self, slice: &[T], mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut Writer<Ctx>, &T) -> Result<(), Error>,
    {
        for value in slice {
            f(self, value)?;
        }
        Ok(())
    }

    fn write_prefixed_slice_with<T, L, F>(&mut self, slice: &[T], f: F) -> Result<(), Error>
    where
        L: Encode<Ctx> + CollectionLength,
        F: FnMut(&mut Writer<Ctx>, &T) -> Result<(), Error>,
    {
        let len = slice.len();
        let len_prefix = L::try_from_usize(len)?;
        self.write(&len_prefix)?;
        self.write_slice_with(slice, f)
    }
}

/// A macro to implement the `CollectionLength` trait for primitive integer types.
/// Signed types will return an error if the value is negative, and all types will return an error
/// if the value cannot be converted to `usize`.
macro_rules! impl_collection_length_for_primitives {
    (
        unsigned: $($unsigned_ty:ty),* $(,)?;
        signed: $($signed_ty:ty),* $(,)?;
    ) => {
        $(impl_collection_length_for_primitives!(@impl unsigned, $unsigned_ty);)*
        $(impl_collection_length_for_primitives!(@impl signed, $signed_ty);)*
    };

    (@impl $kind:ident, $ty:ty) => {
        impl CollectionLength for $ty {
            #[inline]
            fn to_usize(self) -> Result<usize, Error> {
                impl_collection_length_for_primitives!(@validate $kind self);
                self.try_into()
                    .map_err(|_| Error::CollectionLengthOverflow)
            }

            #[inline]
            fn try_from_usize(len: usize) -> Result<Self, Error> {
                len.try_into()
                    .map_err(|_| Error::CollectionLengthOverflow)
            }
        }
    };

    (@validate unsigned $value:expr) => {};

    (@validate signed $value:expr) => {
        if $value < 0 {
            return Err(Error::CollectionLengthNegative);
        }
    };
}

// Implement `CollectionLength` for primitive integer types.
impl_collection_length_for_primitives!(
    unsigned: u8, u16, u32, u64, u128, usize;
    signed: i8, i16, i32, i64, i128, isize;
);

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::{CollectionLength, ReadCollection, WriteCollection};
    use crate::{Endian, Error, ReadPrimitive, Reader, Writer};

    #[test]
    fn read_vec_and_array_with_work() {
        let mut reader = Reader::new(&[1, 2, 3, 4, 5], Endian::Little, ());

        let vec = reader.read_vec_with(3, |r| r.read_u8()).unwrap();
        assert_eq!(vec, vec![1, 2, 3]);

        let arr = reader.read_array_with::<_, 2, _>(|r| r.read_u8()).unwrap();
        assert_eq!(arr, [4, 5]);
        assert!(reader.is_eof());
    }

    #[test]
    fn read_prefixed_vec_reads_prefix_and_elements() {
        let mut reader = Reader::new(&[3, 10, 11, 12], Endian::Little, ());
        let values = reader.read_prefixed_vec::<u8, u8>().unwrap();

        assert_eq!(values, vec![10, 11, 12]);
        assert!(reader.is_eof());
    }

    #[test]
    fn read_prefixed_vec_rejects_negative_signed_length() {
        let mut reader = Reader::new(&[0xFF], Endian::Little, ());
        let err = reader.read_prefixed_vec::<u8, i8>();

        assert!(matches!(err, Err(Error::CollectionLengthNegative)));
    }

    #[test]
    fn write_slice_and_prefixed_slice_work() {
        let mut writer = Writer::new(Endian::Little, ());

        writer.write_slice(&[1u8, 2, 3]).unwrap();
        writer.write_prefixed_slice::<_, u8>(&[9u8, 8]).unwrap();

        assert_eq!(writer.finish(), vec![1, 2, 3, 2, 9, 8]);
    }

    #[test]
    fn write_prefixed_slice_reports_length_overflow() {
        let mut writer = Writer::new(Endian::Little, ());
        let data = vec![0u8; 300];

        let err = writer.write_prefixed_slice::<_, u8>(&data);
        assert!(matches!(err, Err(Error::CollectionLengthOverflow)));
    }

    #[test]
    fn collection_length_conversions_handle_boundaries() {
        assert_eq!(u8::to_usize(10).unwrap(), 10);
        assert!(matches!(u8::try_from_usize(300), Err(Error::CollectionLengthOverflow)));

        assert_eq!(i8::to_usize(7).unwrap(), 7);
        assert!(matches!(i8::to_usize(-1), Err(Error::CollectionLengthNegative)));
    }
}
