use tiberius_derive::FromRow;

#[derive(FromRow)]
struct FoobarNoLifetime {
    pub foo: Option<i32>,
}

// impl tiberius_derive_traits::FromRowCopy for FoobarNoLifetime {
//     fn from_row(__row: &tiberius::Row) -> Result<FoobarNoLifetime, tiberius::error::Error> {
//         Ok(Self {
//             foo: { __row.try_get("foo")? },
//         })
//     }
// }

fn main() {}
