use crate::databases_bigquery::BigQuery;
use crate::databases_snowflake;
use quary_core::database_bigquery::DatabaseQueryGeneratorBigQuery;
use quary_core::database_duckdb::DatabaseQueryGeneratorDuckDB;
use quary_core::database_snowflake::DatabaseQueryGeneratorSnowflake;
use quary_core::database_sqlite::DatabaseQueryGeneratorSqlite;
use quary_core::databases::{DatabaseConnection, DatabaseQueryGenerator};
use quary_proto::connection_config::Config::{
    BigQuery as OtherBigQueryConfig, Duckdb, DuckdbInMemory, Snowflake, Sqlite, SqliteInMemory,
};
use std::{env, fs};

pub async fn database_from_config(
    config: &quary_proto::ConnectionConfig,
) -> Result<Box<dyn DatabaseConnection>, String> {
    if let Some(config) = &config.config {
        match config {
            DuckdbInMemory(config) => {
                let database =
                    crate::databases_duckdb::DuckDB::new_in_memory(config.schema.clone())?;
                Ok(Box::new(database))
            }
            Duckdb(config) => {
                let database = crate::databases_duckdb::DuckDB::new_with_file(
                    config.schema.clone(),
                    config.path.as_str(),
                )?;
                Ok(Box::new(database))
            }
            SqliteInMemory(_) => {
                let database = crate::databases_sqlite::Sqlite::new_in_memory()
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Box::new(database))
            }
            Sqlite(config) => {
                let path = format!("./{}", config.path.as_str(),);
                if fs::metadata(path.clone()).is_err() {
                    fs::File::create(path.clone().as_str())
                        .map_err(|e| format!("creating file at {}: {:?}", path, e))?;
                }
                let database = crate::databases_sqlite::Sqlite::new_with_file(config.path.as_str())
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(Box::new(database))
            }
            OtherBigQueryConfig(config) => {
                let google_access_token = env::var("GOOGLE_CLOUD_ACCESS_TOKEN");
                if let Ok(google_access_token) = google_access_token {
                    println!("Using GOOGLE_CLOUD_ACCESS_TOKEN");
                    let database = BigQuery::new(
                        config.project_id.clone(),
                        config.dataset_id.clone(),
                        Some(google_access_token.to_string()),
                    )
                    .await?;
                    Ok(Box::new(database))
                } else {
                    let database =
                        BigQuery::new(config.project_id.clone(), config.dataset_id.clone(), None)
                            .await?;
                    Ok(Box::new(database))
                }
            }
            Snowflake(config) => {
                let account_identifier = env::var("SNOWSQL_ACCOUNT").map_err(|_| {
                    "SNOWSQL_ACCOUNT must be set to connect to Snowflake".to_string()
                })?;
                let warehouse = env::var("SNOWSQL_WAREHOUSE").map_err(|_| {
                    "SNOWSQL_WAREHOUSE must be set to connect to Snowflake".to_string()
                })?;
                let username = env::var("SNOWSQL_USER")
                    .map_err(|_| "SNOWSQL_USER must be set to connect to Snowflake".to_string())?;
                let role = env::var("SNOWSQL_ROLE")
                    .map_err(|_| "SNOWSQL_ROLE must be set to connect to Snowflake".to_string())?;
                let password = env::var("SNOWSQL_PWD")
                    .map_err(|_| "SNOWSQL_PWD must be set to connect to Snowflake".to_string())?;

                let database = databases_snowflake::Snowflake::new(
                    account_identifier.as_str(),
                    warehouse.as_str(),
                    &config.database,
                    &config.schema,
                    username.as_str(),
                    Some(role.as_str()),
                    password.as_str(),
                )?;
                Ok(Box::new(database))
            }
        }
    } else {
        Err("No config provided".to_string())
    }
}

pub fn database_query_generator_from_config(
    config: quary_proto::ConnectionConfig,
) -> Result<Box<dyn DatabaseQueryGenerator>, String> {
    match config.config {
        Some(SqliteInMemory(_)) => {
            let database = DatabaseQueryGeneratorSqlite::default();
            Ok(Box::new(database))
        }
        Some(Sqlite(_)) => {
            let database = DatabaseQueryGeneratorSqlite::default();
            Ok(Box::new(database))
        }
        Some(OtherBigQueryConfig(config)) => {
            let database =
                DatabaseQueryGeneratorBigQuery::new(config.project_id, config.dataset_id);
            Ok(Box::new(database))
        }
        Some(Snowflake(config)) => Ok(Box::new(DatabaseQueryGeneratorSnowflake::new(
            config.database,
            config.schema,
        ))),
        Some(Duckdb(config)) => Ok(Box::new(DatabaseQueryGeneratorDuckDB::new(config.schema))),
        Some(DuckdbInMemory(config)) => {
            Ok(Box::new(DatabaseQueryGeneratorDuckDB::new(config.schema)))
        }
        _ => Err("not implemented".to_string()),
    }
}
