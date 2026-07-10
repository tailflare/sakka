use sakka::{Decode, Encode, Endian, Reader, Writer};

#[derive(Debug, Default, PartialEq)]
struct Ctx {
    version: u8,
    count: u16,
    size: usize,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(context = Ctx)]
struct StoredFields {
    #[sakka(store = version)]
    version: u8,
    #[sakka(store = count)]
    count: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[sakka(context = Ctx)]
struct StoredFieldExpr {
    #[sakka(store(version = value))]
    version: u8,
    #[sakka(store(count = value.len() as u16))]
    payload: [u8; 3],
    #[sakka(store(size = value.len()))]
    trailer: [u8; 2],
}

#[test]
fn store_updates_context_after_encode() {
    let value = StoredFields { version: 3, count: 42 };

    let mut writer = Writer::new(Endian::Little, Ctx::default());
    writer.write(&value).unwrap();

    assert_eq!(writer.context(), &Ctx { version: 3, count: 42, size: 0 });
}

#[test]
fn store_updates_context_after_decode() {
    let bytes = [3u8, 42u8, 0u8];

    let mut reader = Reader::new(&bytes, Endian::Little, Ctx::default());
    let decoded: StoredFields = reader.read().unwrap();

    assert_eq!(decoded, StoredFields { version: 3, count: 42 });
    assert_eq!(reader.context(), &Ctx { version: 3, count: 42, size: 0 });
}

#[test]
fn store_expr_updates_context_after_encode() {
    let value = StoredFieldExpr { version: 7, payload: [1, 2, 3], trailer: [4, 5] };

    let mut writer = Writer::new(Endian::Little, Ctx::default());
    writer.write(&value).unwrap();

    assert_eq!(writer.context(), &Ctx { version: 7, count: 3, size: 2 },);
}

#[test]
fn store_expr_updates_context_after_decode() {
    let bytes = [7u8, 1u8, 2u8, 3u8, 4u8, 5u8];

    let mut reader = Reader::new(&bytes, Endian::Little, Ctx::default());
    let decoded: StoredFieldExpr = reader.read().unwrap();

    assert_eq!(decoded, StoredFieldExpr { version: 7, payload: [1, 2, 3], trailer: [4, 5] });
    assert_eq!(reader.context(), &Ctx { version: 7, count: 3, size: 2 },);
}
