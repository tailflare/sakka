use sakka::{Decode, Encode, Reader, Writer};

#[derive(Default)]
struct MyCtx {
    _tag: u8,
}

#[derive(Encode, Decode)]
#[sakka(context = MyCtx)]
struct StructContextValid {
    value: u16,
}

fn main() {
    let mut writer = Writer::new(sakka::Endian::Little, MyCtx::default());
    let value = StructContextValid { value: 0x1234 };
    let _: Result<(), sakka::Error> = writer.write(&value);

    let bytes = writer.finish();
    let mut reader = Reader::new(&bytes, sakka::Endian::Little, MyCtx::default());
    let _: Result<StructContextValid, sakka::Error> = reader.read();
}
