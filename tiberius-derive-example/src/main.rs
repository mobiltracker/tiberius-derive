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

#[derive(FromRow)]
#[tiberius_derive(by_index)]
struct FoobarByIndex {
    pub foo: Option<i32>,
    pub bar: i32,
}

#[derive(FromRow)]
#[tiberius_derive(rename_all = "camelCase")]
struct FoobarRenamed {
    pub foo_bar: Option<i32>,
    pub bar_foo: i32,
}

fn main() {
    println!("Hello, world!");
}
