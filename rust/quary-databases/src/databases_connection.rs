use crate::databases_bigquery::BigQuery;
use crate::databases_clickhouse::Clickhouse;
use crate::databases_dremio::DremioAuth;
use crate::databases_postgres::Postgres;
use crate::databases_redshift::Redshift;
use crate::databases_snowflake;
use quary_core::database_bigquery::DatabaseQueryGeneratorBigQuery;
use quary_core::database_clickhouse::DatabaseQueryGeneratorClickhouse;
use quary_core::database_dremio::DatabaseQueryGeneratorDremio;
use quary_core::database_duckdb::DatabaseQueryGeneratorDuckDB;
use quary_core::database_postgres::DatabaseQueryGeneratorPostgres;
use quary_core::database_redshift::DatabaseQueryGeneratorRedshift;
use quary_core::database_snowflake::DatabaseQueryGeneratorSnowflake;
use quary_core::database_sqlite::DatabaseQueryGeneratorSqlite;
use quary_core::databases::{DatabaseConnection, DatabaseQueryGenerator};
use quary_proto::connection_config::Config;
use quary_proto::connection_config::Config::{
    BigQuery as BigQueryConfig, Clickhouse as ClickhouseConfig, Dremio as DremioConfig, Duckdb,
    DuckdbInMemory, Postgres as PostgresConfig, Redshift as RedshiftConfig, Snowflake, Sqlite,
    SqliteInMemory,
};
use std::{env, fs};

