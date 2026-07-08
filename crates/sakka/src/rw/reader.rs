use crate::{Decode, Endian, Error, common::alignment};

/// A reader for reading binary data from a byte slice.
pub struct Reader<'a, Ctx = ()> {
    bytes: &'a [u8],
    position: usize,
    endian: Endian,
    context: Ctx,
}

impl<'a, Ctx> Reader<'a, Ctx> {
    /// Creates a new reader with the given data and context.
    #[inline]
    pub const fn new(data: &'a [u8], endian: Endian, context: Ctx) -> Self {
        Self { bytes: data, position: 0, endian, context }
    }

    /// Returns a reference to the underlying context.
    #[inline]
    pub fn context(&self) -> &Ctx {
        &self.context
    }

    /// Returns a mutable reference to the underlying context.
    #[inline]
    pub fn context_mut(&mut self) -> &mut Ctx {
        &mut self.context
    }

    /// Replaces the underlying context with the given one, returning the old context.
    #[inline]
    pub fn replace_context(&mut self, context: Ctx) -> Ctx {
        core::mem::replace(&mut self.context, context)
    }

    /// Returns the endianness of the reader.
    #[inline]
    pub const fn endian(&self) -> Endian {
        self.endian
    }

    /// Returns the total length of the underlying data.
    #[inline]
    pub const fn total_len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns the current position of the reader.
    #[inline]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns the number of bytes remaining to be read.
    #[inline]
    pub const fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    /// Returns `true` if the reader has reached the end of the underlying data.
    #[inline]
    pub const fn is_eof(&self) -> bool {
        self.remaining() == 0
    }

