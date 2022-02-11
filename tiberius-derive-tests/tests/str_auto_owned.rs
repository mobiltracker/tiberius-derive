use crate::util::connect_localhost;
use chrono::NaiveDateTime;
use tiberius::Uuid;
use tiberius_derive::FromRow;
mod util;

#[allow(non_snake_case)]
#[derive(FromRow, Debug, Clone, PartialEq)]
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

#[allow(non_snake_case)]
#[derive(FromRow, Debug, Clone, PartialEq)]
#[tiberius_derive(auto, by_position)]
struct TestRowByPositionAuto {
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

#[allow(non_snake_case)]
#[derive(FromRow, Debug, Clone, PartialEq, Default)]
#[tiberius_derive(auto)]
struct TestRowNullable {
    pub Id: i32,
    pub VarCharRow: Option<String>,
    pub NVarCharRow: Option<String>,
    pub UuidRow: Option<Uuid>,
    pub LongRow: Option<i64>,
    pub DateTimeRow: Option<chrono::NaiveDateTime>,
    pub SmallIntRow: Option<i16>,
    pub BitRow: Option<bool>,
    pub FloatRow: Option<f32>,
    //  pub DoubleRow: Option<f64>, // borked. Breaks because tiberius inteprets a a nullable float field as F32(None)
    pub RealRow: Option<f32>,
}

#[tokio::test]
async fn by_ref_auto_not_null() -> Result<(), tiberius::error::Error> {
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
        Id: 1,
        VarCharRow: "varchar".to_owned(),
        NVarCharRow: "nvarchar".to_owned(),
        LongRow: 9999999999999999,
        SmallIntRow: 2,
        BitRow: true,
        FloatRow: 10.123123125,
        DoubleRow: 99.1231231258,
        RealRow: 10.5,
        UuidRow: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6").unwrap(),
        DateTimeRow: expected_time,
    };

    assert_eq!(&expected, rows.first().unwrap());

    Ok(())
}

#[tokio::test]
async fn by_ref_by_position_auto_not_null() -> Result<(), tiberius::error::Error> {
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
        .map(TestRowByPositionAuto::from_row)
        .collect::<Result<Vec<_>, _>>()?;

    let expected_time =
        NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let expected = TestRowByPositionAuto {
        Id: 1,
        VarCharRow: "varchar".to_owned(),
        NVarCharRow: "nvarchar".to_owned(),
        LongRow: 9999999999999999,
        SmallIntRow: 2,
        BitRow: true,
        FloatRow: 10.123123125,
        DoubleRow: 99.1231231258,
        RealRow: 10.5,
        UuidRow: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6").unwrap(),
        DateTimeRow: expected_time,
    };

    assert_eq!(&expected, rows.first().unwrap());

    Ok(())
}

#[tokio::test]
async fn by_ref_auto_nullable() -> Result<(), tiberius::error::Error> {
    let mut client = connect_localhost().await.unwrap();
    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],
        -- [DoubleRow],
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
        Id: 1,
        VarCharRow: "varchar".to_owned().into(),
        NVarCharRow: "nvarchar".to_owned().into(),
        LongRow: 9999999999999999.into(),
        SmallIntRow: 2.into(),
        BitRow: true.into(),
        FloatRow: 10.123123125.into(),
        RealRow: 10.5.into(),
        UuidRow: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6")
            .unwrap()
            .into(),
        DateTimeRow: expected_time.into(),
    };

    let expected_2 = TestRowNullable {
        Id: 2,
        ..TestRowNullable::default()
    };

    assert_eq!(expected_1, rows[0]);
    assert_eq!(expected_2, rows[1]);

    Ok(())
}
