use sakka::{Decode, Encode, Error, Reader, Writer};

#[derive(Debug)]
enum MyError {
    Core(Error),
}

impl From<Error> for MyError {
    fn from(value: Error) -> Self {
        Self::Core(value)
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(error = MyError)]
struct OptionalBoolCustomError {
    #[sakka(optional(bool))]
    value: Option<u16>,
}

fn main() {
    let mut writer = Writer::new(sakka::Endian::Little, ());
    let value = OptionalBoolCustomError { value: Some(0x1234) };
    let _: Result<(), MyError> = writer.write(&value);

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes, sakka::Endian::Little, ());
    let _: Result<OptionalBoolCustomError, MyError> = reader.read();
}
