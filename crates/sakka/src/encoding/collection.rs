use crate::{Decode, Encode, ReadCollection, Reader, WriteCollection, Writer};

impl<Ctx, T, const N: usize> Encode<Ctx> for [T; N]
where
    T: Encode<Ctx>,
{
    type Error = T::Error;

    #[inline]
    fn encode(&self, writer: &mut Writer<Ctx>) -> Result<(), Self::Error> {
        writer.write_slice(self)
    }
}

impl<Ctx, T, const N: usize> Decode<Ctx> for [T; N]
where
    T: Decode<Ctx>,
{
    type Error = T::Error;

    #[inline]
    fn decode(reader: &mut Reader<'_, Ctx>) -> Result<Self, Self::Error> {
        reader.read_array()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Endian;

    #[test]
    fn encode_array_of_primitives() {
        let array: [u32; 3] = [0x1234_5678, 0xABCD_EF00, 0x0011_2233];
        let mut writer = Writer::new(Endian::Little, ());
        array.encode(&mut writer).unwrap();

        let bytes = writer.finish();
        // 3 * 4 bytes = 12 bytes
        assert_eq!(bytes.len(), 12);
    }

    #[test]
    fn decode_array_of_primitives() {
        let data: [u8; 12] = [
            0x78, 0x56, 0x34, 0x12, // 0x1234_5678 (little endian)
            0x00, 0xEF, 0xCD, 0xAB, // 0xABCD_EF00 (little endian)
            0x33, 0x22, 0x11, 0x00, // 0x0011_2233 (little endian)
        ];

        let mut reader = Reader::new(&data, Endian::Little, ());
        let array: [u32; 3] = reader.read_array().unwrap();

        assert_eq!(array, [0x1234_5678, 0xABCD_EF00, 0x0011_2233]);
    }

    #[test]
    fn roundtrip_array_of_primitives() {
        let original: [u16; 5] = [0x1234, 0x5678, 0xABCD, 0xEF00, 0x0011];

        let mut writer = Writer::new(Endian::Big, ());
        original.encode(&mut writer).unwrap();
        let bytes = writer.finish();

        let mut reader = Reader::new(&bytes, Endian::Big, ());
        let decoded: [u16; 5] = reader.read_array().unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn encode_array_of_u8() {
        let array: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        let mut writer = Writer::new(Endian::Little, ());
        array.encode(&mut writer).unwrap();

        let bytes = writer.finish();
        assert_eq!(bytes, &[0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn decode_array_of_u8() {
        let data = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

        let mut reader = Reader::new(&data, Endian::Little, ());
        let array: [u8; 5] = reader.read_array().unwrap();

        assert_eq!(array, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
    }

    #[test]
    fn roundtrip_array_of_u8() {
        let original: [u8; 3] = [0x11, 0x22, 0x33];

        let mut writer = Writer::new(Endian::Little, ());
        original.encode(&mut writer).unwrap();
        let bytes = writer.finish();

        let mut reader = Reader::new(&bytes, Endian::Little, ());
        let decoded: [u8; 3] = reader.read_array().unwrap();

        assert_eq!(decoded, original);
    }
}
