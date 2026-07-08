// Generates ReadPrimitiveExt methods like read_u32/read_f64 from type lists.

macro_rules! impl_rw_primitive_exts {
    (
        endian: $($endian_ty:ident),+ $(,)?;
        raw: $($raw_ty:ident),+ $(,)?;
    ) => {

        /// A trait that provides methods for reading primitive types with configurable endianness
        /// support.
        ///
        /// Primitive values with multi-byte representations are interpreted according to the
        /// configured `Endian` setting of the reader. Most fixed-width primitives have a stable
        /// representation when an explicit endianness is used.
        ///
        /// Note: Platform-dependent types such as `usize` and `isize` use the native
        /// size of the target architecture and are therefore not portable between
        /// different platforms. Similarly, [`Endian::Native`] should only be used when
        /// the serialized data is intended for use on the same architecture.
        pub trait ReadPrimitive{
            $(
                pastey::paste! {
                    #[doc = concat!("Reads a `", stringify!($endian_ty), "` using the reader's configured endianness.")]
                    fn [<read_ $endian_ty>](&mut self) -> Result<$endian_ty, crate::Error>;
                }
            )+

            $(
                pastey::paste! {
                    #[doc = concat!("Reads a raw `", stringify!($raw_ty), "` byte value (endianness is not applied).")]
                    fn [<read_ $raw_ty>](&mut self) -> Result<$raw_ty, crate::Error>;
                }
            )+

            /// Reads a `bool` from the reader, where 0 represents `false` and any non-zero
            /// value represents `true`.
            fn read_bool_byte(&mut self) -> Result<bool, crate::Error>;
        }

        /// A trait that provides methods for writing primitive types with configurable endianness
        /// support.
        ///
        /// Primitive values with multi-byte representations are interpreted according to the
        /// configured `Endian` setting of the writer. Most fixed-width primitives have a stable
        /// representation when an explicit endianness is used.
        ///
        /// Note: Platform-dependent types such as `usize` and `isize` use the native
        /// size of the target architecture and are therefore not portable between
        /// different platforms. Similarly, [`Endian::Native`] should only be used when
        /// the serialized data is intended for use on the same architecture.
        pub trait WritePrimitive {
            $(
                pastey::paste! {
                    #[doc = concat!("Writes a `", stringify!($endian_ty), "` using the writer's configured endianness.")]
                    fn [<write_ $endian_ty>](&mut self, value: $endian_ty) -> Result<(), crate::Error>;
                }
            )+

            $(
                pastey::paste! {
                    #[doc = concat!("Writes a raw `", stringify!($raw_ty), "` byte value (endianness is not applied).")]
                    fn [<write_ $raw_ty>](&mut self, value: $raw_ty) -> Result<(), crate::Error>;
                }
            )+

            /// Writes a `bool` as a single byte to the writer, where 0 represents `false`
            /// and any non-zero value represents `true`.
            fn write_bool_byte(&mut self, value: bool) -> Result<(), crate::Error>;
        }

        impl<'a, Ctx> crate::ReadPrimitive for crate::Reader<'a, Ctx> {
            $(
                pastey::paste! {
                    #[inline]
                    fn [<read_ $endian_ty>](&mut self) -> Result<$endian_ty, crate::Error> {
                        let raw = self.read_bytes::<{ core::mem::size_of::<$endian_ty>() }>()?;

                        let result = match self.endian() {
                            crate::Endian::Native => <$endian_ty>::from_ne_bytes(raw),
                            crate::Endian::Little => <$endian_ty>::from_le_bytes(raw),
                            crate::Endian::Big => <$endian_ty>::from_be_bytes(raw),
                        };

                        Ok(result)
                    }
                }
            )+

            $(
                pastey::paste! {
                    #[inline]
                    fn [<read_ $raw_ty>](&mut self) -> Result<$raw_ty, crate::Error> {
                        let raw = self.read_bytes::<1>()?;
                        Ok(raw[0] as $raw_ty)
                    }
                }
            )+

            #[inline]
            fn read_bool_byte(&mut self) -> Result<bool, crate::Error> {
                let raw = self.read_bytes::<1>()?;
                Ok(raw[0] != 0)
            }
        }

        impl<Ctx> crate::WritePrimitive for crate::Writer<Ctx> {
            $(
                pastey::paste! {
                    #[inline]
                    fn [<write_ $endian_ty>](&mut self, value: $endian_ty) -> Result<(), crate::Error> {
                        let bytes = match self.endian() {
                            crate::Endian::Native => value.to_ne_bytes(),
                            crate::Endian::Little => value.to_le_bytes(),
                            crate::Endian::Big => value.to_be_bytes(),
                        };
                        self.write_bytes(&bytes)
                    }
                }
            )+

            $(
                pastey::paste! {
                    #[inline]
                    fn [<write_ $raw_ty>](&mut self, value: $raw_ty) -> Result<(), crate::Error> {
                        self.write_bytes(&[value as u8])
                    }
                }
            )+

            #[inline]
            fn write_bool_byte(&mut self, value: bool) -> Result<(), crate::Error> {
                self.write_bytes(&[if value { 1 } else { 0 }])
            }
        }
    };
}

