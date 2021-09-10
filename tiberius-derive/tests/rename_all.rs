use tiberius_derive::FromRow;

#[derive(FromRow)]
#[tiberius_derive(rename_all = "PascalCase")]

pub struct Foobar<'a> {
    pub foo_bar: Option<i32>,
    pub bar_foo: Option<&'a str>,
}

// impl FoobarByIndex {
//     pub fn from_row(row: &tiberius::Row) -> Result<Self, tiberius::error::Error> {
//         Ok(Self {
//             foo: { row.try_get("FooBar")? },
//             bar: {
//                 row.try_get("BarFoo")?
//                     .ok_or_else(|| tiberius::error::Error::Conversion(.....))?
//             },
//         })
//     }
// }

fn main() {}
