use tiberius_derive::FromRow;

#[derive(FromRow)]
pub struct FoobarNoLifetime {
    pub foo: Option<i32>,
}

// impl FoobarNoLifetime {
//     fn from_row(__row: &tiberius::Row) -> Result<FoobarNoLifetime, tiberius::error::Error> {
//         Ok(Self {
//             foo: { __row.try_get("foo")? },
//         })
//     }
// }

fn main() {}
