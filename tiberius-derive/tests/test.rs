#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/by_ref.rs");
    t.pass("tests/by_copy.rs");
    t.pass("tests/by_position_owned.rs");
    t.pass("tests/by_ref_by_index.rs");
}
