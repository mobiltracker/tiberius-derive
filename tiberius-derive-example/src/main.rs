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
#[derive(FromRow)]
#[tiberius_derive(owned)]
struct FoobarOwned {
    pub foo: Option<String>,
    pub bar: String,
}

fn main() {
    println!("Hello, world!");
}
