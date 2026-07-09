use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, Default, PartialEq)]
struct Ctx {
    version: u8,
    count: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(context = Ctx)]
struct StoredFields {
    #[sakka(store = version)]
    version: u8,
    #[sakka(store = count)]
    count: u16,
}

#[test]
fn store_updates_context_after_encode() {
    let value = StoredFields { version: 3, count: 42 };

    let mut writer = Writer::new(Endian::Little, Ctx::default());
    writer.write(&value).unwrap();

    assert_eq!(writer.context(), &Ctx { version: 3, count: 42 });
}

#[test]
fn store_updates_context_after_decode() {
    let bytes = [3u8, 42u8, 0u8];

    let mut reader = Reader::new(&bytes, Endian::Little, Ctx::default());
    let decoded: StoredFields = reader.read().unwrap();

    assert_eq!(decoded, StoredFields { version: 3, count: 42 });
    assert_eq!(reader.context(), &Ctx { version: 3, count: 42 });
}
