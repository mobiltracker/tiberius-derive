use crate::util::connect_localhost;
use chrono::NaiveDateTime;
use tiberius::Uuid;
use tiberius_derive::FromRow;

mod util;

#[allow(non_snake_case)]
#[derive(FromRow, Debug, Clone, PartialEq)]
struct TestRow<'a> {
    pub Id: i32,
    pub VarCharRow: &'a str,
    pub NVarCharRow: &'a str,
    pub UuidRow: Uuid,
    pub LongRow: i64,
    pub DateTimeRow: chrono::NaiveDateTime,
    pub SmallIntRow: i16,
    pub BitRow: bool,
    pub FloatRow: f64,
    pub RealRow: f32,
}

#[tokio::test]
async fn it_works() -> Result<(), tiberius::error::Error> {
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
        .iter()
        .map(TestRow::from_row)
        .collect::<Result<Vec<_>, _>>()?;

    let expected_time =
        NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let expected = TestRow {
        Id: 1,
        VarCharRow: "varchar",
        NVarCharRow: "nvarchar",
        LongRow: 9999999999999999,
        SmallIntRow: 2,
        BitRow: true,
        FloatRow: 10.123123125,
        RealRow: 10.5,
        UuidRow: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6").unwrap(),
        DateTimeRow: expected_time,
    };

    assert_eq!(&expected, rows.first().unwrap());

    Ok(())
}
