// Generates Decode and Encode impls for primitive types via read_<suffix>/write_<suffix>.
macro_rules! impl_codec_primitive {
    ($( $ty:ident => ($read:ident, $write:ident) ),+ $(,)?) => {
        $(
            impl<Ctx> crate::Decode<Ctx> for $ty {
                #[inline]
                fn decode(r: &mut crate::Reader<'_, Ctx>) -> Result<Self, crate::Error> {
                    crate::ReadPrimitive::$read(r)
                }
            }

            impl<Ctx> crate::Encode<Ctx> for $ty {
                #[inline]
                fn encode(&self, w: &mut crate::Writer<Ctx>) -> Result<(), crate::Error> {
                    crate::WritePrimitive::$write(w, *self)
                }
            }
        )+
    };
}

// Add primitive types here to generate matching Decode and Encode impls.
impl_codec_primitive!(
    bool => (read_bool_byte, write_bool_byte),
    u8 => (read_u8, write_u8),
    u16 => (read_u16, write_u16),
    u32 => (read_u32, write_u32),
    u64 => (read_u64, write_u64),
    u128 => (read_u128, write_u128),
    usize => (read_usize, write_usize),
    i8 => (read_i8, write_i8),
    i16 => (read_i16, write_i16),
    i32 => (read_i32, write_i32),
    i64 => (read_i64, write_i64),
    i128 => (read_i128, write_i128),
    isize => (read_isize, write_isize),
    f32 => (read_f32, write_f32),
    f64 => (read_f64, write_f64),
);

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::{Endian, Error, Reader, Writer};

    #[test]
    fn encode_respects_endianness_for_multibyte_types() {
        let mut little = Writer::new(Endian::Little, ());
        little.write(&0x1234u16).unwrap();
        assert_eq!(little.finish(), vec![0x34, 0x12]);

        let mut big = Writer::new(Endian::Big, ());
        big.write(&0x1234u16).unwrap();
        assert_eq!(big.finish(), vec![0x12, 0x34]);
    }

    #[test]
    fn decode_respects_endianness_for_multibyte_types() {
        let mut little = Reader::new(&[0x34, 0x12], Endian::Little, ());
        let lv: u16 = little.read().unwrap();
        assert_eq!(lv, 0x1234);

        let mut big = Reader::new(&[0x12, 0x34], Endian::Big, ());
        let bv: u16 = big.read().unwrap();
        assert_eq!(bv, 0x1234);
    }

    #[test]
    fn encode_decode_roundtrip_uses_codec_traits() {
        let mut writer = Writer::new(Endian::Big, ());
        writer.write(&true).unwrap();
        writer.write(&0xDEAD_BEEFu32).unwrap();
        writer.write(&-12i8).unwrap();
        writer.write(&3.5f32).unwrap();

        let bytes = writer.finish();
        let mut reader = Reader::new(&bytes, Endian::Big, ());

        let b: bool = reader.read().unwrap();
        let n: u32 = reader.read().unwrap();
        let i: i8 = reader.read().unwrap();
        let f: f32 = reader.read().unwrap();

        assert!(b);
        assert_eq!(n, 0xDEAD_BEEF);
        assert_eq!(i, -12);
        assert_eq!(f, 3.5);
    }

    #[test]
    fn decode_errors_on_insufficient_input() {
        let mut reader = Reader::new(&[0xAA, 0xBB, 0xCC], Endian::Little, ());
        let result: Result<u32, Error> = reader.read();
        assert!(matches!(result, Err(Error::OutOfBounds)));
    }
}
