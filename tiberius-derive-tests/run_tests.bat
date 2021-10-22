@echo off
cargo test test --package tiberius-derive-tests --test setup_database -- test_datebase_down --exact --ignored >NUL
cargo test test --package tiberius-derive-tests --test setup_database -- test_database_up --exact --ignored 
cargo test test --package tiberius-derive-tests --test setup_database -- test_insert_rows --exact --ignored
cargo test
cargo test test --package tiberius-derive-tests --test setup_database -- test_datebase_down --exact --ignored
