use tiberius_derive::FromRow;

#[derive(FromRow)]
#[tiberius_derive(by_position)]
pub struct Foobar<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
}

#[derive(FromRow)]
#[tiberius_derive(by_position, auto)]
pub struct FoobarAuto<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
    pub auto: String,
    pub auto_opn: Option<String>,
}

fn main() {}
