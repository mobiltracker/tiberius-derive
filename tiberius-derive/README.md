# tiberius-derive

Derive macros for [Tiberius](https://github.com/prisma/tiberius).

This crate provides a FromRow derive macro that allows for easier parsing and error handling of rows. 
Struct fields must implement either [FromSql](https://docs.rs/tiberius/latest/tiberius/trait.FromSql.html) or [FromSqlOwned](https://docs.rs/tiberius/latest/tiberius/trait.FromSqlOwned.html)
and FromRow parsing behaviour is controlled by struct container attributes. 

It should be possible to customize macro expansion to get columns by 'name/key' or by position (by position being faster, but 
forcing the struct fields to be in the same order as the values in the SELECT clause), either by borrowing (struct fields implementing FromSql) or by 
value (struct fields implementing FromSqlOwned). Field renaming is also implemented similarly as in [serde](https://serde.rs/container-attrs.html#rename_all)
 (right now only renameAll is implemented). 
 

Everything is still in alpha, and there is a lot of functionality to be implemented but have been using this in production in a bunch of critical/HA services 
for close to an year now without major issues. Error handling could be vastly improved, and there is no support for changing the behaviour with field attributes
(ex: renaming a single struct field instead of all of them) at the moment.


## Known issues:

There is a tiberius bug(?) that breaks Option<f64> fields at the moment. Tiberius appears to always parse null Float(53) (double) columns as Nullable(f32)
independently of the column type.

```rust
// DoubleColumn is a Nullable float(53)
let query = "SELECT
        [DoubleColumn]
    FROM 
        [TestRow]
";

let rows = client
        .simple_query(query)
        .await?
        .into_first_result()
        .await?;

let double : Option<f64> = rows[0].get("DoubleColumn") // Error: Conversion("cannot interpret F32(None) as an f64 value")

```

## Examples

### FromRow: Get by ref and name
Fields must implement FromSql. Since ```String``` does not implement FromSql, if you want to use strings you need the #[tiberius_derive(auto)] container 
attrbute.


https://github.com/mobiltracker/tiberius-derive/blob/master/tiberius-derive-tests/tests/by_ref.rs

```rust
#[allow(non_snake_case)]
#[derive(FromRow)]
struct TestRow<'a> {
    pub Id: i32,
    pub VarCharRow: &'a str,
    pub NVarCharRow: &'a str,
    pub UuidRow: Uuid,
    pub LongRow: i64,
    pub DateTimeRow: chrono::NaiveDateTime,
    pub SmallIntRow: i16,
    pub BitRow: bool,
    pub FloatRow: f32,
    pub DoubleRow: f64,
    pub RealRow: f32,
}

async fn by_ref_not_null() -> Result<(), tiberius::error::Error> {
  let query = r"
  SELECT
      [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[DoubleRow],[RealRow]
  FROM 
      [TiberiusDeriveTest].[dbo].[TestRow]
  ";

  let rows = client
      .simple_query(query)
      .await?
      .into_first_result()
      .await?;

  let rows = rows
      .iter()
      .map(TestRow::from_row)
      .collect::<Result<Vec<_>, _>>()?;
        
}
```

### FromRow: Get by ref and position
```#[tiberius_derive(by_position)]```

Faster, but struct fields must be in the same order as the values in the SELECT clause.

https://github.com/mobiltracker/tiberius-derive/blob/master/tiberius-derive-tests/tests/by_ref_by_position.rs

``` rust
#[derive(FromRow)]
#[tiberius_derive(by_position)]
struct TestRow<'a> {
    pub id: i32,
    pub var_char_row: &'a str,
    pub n_var_char_row: &'a str,
    pub uuid_row: Uuid,
    pub long_row: i64,
    pub date_time_row: chrono::NaiveDateTime,
    pub small_int_row: i16,
    pub bit_row: bool,
    pub float_row: f32,
    pub double_row: f64,
    pub real_row: f32,
}


async fn by_ref_indexed_not_null() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[DoubleRow],[RealRow]
    FROM 
        [TiberiusDeriveTest].[dbo].[TestRow]
        ";

    let rows = client
        .simple_query(query)
        .await?
        .into_first_result()
        .await?;

    let rows = rows
        .iter()
        .map(TestRow::from_row)
        .collect::<Result<Vec<_>, _>>()?;
}


```

### FromRow: Get by value (and by position)

```#[tiberius_derive(owned)]```

Fields must implement FromSqlOwned, and struct fields must be in the same order as the values in the SELECT clause.

https://github.com/mobiltracker/tiberius-derive/blob/master/tiberius-derive-tests/tests/by_value.rs

``` rust

#[allow(non_snake_case)]
#[derive(FromRow)]
#[tiberius_derive(owned)]
struct TestRow {
    pub id: i32,
    pub var_char_row: String,
    pub n_var_char_row: String,
    pub uuid_row: Uuid,
    pub long_row: i64,
    pub date_time_row: chrono::NaiveDateTime,
    pub small_int_row: i16,
    pub bit_row: bool,
    pub float_row: f32,
    pub double_row: f64,
    pub real_row: f32,
}


#[tokio::test]
async fn by_value() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[DoubleRow],[RealRow]
    FROM 
        [TiberiusDeriveTest].[dbo].[TestRow]
        ";

    let rows = client
        .simple_query(query)
        .await?
        .into_first_result()
        .await?;

    let rows = rows
        .into_iter()
        .map(TestRow::from_row)
        .collect::<Result<Vec<_>, _>>()?;
 }

```

### FromRow: RenameAll
```#[tiberius_derive(rename_all = "PascalCase")]```

https://github.com/mobiltracker/tiberius-derive/blob/master/tiberius-derive-tests/tests/rename_all.rs

Renames fields (if getting column data by name)

``` rust
#[derive(FromRowa)]
#[tiberius_derive(rename_all = "PascalCase")]
struct TestRow<'a> {
    pub id: i32,
    pub var_char_row: &'a str,
    pub n_var_char_row: &'a str,
    pub uuid_row: Uuid,
    pub long_row: i64,
    pub date_time_row: chrono::NaiveDateTime,
    pub small_int_row: i16,
    pub bit_row: bool,
    pub float_row: f32,
    pub double_row: f64,
    pub real_row: f32,
}

#[tokio::test]
async fn rename_all() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[DoubleRow],[RealRow]
    FROM 
        [TiberiusDeriveTest].[dbo].[TestRow]
        ";

    let rows = client
        .simple_query(query)
        .await?
        .into_first_result()
        .await?;

    let rows = rows
        .iter()
        .map(TestRow::from_row)
        .collect::<Result<Vec<_>, _>>()?;
 }
```

### FromRow: Converting &str to String by cloning
```#[tiberius_derive(auto)]```

https://github.com/mobiltracker/tiberius-derive/blob/master/tiberius-derive-tests/tests/str_auto_owned.rs

Since strings do not implement FromSql, having an attribute to do this is reasonably ergonomic. 

``` rust

#[allow(non_snake_case)]
#[derive(FromRow)]
#[tiberius_derive(auto)]
struct TestRow {
    pub Id: i32,
    pub VarCharRow: String,
    pub NVarCharRow: String,
    pub UuidRow: Uuid,
    pub LongRow: i64,
    pub DateTimeRow: chrono::NaiveDateTime,
    pub SmallIntRow: i16,
    pub BitRow: bool,
    pub FloatRow: f32,
    pub DoubleRow: f64,
    pub RealRow: f32,
}

#[tokio::test]
async fn by_ref_auto_not_null() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[DoubleRow],[RealRow]
    FROM 
        [TiberiusDeriveTest].[dbo].[TestRow]
        ";

    let rows = client
        .simple_query(query)
        .await?
        .into_first_result()
        .await?;

    let rows = rows
        .iter()
        .map(TestRow::from_row)
        .collect::<Result<Vec<_>, _>>()?;
}


```




