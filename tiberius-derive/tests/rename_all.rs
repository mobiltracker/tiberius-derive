use tiberius_derive::FromRow;

#[derive(FromRow)]
#[tiberius_derive(rename_all = "PascalCase")]

pub struct Foobar<'a> {
    pub foo_bar: Option<i32>,
    pub bar_foo: Option<&'a str>,
}

fn main() {}
