use crate::util::connect_localhost;
use chrono::NaiveDateTime;
use tiberius::Uuid;
use tiberius_derive::FromRow;
mod util;

#[derive(FromRow, Debug, Clone, PartialEq)]
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
    pub float_row: f64,
    pub real_row: f32,
}

#[tokio::test]
async fn by_value() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[RealRow]
    FROM 
        [TiberiusDeriveTest].[dbo].[TestRow]
    ORDER BY ID
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

    let expected_time =
        NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let expected = TestRow {
        id: 1,
        var_char_row: "varchar".to_owned(),
        n_var_char_row: "nvarchar".to_owned(),
        long_row: 9999999999999999,
        small_int_row: 2,
        bit_row: true,
        float_row: 10.123123125,
        real_row: 10.5,
        uuid_row: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6").unwrap(),
        date_time_row: expected_time,
    };

    assert_eq!(&expected, rows.first().unwrap());

    Ok(())
}
