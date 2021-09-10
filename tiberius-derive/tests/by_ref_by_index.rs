use tiberius_derive::FromRow;

#[derive(FromRow)]
#[tiberius_derive(by_index)]
pub struct Foobar<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
}

// impl<'a> FoobarByIndex<'a> {
//     fn from_row(row: &tiberius::Row) -> Result<Self, tiberius::error::Error> {
//         Ok(Self {
//             foo: { row.try_get(0usize)? },
//             bar: {
//                 row.try_get("1usize")?
//                     .ok_or_else(|| tiberius::error::Error::Conversion(.....))?
//             },
//         })
//     }
// }

fn main() {}
