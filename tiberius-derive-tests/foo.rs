#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use crate::util::connect_localhost;
use chrono::NaiveDateTime;
use tiberius::Uuid;
use tiberius_derive::FromRow;
mod util {
    #![allow(dead_code)]
    use once_cell::sync::Lazy;
    use tiberius::Client;
    use tokio::net::TcpStream;
    use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
    static CONN_STR: Lazy<String> = Lazy::new(|| {
        std::env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
            "server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=true"
                .to_owned()
        })
    });
    pub async fn connect_localhost() -> Result<Client<Compat<TcpStream>>, tiberius::error::Error> {
        let config = tiberius::Config::from_ado_string(&CONN_STR)?;
        let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        let client = tiberius::Client::connect(config, tcp.compat_write()).await?;
        Ok(client)
    }
}
#[tiberius_derive(by_index)]
struct TestRow<'a> {
    pub id: i32,
    pub var_char_row: &'a str,
    pub n_var_char_row: &'a str,
    pub uuid_row: Uuid,
    pub long_row: i64,
    pub date_time_row: chrono::NaiveDateTime,
    pub small_int_row: i16,
    pub bit_row: bool,
    pub float_row: f64,
    pub real_row: f32,
}
impl<'a> TestRow<'a> {
    pub fn from_row(row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error> {
        Ok(Self {
            id: {
                row.try_get("0usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"id", &0usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            var_char_row: {
                row.try_get("1usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"var_char_row", &1usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            n_var_char_row: {
                row.try_get("2usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"n_var_char_row", &2usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            uuid_row: {
                row.try_get("3usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"uuid_row", &3usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            long_row: {
                row.try_get("4usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"long_row", &4usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            date_time_row: {
                row.try_get("5usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"date_time_row", &5usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            small_int_row: {
                row.try_get("6usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"small_int_row", &6usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            bit_row: {
                row.try_get("7usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"bit_row", &7usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            float_row: {
                row.try_get("8usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"float_row", &8usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
            real_row: {
                row.try_get("9usize")?.ok_or_else(|| {
                    tiberius::error::Error::Conversion(
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    " None value for non optional field ",
                                    " from column with index ",
                                ],
                                &match (&"real_row", &9usize) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }
                        .into(),
                    )
                })?
            },
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::fmt::Debug for TestRow<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            TestRow {
                id: ref __self_0_0,
                var_char_row: ref __self_0_1,
                n_var_char_row: ref __self_0_2,
                uuid_row: ref __self_0_3,
                long_row: ref __self_0_4,
                date_time_row: ref __self_0_5,
                small_int_row: ref __self_0_6,
                bit_row: ref __self_0_7,
                float_row: ref __self_0_8,
                real_row: ref __self_0_9,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "TestRow");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "var_char_row",
                    &&(*__self_0_1),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "n_var_char_row",
                    &&(*__self_0_2),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "uuid_row",
                    &&(*__self_0_3),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "long_row",
                    &&(*__self_0_4),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "date_time_row",
                    &&(*__self_0_5),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "small_int_row",
                    &&(*__self_0_6),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "bit_row",
                    &&(*__self_0_7),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "float_row",
                    &&(*__self_0_8),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "real_row",
                    &&(*__self_0_9),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::clone::Clone for TestRow<'a> {
    #[inline]
    fn clone(&self) -> TestRow<'a> {
        match *self {
            TestRow {
                id: ref __self_0_0,
                var_char_row: ref __self_0_1,
                n_var_char_row: ref __self_0_2,
                uuid_row: ref __self_0_3,
                long_row: ref __self_0_4,
                date_time_row: ref __self_0_5,
                small_int_row: ref __self_0_6,
                bit_row: ref __self_0_7,
                float_row: ref __self_0_8,
                real_row: ref __self_0_9,
            } => TestRow {
                id: ::core::clone::Clone::clone(&(*__self_0_0)),
                var_char_row: ::core::clone::Clone::clone(&(*__self_0_1)),
                n_var_char_row: ::core::clone::Clone::clone(&(*__self_0_2)),
                uuid_row: ::core::clone::Clone::clone(&(*__self_0_3)),
                long_row: ::core::clone::Clone::clone(&(*__self_0_4)),
                date_time_row: ::core::clone::Clone::clone(&(*__self_0_5)),
                small_int_row: ::core::clone::Clone::clone(&(*__self_0_6)),
                bit_row: ::core::clone::Clone::clone(&(*__self_0_7)),
                float_row: ::core::clone::Clone::clone(&(*__self_0_8)),
                real_row: ::core::clone::Clone::clone(&(*__self_0_9)),
            },
        }
    }
}
impl<'a> ::core::marker::StructuralPartialEq for TestRow<'a> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::cmp::PartialEq for TestRow<'a> {
    #[inline]
    fn eq(&self, other: &TestRow<'a>) -> bool {
        match *other {
            TestRow {
                id: ref __self_1_0,
                var_char_row: ref __self_1_1,
                n_var_char_row: ref __self_1_2,
                uuid_row: ref __self_1_3,
                long_row: ref __self_1_4,
                date_time_row: ref __self_1_5,
                small_int_row: ref __self_1_6,
                bit_row: ref __self_1_7,
                float_row: ref __self_1_8,
                real_row: ref __self_1_9,
            } => match *self {
                TestRow {
                    id: ref __self_0_0,
                    var_char_row: ref __self_0_1,
                    n_var_char_row: ref __self_0_2,
                    uuid_row: ref __self_0_3,
                    long_row: ref __self_0_4,
                    date_time_row: ref __self_0_5,
                    small_int_row: ref __self_0_6,
                    bit_row: ref __self_0_7,
                    float_row: ref __self_0_8,
                    real_row: ref __self_0_9,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                        && (*__self_0_6) == (*__self_1_6)
                        && (*__self_0_7) == (*__self_1_7)
                        && (*__self_0_8) == (*__self_1_8)
                        && (*__self_0_9) == (*__self_1_9)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &TestRow<'a>) -> bool {
        match *other {
            TestRow {
                id: ref __self_1_0,
                var_char_row: ref __self_1_1,
                n_var_char_row: ref __self_1_2,
                uuid_row: ref __self_1_3,
                long_row: ref __self_1_4,
                date_time_row: ref __self_1_5,
                small_int_row: ref __self_1_6,
                bit_row: ref __self_1_7,
                float_row: ref __self_1_8,
                real_row: ref __self_1_9,
            } => match *self {
                TestRow {
                    id: ref __self_0_0,
                    var_char_row: ref __self_0_1,
                    n_var_char_row: ref __self_0_2,
                    uuid_row: ref __self_0_3,
                    long_row: ref __self_0_4,
                    date_time_row: ref __self_0_5,
                    small_int_row: ref __self_0_6,
                    bit_row: ref __self_0_7,
                    float_row: ref __self_0_8,
                    real_row: ref __self_0_9,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                        || (*__self_0_6) != (*__self_1_6)
                        || (*__self_0_7) != (*__self_1_7)
                        || (*__self_0_8) != (*__self_1_8)
                        || (*__self_0_9) != (*__self_1_9)
                }
            },
        }
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const by_ref: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("by_ref"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(by_ref())),
};
fn by_ref() -> Result<(), tiberius::error::Error> {
    let body = async {
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
            id: 1,
            var_char_row: "varchar",
            n_var_char_row: "nvarchar",
            long_row: 9999999999999999,
            small_int_row: 2,
            bit_row: true,
            float_row: 10.123123125,
            real_row: 10.5,
            uuid_row: Uuid::parse_str("89e022ce-d3b6-43a7-a359-4618571487a6").unwrap(),
            date_time_row: expected_time,
        };
        {
            match (&&expected, &rows.first().unwrap()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        };
        Ok(())
    };
    #[allow(clippy::expect_used)]
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&by_ref])
}