pub async fn database_from_config(
    config: &quary_proto::ConnectionConfig,
) -> Result<Box<dyn DatabaseConnection>, String> {
    let config = config
        .config
        .clone()
        .ok_or("No config provided".to_string())?;

    match config {
        DuckdbInMemory(config) => {
            let database = crate::databases_duckdb::DuckDB::new_in_memory(config.schema.clone())?;
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
        BigQueryConfig(config) => {
            let google_access_token = env::var("GOOGLE_CLOUD_ACCESS_TOKEN");
            if let Ok(google_access_token) = google_access_token {
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
            let account_identifier = env::var("SNOWSQL_ACCOUNT")
                .map_err(|_| "SNOWSQL_ACCOUNT must be set to connect to Snowflake".to_string())?;
            let warehouse = env::var("SNOWSQL_WAREHOUSE")
                .map_err(|_| "SNOWSQL_WAREHOUSE must be set to connect to Snowflake".to_string())?;
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
        PostgresConfig(config) => {
            let host = env::var("PGHOST")
                .map_err(|_| "PGHOST must be set to connect to Postgres".to_string())?;
            let user = env::var("PGUSER")
                .map_err(|_| "PGUSER must be set to connect to Postgres".to_string())?;
            let password = env::var("PGPASSWORD")
                .map_err(|_| "PGPASSWORD must be set to connect to Postgres".to_string())?;
            let database = env::var("PGDATABASE")
                .map_err(|_| "PGDATABASE must be set to connect to Postgres".to_string())?;

            let port = if let Ok(port) = env::var("PGPORT") {
                Some(port)
            } else {
                None
            };
            let ssl_mode = if let Ok(ssl_mode) = env::var("PGSSLMODE") {
                Some(ssl_mode.to_string())
            } else {
                None
            };
            let ssl_cert = if let Ok(ssl_cert) = env::var("PGSSLCERT") {
                Some(ssl_cert.to_string())
            } else {
                None
            };
            let ssl_key = if let Ok(ssl_key) = env::var("PGSSLKEY") {
                Some(ssl_key.to_string())
            } else {
                None
            };
            let ssl_root_cert = if let Ok(ssl_root_cert) = env::var("PGSSLROOTCERT") {
                Some(ssl_root_cert.to_string())
            } else {
                None
            };
            let channel_binding = if let Ok(channel_binding) = env::var("PGCHANNELBINDING") {
                Some(channel_binding.to_string())
            } else {
                None
            };
            let database = Postgres::new(
                &host,
                port,
                &user,
                &password,
                &database,
                &config.schema,
                ssl_mode,
                ssl_cert,
                ssl_key,
                ssl_root_cert,
                channel_binding,
                None,
            )
            .await
            .map_err(|e| e.to_string())?;
            Ok(Box::new(database))
        }
        ClickhouseConfig(config) => {
            let host = env::var("CLICKHOUSE_HOST")
                .map_err(|_| "CLICKHOUSE_HOST must be set to connect to Clickhouse".to_string())?;
            let port = env::var("CLICKHOUSE_PORT").ok();
            let user = env::var("CLICKHOUSE_USER").ok();
            let password = env::var("CLICKHOUSE_PASSWORD").ok();

            let database = Clickhouse::new(
                &host,
                port.as_deref(),
                user.as_deref(),
                password.as_deref(),
                Some(&config.database),
            )
            .await
            .map_err(|e| e.to_string())?;
            Ok(Box::new(database))
        }
        RedshiftConfig(config) => {
            let host = env::var("RSHOST")
                .map_err(|_| "RSHOST must be set to connect to Redshift".to_string())?;
            let user = env::var("RSUSER")
                .map_err(|_| "RSUSER must be set to connect to Redshift".to_string())?;
            let password = env::var("RSPASSWORD")
                .map_err(|_| "RSPASSWORD must be set to connect to Redshift".to_string())?;
            let database = env::var("RSDATABASE")
                .map_err(|_| "RSDATABASE must be set to connect to Redshift".to_string())?;

            let port = if let Ok(port) = env::var("RSPORT") {
                Some(port)
            } else {
                None
            };
            let ssl_mode = if let Ok(ssl_mode) = env::var("PGSSLMODE") {
                Some(ssl_mode.to_string())
            } else {
                None
            };
            let ssl_cert = if let Ok(ssl_cert) = env::var("PGSSLCERT") {
                Some(ssl_cert.to_string())
            } else {
                None
            };
            let ssl_key = if let Ok(ssl_key) = env::var("PGSSLKEY") {
                Some(ssl_key.to_string())
            } else {
                None
            };
            let ssl_root_cert = if let Ok(ssl_root_cert) = env::var("PGSSLROOTCERT") {
                Some(ssl_root_cert.to_string())
            } else {
                None
            };
            let channel_binding = if let Ok(channel_binding) = env::var("PGCHANNELBINDING") {
                Some(channel_binding.to_string())
            } else {
                None
            };

            let database = Redshift::new(
                &host,
                port,
                &user,
                &password,
                &database,
                &config.schema,
                ssl_mode,
                ssl_cert,
                ssl_key,
                ssl_root_cert,
                channel_binding,
            )
            .await
            .map_err(|e| e.to_string())?;
            Ok(Box::new(database))
        }
        DremioConfig(config) => {
            let host = env::var("DREMIO_HOST")
                .map_err(|_| "DREMIO_HOST must be set to connect to Dremio".to_string())?;
            let port = env::var("DREMIO_PORT")
                .map_err(|_| "DREMIO_PORT must be set to connect to Dremio".to_string())?;
            let use_ssl = env::var("DREMIO_USE_SSL")
                .map_err(|_| "DREMIO_USE_SSL must be set to connect to Dremio".to_string())?;
            let username = env::var("DREMIO_USER")
                .map_err(|_| "DREMIO_USER must be set to connect to Dremio".to_string())?;
            let password = env::var("DREMIO_PASSWORD")
                .map_err(|_| "DREMIO_PASSWORD must be set to connect to Dremio".to_string())?;

            let auth = if let Ok(personal_access_token) = env::var("DREMIO_PERSONAL_ACCESS_TOKEN") {
                DremioAuth::UsernamePersonalAccessToken(username, personal_access_token)
            } else {
                DremioAuth::UsernamePassword(username, password)
            };

            let database = crate::databases_dremio::Dremio::new(
                config,
                auth,
                use_ssl.parse().unwrap(),
                host,
                port,
            )
            .await?;
            Ok(Box::new(database))
        }
    }
}

pub fn database_query_generator_from_config(
    config: quary_proto::ConnectionConfig,
) -> Result<Box<dyn DatabaseQueryGenerator>, String> {
    let config = config.config.ok_or("No config provided".to_string())?;
    match config {
        SqliteInMemory(_) => {
            let database = DatabaseQueryGeneratorSqlite::default();
            Ok(Box::new(database))
        }
        Sqlite(_) => {
            let database = DatabaseQueryGeneratorSqlite::default();
            Ok(Box::new(database))
        }
        BigQueryConfig(config) => {
            let database =
                DatabaseQueryGeneratorBigQuery::new(config.project_id, config.dataset_id);
            Ok(Box::new(database))
        }
        Snowflake(config) => Ok(Box::new(DatabaseQueryGeneratorSnowflake::new(
            config.database,
            config.schema,
        ))),
        Duckdb(config) => Ok(Box::new(DatabaseQueryGeneratorDuckDB::new(
            config.schema,
            None,
        ))),
        DuckdbInMemory(config) => Ok(Box::new(DatabaseQueryGeneratorDuckDB::new(
            config.schema,
            None,
        ))),
        PostgresConfig(config) => Ok(Box::new(DatabaseQueryGeneratorPostgres::new(
            config.schema,
            None,
        ))),
        RedshiftConfig(config) => Ok(Box::new(DatabaseQueryGeneratorRedshift::new(
            config.schema,
            None,
        ))),
        Config::Clickhouse(config) => Ok(Box::new(DatabaseQueryGeneratorClickhouse::new(
            config.database,
            None,
        ))),
        Config::Dremio(config) => Ok(Box::new(DatabaseQueryGeneratorDremio::new(
            config.dremio_space,
            config.dremio_space_folder,
        ))),
    }
}
