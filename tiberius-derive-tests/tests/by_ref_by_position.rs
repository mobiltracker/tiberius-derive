use crate::util::connect_localhost;
use chrono::NaiveDateTime;
use tiberius::Uuid;
use tiberius_derive::FromRow;

mod util;

#[derive(FromRow, Debug, Clone, PartialEq)]
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

#[derive(FromRow, Debug, Clone, PartialEq, Default)]
#[tiberius_derive(by_position)]
struct TestRowNullable<'a> {
    pub id: i32,
    pub var_char_row: Option<&'a str>,
    pub n_var_char_row: Option<&'a str>,
    pub uuid_row: Option<Uuid>,
    pub long_row: Option<i64>,
    pub date_time_row: Option<chrono::NaiveDateTime>,
    pub small_int_row: Option<i16>,
    pub bit_row: Option<bool>,
    pub float_row: Option<f32>,
    // pub double_row: Option<f64>, // borked. Breaks because tiberius inteprets a a nullable float field as F32(None), even it its a f64
    pub real_row: Option<f32>,
}

#[tokio::test]
async fn by_ref_indexed_not_null() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[DoubleRow],[RealRow]
    FROM 
        [TiberiusDeriveTest].[dbo].[TestRow]
    WHERE VarCharRow is not null
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
        id: 1,
        var_char_row: "varchar",
        n_var_char_row: "nvarchar",
        long_row: 9999999999999999,
        small_int_row: 2,
        bit_row: true,
        float_row: 10.123123125,
        double_row: 99.1231231258,
        real_row: 10.5,
        uuid_row: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6").unwrap(),
        date_time_row: expected_time,
    };

    assert_eq!(&expected, rows.first().unwrap());

    Ok(())
}

#[tokio::test]
async fn by_ref_indexed_nullable() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],
        --[DoubleRow]
        [RealRow]
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
        .map(TestRowNullable::from_row)
        .collect::<Result<Vec<_>, _>>()?;

    let expected_time =
        NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let expected_1 = TestRowNullable {
        id: 1,
        var_char_row: "varchar".into(),
        n_var_char_row: "nvarchar".into(),
        long_row: 9999999999999999.into(),
        small_int_row: 2.into(),
        bit_row: true.into(),
        float_row: 10.123123125.into(),
        // double_row: 99.1231231258.into(),
        real_row: 10.5.into(),
        uuid_row: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6")
            .unwrap()
            .into(),
        date_time_row: expected_time.into(),
    };

    let expected_2 = TestRowNullable {
        id: 2,
        ..TestRowNullable::default()
    };

    assert_eq!(expected_1, rows[0]);
    assert_eq!(expected_2, rows[1]);

    Ok(())
}
