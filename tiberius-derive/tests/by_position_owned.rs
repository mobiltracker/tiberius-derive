use tiberius_derive::FromRow;

#[derive(FromRow)]
#[tiberius_derive(owned)]
struct FoobarOwned {
    pub foo: Option<String>,
    pub bar: String,
}

fn main() {}
