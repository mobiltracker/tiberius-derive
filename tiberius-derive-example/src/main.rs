use std::usize;
use tiberius_derive::FromRow;

#[derive(FromRow)]
struct Foobar<'b> {
    pub foo: Option<i32>,
    pub bar: Option<&'b str>,
}

#[derive(FromRow)]
struct FoobarNoLifetime {
    pub foo: Option<i32>,
}

fn main() {
    println!("Hello, world!");
}
