#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::needless_lifetimes)]
#![deny(clippy::needless_borrow)]
#![deny(clippy::useless_conversion)]
#![deny(clippy::unwrap_used)]
#![deny(unused_imports)]
#![deny(unused_import_braces)]

extern crate core;

pub mod automatic_branching;
pub mod config;
pub mod database_bigquery;
pub mod database_duckdb;
pub mod database_postgres;
pub mod database_snowflake;
pub mod database_sqlite;
pub mod databases;
pub mod description_table;
pub mod file_system;
pub mod graph;
pub mod inference;
pub mod init;
mod map_helpers;
pub mod models;
pub mod onboarding;
pub mod project;
pub mod project_file;
pub mod project_tests;
pub mod rpc_proto_defined_functions;
pub mod schema_name;
mod seeds;
pub mod sources;
mod sql;
pub mod sql_inference_translator;
pub mod sql_model_finder;
mod test_helpers;
pub mod test_runner;
pub mod tests;
