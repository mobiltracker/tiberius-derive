use std::usize;
use tiberius_derive::FromRow;

#[derive(FromRow)]
struct Foobar {
    pub foo: Option<i32>,
    pub bar: Option<String>,
}

// impl<'a> FromRow<'a> for Foobar<'a> {
//     fn from_row(__row: &'a tiberius::Row) -> Result<Foobar<'a>, tiberius::error::Error> {
//         let foo = __row.get(0);
//         let bar = __row.get(1);
//         let xar = __row.get::<&'a str, usize>(2).expect("foobar");

//         Ok(Self { bar, foo, xar })
//     }
// }

fn main() {
    println!("Hello, world!");
}
