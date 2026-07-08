use crate::{Decode, Encode, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

/// A trait for types that can be read from a [Reader] as an `Option`.
pub trait ReadOption<Ctx> {
    /// Reads an `Option` from the reader using the provided function to read the inner value.
    fn read_option_with<T, F>(&mut self, f: F) -> Result<Option<T>, Error>
    where
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>;

    /// Reads an `Option` from the reader using the [Decode] trait to read the inner value.
    #[inline]
    fn read_option<T>(&mut self) -> Result<Option<T>, Error>
    where
        T: Decode<Ctx>,
    {
        self.read_option_with(T::decode)
    }
}

/// A trait for types that can be written to a [Writer] as an `Option`.
pub trait WriteOption<Ctx> {
    /// Writes an `Option` to the writer using the provided function to write the inner value.
    fn write_option_with<T, F>(&mut self, value: &Option<T>, f: F) -> Result<(), Error>
    where
        F: FnMut(&mut Writer<Ctx>, &T) -> Result<(), Error>;

    /// Writes an `Option` to the writer using the [Encode] trait to write the inner value.
    #[inline]
    fn write_option<T>(&mut self, value: &Option<T>) -> Result<(), Error>
    where
        T: Encode<Ctx>,
    {
        self.write_option_with(value, |w, v| v.encode(w))
    }
}

impl<'a, Ctx> ReadOption<Ctx> for Reader<'a, Ctx> {
    fn read_option_with<T, F>(&mut self, mut f: F) -> Result<Option<T>, Error>
    where
        F: FnMut(&mut Reader<'_, Ctx>) -> Result<T, Error>,
    {
        let is_some = self.read_bool_byte()?;
        if is_some {
            let value = f(self)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<Ctx> WriteOption<Ctx> for Writer<Ctx> {
    fn write_option_with<T, F>(&mut self, value: &Option<T>, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut Writer<Ctx>, &T) -> Result<(), Error>,
    {
        match value {
            Some(v) => {
                self.write_bool_byte(true)?;
                f(self, v)
            }
            None => {
                self.write_bool_byte(false)?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::{ReadOption, WriteOption};
    use crate::{Endian, ReadPrimitive, Reader, WritePrimitive, Writer};

    #[test]
    fn write_and_read_some_variant() {
        let mut writer = Writer::new(Endian::Little, ());
        writer.write_option_with(&Some(42u32), |w, v| w.write_u32(*v)).unwrap();

        let bytes = writer.finish();
        assert_eq!(bytes, vec![1, 42, 0, 0, 0]);

        let mut reader = Reader::new(&bytes, Endian::Little, ());
        let value = reader.read_option_with(|r| r.read_u32()).unwrap();

        assert_eq!(value, Some(42));
    }

    #[test]
    fn write_and_read_none_variant() {
        let mut writer = Writer::new(Endian::Little, ());
        let none_val: Option<u32> = None;
        writer.write_option_with(&none_val, |w, v| w.write_u32(*v)).unwrap();

        let bytes = writer.finish();
        assert_eq!(bytes, vec![0]);

        let mut reader = Reader::new(&bytes, Endian::Little, ());
        let value = reader.read_option_with(|r| r.read_u32()).unwrap();

        assert_eq!(value, None);
    }

    #[test]
    fn write_option_generic_trait_method() {
        let mut writer = Writer::new(Endian::Big, ());
        writer.write_option(&Some(0xDEADBEEFu32)).unwrap();
        writer.write_option::<u32>(&None).unwrap();

        let bytes = writer.finish();
        assert_eq!(bytes[0], 1); // Some marker
        assert_eq!(bytes[1..5], [0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(bytes[5], 0); // None marker
    }

    #[test]
    fn read_option_generic_trait_method() {
        let bytes = [1u8, 0xDE, 0xAD, 0xBE, 0xEF, 0];
        let mut reader = Reader::new(&bytes, Endian::Big, ());

        let some_val = reader.read_option::<u32>().unwrap();
        assert_eq!(some_val, Some(0xDEADBEEF));

        let none_val = reader.read_option::<u32>().unwrap();
        assert_eq!(none_val, None);
    }

    #[test]
    fn roundtrip_nested_options() {
        let mut writer = Writer::new(Endian::Little, ());
        writer.write_option(&Some(true)).unwrap();
        writer.write_option::<bool>(&None).unwrap();
        writer.write_option(&Some(false)).unwrap();

        let bytes = writer.finish();
        let mut reader = Reader::new(&bytes, Endian::Little, ());

        let a = reader.read_option::<bool>().unwrap();
        let b = reader.read_option::<bool>().unwrap();
        let c = reader.read_option::<bool>().unwrap();

        assert_eq!(a, Some(true));
        assert_eq!(b, None);
        assert_eq!(c, Some(false));
        assert!(reader.is_eof());
    }
}