// Add primitive types here to generate matching read_<type> and write_<type> methods.
impl_rw_primitive_exts!(
    endian: u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64;
    raw: u8, i8;
);

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::{Endian, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

    #[test]
    fn writes_endian_primitives_with_configured_byte_order() {
        let mut le = Writer::new(Endian::Little, ());
        le.write_u16(0x1234).unwrap();
        assert_eq!(le.finish(), vec![0x34, 0x12]);

        let mut be = Writer::new(Endian::Big, ());
        be.write_u16(0x1234).unwrap();
        assert_eq!(be.finish(), vec![0x12, 0x34]);

        let mut native = Writer::new(Endian::Native, ());
        native.write_u32(0x0102_0304).unwrap();
        assert_eq!(native.finish(), 0x0102_0304u32.to_ne_bytes().to_vec());
    }

    #[test]
    fn reads_endian_primitives_with_configured_byte_order() {
        let mut le = Reader::new(&[0x34, 0x12], Endian::Little, ());
        assert_eq!(le.read_u16().unwrap(), 0x1234);

        let mut be = Reader::new(&[0x12, 0x34], Endian::Big, ());
        assert_eq!(be.read_u16().unwrap(), 0x1234);

        let native_bytes = 0x1122_3344u32.to_ne_bytes();
        let mut native = Reader::new(&native_bytes, Endian::Native, ());
        assert_eq!(native.read_u32().unwrap(), 0x1122_3344);
    }

    #[test]
    fn raw_byte_and_bool_methods_work() {
        let mut writer = Writer::new(Endian::Little, ());
        writer.write_u8(255).unwrap();
        writer.write_i8(-1).unwrap();
        writer.write_bool_byte(false).unwrap();
        writer.write_bool_byte(true).unwrap();

        let bytes = writer.finish();
        assert_eq!(bytes, vec![255, 255, 0, 1]);

        let mut reader = Reader::new(&bytes, Endian::Little, ());
        assert_eq!(reader.read_u8().unwrap(), 255);
        assert_eq!(reader.read_i8().unwrap(), -1);
        assert!(!reader.read_bool_byte().unwrap());
        assert!(reader.read_bool_byte().unwrap());
    }

    #[test]
    fn roundtrip_multiple_primitive_types() {
        let mut writer = Writer::new(Endian::Big, ());
        writer.write_u32(0xDEAD_BEEF).unwrap();
        writer.write_i16(-1234).unwrap();
        writer.write_f32(3.5).unwrap();
        writer.write_f64(-1.25).unwrap();

        let bytes = writer.finish();
        let mut reader = Reader::new(&bytes, Endian::Big, ());

        assert_eq!(reader.read_u32().unwrap(), 0xDEAD_BEEF);
        assert_eq!(reader.read_i16().unwrap(), -1234);
        assert_eq!(reader.read_f32().unwrap(), 3.5);
        assert_eq!(reader.read_f64().unwrap(), -1.25);
    }

    #[test]
    fn read_errors_when_not_enough_bytes() {
        let mut reader = Reader::new(&[0xAA, 0xBB, 0xCC], Endian::Little, ());
        assert!(matches!(reader.read_u32(), Err(Error::OutOfBounds)));
    }
}
