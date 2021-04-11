#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/by_ref.rs");
}
