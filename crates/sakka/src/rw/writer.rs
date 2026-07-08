use alloc::vec::Vec;

use crate::{Encode, Endian, Error, common::alignment};

pub struct Writer<Ctx = ()> {
    inner: Vec<u8>,
    endian: Endian,
    context: Ctx,
}

impl<Ctx> Writer<Ctx> {
    /// Creates a new writer with the given endianness and context.
    #[inline]
    pub const fn new(endian: Endian, context: Ctx) -> Self {
        Self { inner: Vec::new(), endian, context }
    }

    /// Returns an immutable reference to the writer context.
    #[inline]
    pub fn context(&self) -> &Ctx {
        &self.context
    }

    /// Returns a mutable reference to the writer context.
    #[inline]
    pub fn context_mut(&mut self) -> &mut Ctx {
        &mut self.context
    }

    /// Replaces the context and returns the previous value.
    #[inline]
    pub fn replace_context(&mut self, context: Ctx) -> Ctx {
        core::mem::replace(&mut self.context, context)
    }

    /// Returns the endianness of the writer.
    #[inline]
    pub const fn endian(&self) -> Endian {
        self.endian
    }

    /// Returns the current length of the writer.
    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the writer is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a slice of the accumulated bytes.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    /// Consumes the writer and returns the accumulated bytes.
    #[inline]
    pub fn finish(self) -> Vec<u8> {
        self.inner
    }

    /// Reserves capacity for at least `additional` more bytes to be written to the writer.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Reserves capacity for at least `size_of::<T>()` more bytes to be written to the writer.
    #[inline]
    pub fn reserve_for<T: Sized>(&mut self) {
        self.reserve(core::mem::size_of::<T>());
    }

    /// Writes `n` zero bytes to the writer.
    #[inline]
    pub fn write_zeroes(&mut self, n: usize) -> Result<(), Error> {
        self.inner.resize(self.inner.len() + n, 0);
        Ok(())
    }

    /// Aligns the writer's position to the given alignment, returning an error if the alignment
    /// is invalid.
    #[inline]
    pub fn align(&mut self, alignment: usize) -> Result<(), Error> {
        let padding = alignment::alignment_padding(self.len(), alignment)?;
        self.write_zeroes(padding)
    }

    /// Writes a slice of bytes to the writer.
    #[inline]
    pub fn write_bytes(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.inner.extend_from_slice(buf);
        Ok(())
    }

    /// Writes a value of type `T` to the writer using the [Encode] trait.
    #[inline]
    pub fn write<T: Encode<Ctx>>(&mut self, value: &T) -> Result<(), T::Error> {
        value.encode(self)
    }
}

impl<Ctx: Default> Default for Writer<Ctx> {
    fn default() -> Self {
        Self::new(Endian::Little, Default::default())
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::Writer;
    use crate::{Endian, Error};

    #[test]
    fn new_starts_empty_with_given_endian_and_context() {
        let writer = Writer::new(Endian::Big, 7u32);

        assert!(writer.is_empty());
        assert_eq!(writer.len(), 0);
        assert_eq!(writer.endian(), Endian::Big);
        assert_eq!(*writer.context(), 7);
    }

    #[test]
    fn default_uses_little_endian_and_default_context() {
        let writer = Writer::<u16>::default();

        assert_eq!(writer.endian(), Endian::Little);
        assert_eq!(*writer.context(), 0);
        assert!(writer.is_empty());
    }

    #[test]
    fn context_mut_and_replace_context_work() {
        let mut writer = Writer::new(Endian::Little, 10i32);

        *writer.context_mut() += 5;
        assert_eq!(*writer.context(), 15);

        let old = writer.replace_context(42);
        assert_eq!(old, 15);
        assert_eq!(*writer.context(), 42);
    }

    #[test]
    fn write_bytes_write_zeroes_and_finish() {
        let mut writer = Writer::new(Endian::Little, ());

        writer.write_bytes(&[1, 2, 3]).unwrap();
        writer.write_zeroes(2).unwrap();
        writer.write_bytes(&[9]).unwrap();

        assert_eq!(writer.len(), 6);
        assert_eq!(writer.as_slice(), &[1, 2, 3, 0, 0, 9]);
        assert_eq!(writer.finish(), vec![1, 2, 3, 0, 0, 9]);
    }

    #[test]
    fn align_adds_expected_padding() {
        let mut writer = Writer::new(Endian::Little, ());

        writer.write_bytes(&[0xAA, 0xBB, 0xCC]).unwrap();
        writer.align(4).unwrap();

        assert_eq!(writer.as_slice(), &[0xAA, 0xBB, 0xCC, 0x00]);

        writer.align(4).unwrap();
        assert_eq!(writer.as_slice(), &[0xAA, 0xBB, 0xCC, 0x00]);
    }

    #[test]
    fn align_rejects_invalid_alignment() {
        let mut writer = Writer::new(Endian::Little, ());

        assert!(matches!(writer.align(0), Err(Error::InvalidAlignment)));
        assert!(matches!(writer.align(3), Err(Error::InvalidAlignment)));
    }

    #[test]
    fn reserve_helpers_do_not_change_length() {
        let mut writer = Writer::new(Endian::Little, ());

        writer.reserve(16);
        writer.reserve_for::<u64>();

        assert_eq!(writer.len(), 0);
        assert!(writer.is_empty());
    }
}