    /// Returns the data as a slice.
    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the remaining data as a slice.
    #[inline]
    pub fn remaining_slice(&self) -> &'a [u8] {
        &self.bytes[self.position..]
    }

    /// Sets the position of the reader, returning an error if the position is out of bounds.
    #[inline]
    pub fn set_position(&mut self, position: usize) -> Result<(), Error> {
        if position > self.bytes.len() {
            return Err(Error::OutOfBounds);
        }

        self.position = position;
        Ok(())
    }

    /// Seeks the reader by the given offset, returning an error if the new position is out of bounds.
    #[inline]
    pub fn seek(&mut self, offset: isize) -> Result<(), Error> {
        let new_position = if offset < 0 {
            self.position.checked_sub(offset.unsigned_abs())
        } else {
            self.position.checked_add(offset as usize)
        };

        match new_position {
            Some(pos) if pos <= self.bytes.len() => {
                self.position = pos;
                Ok(())
            }
            _ => Err(Error::OutOfBounds),
        }
    }

    /// Skips the given number of bytes, returning an error if the end of the underlying data is
    /// reached.
    #[inline]
    pub fn skip(&mut self, len: usize) -> Result<(), Error> {
        let end = self.position.checked_add(len).ok_or(Error::OutOfBounds)?;

        if end > self.bytes.len() {
            return Err(Error::OutOfBounds);
        }

        self.position = end;
        Ok(())
    }

    /// Aligns the reader's position to the given alignment, returning an error if the alignment
    /// is invalid or if the end of the underlying data is reached.
    #[inline]
    pub fn align(&mut self, alignment: usize) -> Result<(), Error> {
        let padding = alignment::alignment_padding(self.position, alignment)?;
        self.skip(padding)
    }

    /// Returns a slice of the given length, returning an error if the end of the underlying data is
    /// reached.
    #[inline]
    pub fn take(&mut self, len: usize) -> Result<&'a [u8], Error> {
        let start = self.position;
        let end = start.checked_add(len).ok_or(Error::OutOfBounds)?;

        if end > self.bytes.len() {
            return Err(Error::OutOfBounds);
        }

        self.position = end;
        Ok(&self.bytes[start..end])
    }

    /// Reads exactly the given number of bytes into the provided buffer.
    #[inline]
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let len = buf.len();
        let bytes = self.take(len)?;
        buf.copy_from_slice(bytes);
        Ok(())
    }

    /// Reads exactly the given number of bytes into a fixed-size array.
    #[inline]
    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut buf = [0u8; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads a value of type `D` from the reader using the `Decode` trait.
    #[inline]
    pub fn read<D: Decode<Ctx>>(&mut self) -> Result<D, Error> {
        D::decode(self)
    }

    /// Peeks at the given number of bytes without advancing the reader's position.
    #[inline]
    pub fn peek(&self, len: usize) -> Result<&'a [u8], Error> {
        let start = self.position;
        let end = start.checked_add(len).ok_or(Error::OutOfBounds)?;

        if end > self.bytes.len() {
            return Err(Error::OutOfBounds);
        }

        Ok(&self.bytes[start..end])
    }

    /// Reads exactly the given number of bytes into the provided buffer without advancing the
    /// reader's position.
    #[inline]
    pub fn peek_exact(&self, buf: &mut [u8]) -> Result<(), Error> {
        let bytes = self.peek(buf.len())?;
        buf.copy_from_slice(bytes);
        Ok(())
    }

    /// Peeks at the given number of bytes and returns them as a fixed-size array.
    #[inline]
    pub fn peek_bytes<const N: usize>(&self) -> Result<[u8; N], Error> {
        let mut buf = [0u8; N];
        self.peek_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<Ctx: Default> Default for Reader<'_, Ctx> {
    fn default() -> Self {
        Self::new(&[], Endian::Little, Ctx::default())
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;
    use crate::{Endian, Error};

    #[test]
    fn new_and_default_have_expected_initial_state() {
        let bytes = [1u8, 2, 3];
        let reader = Reader::new(&bytes, Endian::Big, 9u32);

        assert_eq!(reader.endian(), Endian::Big);
        assert_eq!(reader.total_len(), 3);
        assert_eq!(reader.position(), 0);
        assert_eq!(reader.remaining(), 3);
        assert!(!reader.is_eof());
        assert_eq!(*reader.context(), 9);
        assert_eq!(reader.as_slice(), &bytes);
        assert_eq!(reader.remaining_slice(), &bytes);

        let default_reader = Reader::<u16>::default();
        assert_eq!(default_reader.endian(), Endian::Little);
        assert_eq!(*default_reader.context(), 0);
        assert!(default_reader.is_eof());
    }

    #[test]
    fn context_mut_and_replace_context_work() {
        let mut reader = Reader::new(&[1, 2], Endian::Little, 10i32);

        *reader.context_mut() += 5;
        assert_eq!(*reader.context(), 15);

        let old = reader.replace_context(42);
        assert_eq!(old, 15);
        assert_eq!(*reader.context(), 42);
    }

    #[test]
    fn set_position_seek_and_skip_update_position_and_bounds() {
        let mut reader = Reader::new(&[0, 1, 2, 3, 4], Endian::Little, ());

        reader.set_position(2).unwrap();
        assert_eq!(reader.position(), 2);
        assert_eq!(reader.remaining(), 3);

        reader.seek(2).unwrap();
        assert_eq!(reader.position(), 4);

        reader.seek(-3).unwrap();
        assert_eq!(reader.position(), 1);

        reader.skip(2).unwrap();
        assert_eq!(reader.position(), 3);

        assert!(matches!(reader.set_position(6), Err(Error::OutOfBounds)));
        assert!(matches!(reader.seek(-10), Err(Error::OutOfBounds)));
        assert!(matches!(reader.seek(10), Err(Error::OutOfBounds)));
        assert!(matches!(reader.skip(3), Err(Error::OutOfBounds)));
    }

    #[test]
    fn align_obeys_padding_and_errors() {
        let mut reader = Reader::new(&[0, 1, 2, 3], Endian::Little, ());

        reader.set_position(1).unwrap();
        reader.align(4).unwrap();
        assert_eq!(reader.position(), 4);
        assert!(reader.is_eof());

        let mut invalid = Reader::new(&[0, 1], Endian::Little, ());
        assert!(matches!(invalid.align(0), Err(Error::InvalidAlignment)));
        assert!(matches!(invalid.align(3), Err(Error::InvalidAlignment)));

        let mut oob = Reader::new(&[0, 1, 2], Endian::Little, ());
        oob.set_position(2).unwrap();
        assert!(matches!(oob.align(4), Err(Error::OutOfBounds)));
    }

    #[test]
    fn take_read_exact_and_read_bytes_consume_data() {
        let mut reader = Reader::new(&[10, 11, 12, 13], Endian::Little, ());

        assert_eq!(reader.take(2).unwrap(), &[10, 11]);
        assert_eq!(reader.position(), 2);

        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [12]);
        assert_eq!(reader.position(), 3);

        let arr = reader.read_bytes::<1>().unwrap();
        assert_eq!(arr, [13]);
        assert!(reader.is_eof());

        assert!(matches!(reader.take(1), Err(Error::OutOfBounds)));
    }

    #[test]
    fn peek_variants_do_not_advance_position() {
        let mut reader = Reader::new(&[21, 22, 23, 24], Endian::Little, ());
        reader.set_position(1).unwrap();

        assert_eq!(reader.peek(2).unwrap(), &[22, 23]);
        assert_eq!(reader.position(), 1);

        let mut buf = [0u8; 2];
        reader.peek_exact(&mut buf).unwrap();
        assert_eq!(buf, [22, 23]);
        assert_eq!(reader.position(), 1);

        let arr = reader.peek_bytes::<3>().unwrap();
        assert_eq!(arr, [22, 23, 24]);
        assert_eq!(reader.position(), 1);

        assert!(matches!(reader.peek(5), Err(Error::OutOfBounds)));
    }
}
