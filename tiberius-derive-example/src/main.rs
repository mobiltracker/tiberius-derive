use tiberius_derive::FromRow;

#[derive(FromRow)]
pub struct Foobar<'b> {
    pub foo: Option<i32>,
    pub bar: Option<&'b str>,
}

#[derive(FromRow)]
pub struct FoobarNoLifetime {
    pub foo: Option<i32>,
}

#[derive(FromRow)]
#[tiberius_derive(owned)]
pub struct FoobarOwned {
    pub foo: Option<String>,
    pub bar: String,
}

#[derive(FromRow)]
#[tiberius_derive(by_index)]
pub struct FoobarByIndex {
    pub foo: Option<i32>,
    pub bar: i32,
}

#[derive(FromRow)]
#[tiberius_derive(rename_all = "camelCase")]
pub struct FoobarRenamed {
    pub foo_bar: Option<i32>,
    pub bar_foo: i32,
}

impl FoobarRenamed {
    pub fn multiple_impls_are_fine() {}
}

fn main() {
    println!("Hello, world!");
}
