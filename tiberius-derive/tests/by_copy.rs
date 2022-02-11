use tiberius_derive::FromRow;

#[derive(FromRow)]
pub struct FoobarNoLifetime {
    pub foo: Option<i32>,
}

fn main() {}
