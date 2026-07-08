use sakka::{Decode, Encode, Error, ReadPrimitive, Reader, WritePrimitive, Writer};

#[derive(Debug)]
enum MyError {
    Core(Error),
}

impl From<Error> for MyError {
    fn from(value: Error) -> Self {
        Self::Core(value)
    }
}

#[derive(Encode, Decode)]
#[sakka(error = MyError)]
struct StructErrorValid {
    value: u16,
}

fn main() {
    let mut writer = Writer::new(sakka::Endian::Little, ());
    let value = StructErrorValid { value: 0x1234 };
    let _: Result<(), MyError> = writer.write(&value);

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes, sakka::Endian::Little, ());
    let _: Result<StructErrorValid, MyError> = reader.read();

    // Keep primitive methods in scope to validate generated code paths.
    let _ = Writer::new(sakka::Endian::Little, ()).write_u8(1);
    let _ = Reader::new(&[1u8], sakka::Endian::Little, ()).read_u8();
}
