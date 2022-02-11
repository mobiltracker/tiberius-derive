#! /bin/bash
export TIBERIUS_TEST_CONNECTION_STRING="server=tcp:localhost,1433;User Id=localuser;Password=123456;IntegratedSecurity=true;TrustServerCertificate=true"
cargo test --package tiberius-derive-tests --test setup_database -- test_datebase_down --exact --ignored > /dev/null
cargo test --package tiberius-derive-tests --test setup_database -- test_database_up --exact --ignored 
cargo test --package tiberius-derive-tests --test setup_database -- test_insert_rows --exact --ignored
cargo test
cargo test --package tiberius-derive-tests --test setup_database -- test_datebase_down --exact --ignored
