use std::convert::TryFrom;
use tiberius_derive::TiberiusRow;

#[derive(TiberiusRow)]
struct Foobar {
    foo: Option<bool>,
    bar: Option<i32>,
}

fn main() {}
