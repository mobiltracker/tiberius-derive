use tiberius_derive::FromRow;

#[derive(FromRow)]
pub struct Foobar<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
}

#[derive(FromRow)]
#[tiberius_derive(auto)]
pub struct FoobarAuto<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
    pub auto: String,
    pub auto_opt: Option<String>,
}

fn main() {}
