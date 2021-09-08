use tiberius_derive::FromRow;

#[derive(FromRow)]
struct Foobar<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
}

// impl<'a> tiberius_derive_traits::FromRow<'a> for Foobar<'a> {
//     fn from_row(__row: &'a tiberius::Row) -> Result<Foobar<'a>, tiberius::error::Error> {
//         Ok(Self {
//             foo: { __row.try_get("foo")? },
//             bar: { __row.try_get("bar")? },
//         })
//     }
// }

fn main() {}
