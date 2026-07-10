#[test]
fn derive_cases() {
    let t = trybuild::TestCases::new();

    t.pass("tests/trybuild/derive_structs.rs");
    t.pass("tests/trybuild/collection_valid.rs");
    t.pass("tests/trybuild/alignment_valid.rs");
    t.pass("tests/trybuild/padding_valid.rs");
    t.pass("tests/trybuild/custom_codec_valid.rs");
    t.pass("tests/trybuild/struct_error_valid.rs");
    t.pass("tests/trybuild/struct_context_valid.rs");
    t.pass("tests/trybuild/ignore_valid.rs");
    t.pass("tests/trybuild/store_valid.rs");
    t.pass("tests/trybuild/optional_valid.rs");
    t.pass("tests/trybuild/optional_bool_custom_error_valid.rs");
    t.pass("tests/trybuild/optional_collection_valid.rs");

    t.compile_fail("tests/trybuild/derive_non_derived.rs");
    t.compile_fail("tests/trybuild/collection_vec_missing_attr.rs");
    t.compile_fail("tests/trybuild/collection_non_vec.rs");
    t.compile_fail("tests/trybuild/collection_duplicate_attr.rs");
    t.compile_fail("tests/trybuild/collection_field_not_before.rs");
    t.compile_fail("tests/trybuild/alignment_duplicate_attr.rs");
    t.compile_fail("tests/trybuild/padding_dupe.rs");
    t.compile_fail("tests/trybuild/custom_codec_dupe.rs");
    t.compile_fail("tests/trybuild/struct_error_dupe.rs");
    t.compile_fail("tests/trybuild/struct_context_dupe.rs");
    t.compile_fail("tests/trybuild/ignore_dupe.rs");
    t.compile_fail("tests/trybuild/store_dupe.rs");
    t.compile_fail("tests/trybuild/store_missing_context_field.rs");
    t.compile_fail("tests/trybuild/optional_non_option.rs");
    t.compile_fail("tests/trybuild/optional_dupe.rs");
    t.compile_fail("tests/trybuild/optional_collection_missing_attr.rs");
}
