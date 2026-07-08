#[test]
fn derive_cases() {
    let t = trybuild::TestCases::new();

    t.pass("tests/trybuild/derive_structs.rs");
    t.pass("tests/trybuild/collection_valid.rs");
    t.pass("tests/trybuild/alignment_valid.rs");
    t.pass("tests/trybuild/padding_valid.rs");
    t.pass("tests/trybuild/custom_encoding_valid.rs");
    t.pass("tests/trybuild/struct_error_valid.rs");
    t.pass("tests/trybuild/ignore_valid.rs");

    t.compile_fail("tests/trybuild/derive_non_derived.rs");
    t.compile_fail("tests/trybuild/collection_vec_missing_attr.rs");
    t.compile_fail("tests/trybuild/collection_non_vec.rs");
    t.compile_fail("tests/trybuild/collection_duplicate_attr.rs");
    t.compile_fail("tests/trybuild/alignment_duplicate_attr.rs");
    t.compile_fail("tests/trybuild/padding_dupe.rs");
    t.compile_fail("tests/trybuild/custom_encoding_dupe.rs");
    t.compile_fail("tests/trybuild/custom_encoding_wrong_sig.rs");
    t.compile_fail("tests/trybuild/struct_error_dupe.rs");
    t.compile_fail("tests/trybuild/ignore_dupe.rs");
    t.compile_fail("tests/trybuild/ignore_with_custom_encoding.rs");
}
