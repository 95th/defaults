use trybuild::TestCases;

#[test]
fn tests() {
    let t = TestCases::new();
    t.pass("tests/01-simple-struct.rs");
    t.pass("tests/02-other-derive.rs");
    t.compile_fail("tests/03-conflict.rs");
    t.pass("tests/04-custom-default.rs");
    t.pass("tests/05-custom-default-with-fn.rs");
    t.pass("tests/06-cast.rs");
    t.compile_fail("tests/07-invalid-attr-value-type.rs");
    t.compile_fail("tests/08-attr-empty.rs");
    t.compile_fail("tests/09-non-default-field.rs");
    t.pass("tests/10-custom-default-struct.rs");
    t.compile_fail("tests/11-invalid-expr.rs");
    t.compile_fail("tests/12-enums.rs");
    t.compile_fail("tests/13-multiple-attrs.rs");
    t.pass("tests/14-custom-default-struct-named.rs");
    t.pass("tests/15-enums-attr.rs");
    t.pass("tests/16-enums-struct-variant.rs");
    t.compile_fail("tests/17-enums-struct-variant-incomplete.rs");
    t.pass("tests/18-enums-tuple-variant.rs");
    t.compile_fail("tests/19-enums-tuple-variant-incomplete.rs");
}
