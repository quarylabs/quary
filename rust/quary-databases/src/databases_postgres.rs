use async_trait::async_trait;
use chrono::{DateTime, Utc};
use quary_core::database_postgres::DatabaseQueryGeneratorPostgres;
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::TableAddress;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgRow};
use sqlx::types::BigDecimal;
use sqlx::{Column, Pool, Row};
use sqlx::{Error, TypeInfo};
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub struct Postgres {
    pool: Pool<sqlx::Postgres>,
    schema: String,
}

impl Postgres {
    // TODO This should be a builder pattern or something else
    pub async fn new(
        host: &str,
        port: Option<String>,
        user: &str,
        password: &str,
        database: &str,
        schema: &str,
        ssl_mode: Option<String>,
        ssl_cert: Option<String>,
        ssl_key: Option<String>,
        ssl_root_cert: Option<String>,
        channel_binding: Option<String>,
        extra_float_digits: Option<i8>,
    ) -> Result<Self, Error> {
        let params = HashMap::from([
            ("sslmode", ssl_mode),
            ("sslcert", ssl_cert),
            ("sslkey", ssl_key),
            ("sslrootcert", ssl_root_cert),
            ("channel_binding", channel_binding),
        ])
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect::<HashMap<&str, String>>()
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>();

        let params = if params.is_empty() {
            None
        } else {
            Some(format!("?{}", params.join("&")))
        };
        let port = port.unwrap_or("5432".to_string());

        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}{}",
            user,
            password,
            host,
            port,
            database,
            params.unwrap_or("".to_string())
        );

        let options = PgConnectOptions::from_str(connection_string.as_str())?;
        let options = if let Some(extra_float_digits) = extra_float_digits {
            options.extra_float_digits(extra_float_digits)
        } else {
            options
        };
        let pool = PgPoolOptions::new().connect_with(options).await?;
        Ok(Self {
            pool,
            schema: schema.to_string(),
        })
    }
}

impl Postgres {
    pub async fn list_table_like_query(
        &self,
        where_clause: &str,
    ) -> Result<Vec<TableAddress>, String> {
        let query = format!(
            "SELECT
            CASE
                WHEN table_schema <> lower(table_schema) THEN '\"' || table_schema || '\"'
                ELSE table_schema
            END AS table_schema,
            CASE
                WHEN table_name <> lower(table_name) THEN '\"' || table_name || '\"'
                ELSE table_name
            END AS table_name
        FROM information_schema.tables
        WHERE {}
        ORDER BY table_schema, table_name",
            where_clause
        );

        let rows = sqlx::query(query.as_str())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|row| {
                let table_schema: String = row
                    .try_get(0)
                    .map_err(|e| format!("Error getting table schema: {}", e))?;
                let table_name: String = row
                    .try_get(1)
                    .map_err(|e| format!("Error getting table name: {}", e))?;

                Ok(TableAddress {
                    name: table_name.clone(),
                    full_path: format!("{}.{}", table_schema, table_name),
                })
            })
            .collect()
    }
}

#[async_trait]
impl DatabaseConnection for Postgres {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = "table_type = 'BASE TABLE' AND table_schema != 'information_schema' AND table_schema != 'pg_catalog'";
        self.list_table_like_query(where_clause).await
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = "table_type = 'VIEW' AND table_schema != 'information_schema' AND table_schema != 'pg_catalog'";
        self.list_table_like_query(where_clause).await
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = format!(
            "table_schema = '{}' AND table_type = 'BASE TABLE'",
            self.schema
        );
        self.list_table_like_query(where_clause.as_str()).await
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = format!("table_schema = '{}' AND table_type = 'VIEW'", self.schema);
        self.list_table_like_query(where_clause.as_str()).await
    }

    async fn list_columns(&self, table: &str) -> Result<Vec<ColumnWithDetails>, String> {
        let (schema, table) = match table.split('.').collect::<Vec<&str>>().as_slice() {
            [schema, table] => Ok((schema.to_string(), table.to_string())),
            [table] => Ok((self.schema.to_string(), table.to_string())),
            _ => Err(format!(
                "Table name {} does not contain the expected schema",
                table
            )),
        }?;

        let rows = sqlx::query(&format!(
            "
        SELECT
            CASE
                WHEN c.column_name <> lower(c.column_name) THEN '\"' || c.column_name || '\"'
                ELSE c.column_name
            END AS column_name,
            pgd.description AS column_comment,
            c.is_nullable,
            CASE
                WHEN tc.constraint_type = 'UNIQUE' THEN 'YES'
                ELSE 'NO'
            END AS is_unique,
            c.data_type
        FROM
            information_schema.columns c
        LEFT JOIN
            pg_catalog.pg_statio_all_tables AS st ON c.table_schema = st.schemaname AND c.table_name = st.relname
        LEFT JOIN
            pg_catalog.pg_description pgd ON pgd.objoid = st.relid AND pgd.objsubid = c.ordinal_position
        LEFT JOIN
            information_schema.key_column_usage kcu ON c.table_schema = kcu.table_schema
            AND c.table_name = kcu.table_name
            AND c.column_name = kcu.column_name
        LEFT JOIN
            information_schema.table_constraints tc ON kcu.constraint_schema = tc.constraint_schema
            AND kcu.constraint_name = tc.constraint_name
            AND tc.constraint_type = 'UNIQUE'
        WHERE
            c.table_name = '{}' AND c.table_schema = '{}'
        ORDER BY
            c.ordinal_position
        ",
            table,
            schema,
        ))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let columns = rows
            .into_iter()
            .map(|row| {
                let description: Option<String> = row.get(1);
                let is_nullable: String = row.get(2);
                let is_unique: String = row.get(3);
                let data_type: String = row.get(4);

                ColumnWithDetails {
                    name: row.get(0),
                    description,
                    data_type: Some(data_type),
                    is_nullable: Some(is_nullable == "YES"),
                    is_unique: Some(is_unique == "YES"),
                }
            })
            .collect();

        Ok(columns)
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        let query = sqlx::query(query);
        query.execute(&self.pool).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let query_builder = sqlx::query(query);

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| QueryError::new(e.to_string(), query.to_string()))?;

        if rows.is_empty() {
            return Ok(QueryResult {
                columns: Vec::new(),
                rows: Vec::new(),
            });
        }

        let columns = rows[0]
            .columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect::<Vec<String>>();

        let mut rows_vec = Vec::<Vec<String>>::new();

        fn convert_value_to_string(row: &PgRow, i: usize) -> Result<String, Error> {
            let type_name = row.column(i).type_info().name();
            let value: Option<String> = match type_name {
                "INT4" => {
                    row.try_get::<Option<i32>, _>(i)?.map(|v| v.to_string())
                }
                "INT8" => {
                    row.try_get::<Option<i64>, _>(i)?.map(|v| v.to_string())
                }
                "FLOAT8" => {
                    row.try_get::<Option<f64>, _>(i)?.map(|v| v.to_string())
                }
                "BOOL" => {
                    row.try_get::<Option<bool>, _>(i)?.map(|v| v.to_string())
                }
                "TIMESTAMP" => {
                    row
                        .try_get::<Option<chrono::NaiveDateTime>, _>(i)?
                        .map(|v| v.format("%Y-%m-%dT%H:%M:%S").to_string())
                }
                "TIMESTAMPTZ" => {
                    row.try_get::<Option<DateTime<Utc>>, _>(i)?
                        .map(|v| v.to_rfc3339())
                }
                "TEXT" => {
                    row.try_get::<Option<String>, _>(i)?
                }
                "VARCHAR" => {
                   row.try_get::<Option<String>, _>(i)?
                }
                "DATE" => {
                    row
                        .try_get::<Option<chrono::NaiveDate>, _>(i)?
                        .map(|v| v.format("%Y-%m-%d").to_string())
                }
                "NUMERIC" => {
                    row
                        .try_get::<Option<BigDecimal>, _>(i)?
                        .map(|v| v.to_string())
                }
                _ => Some(format!("Unsupported type: {}", type_name)),
            };
            match value {
                Some(value) => Ok(value),
                None => Ok("NULL".to_string()),
            }
        }

        for row in rows {
            let mut row_vec = Vec::new();
            for i in 0..row.len() {
                let value: String = convert_value_to_string(&row, i)
                    .map_err(|e| QueryError::new(e.to_string(), query.to_string()))?;
                row_vec.push(value);
            }

            rows_vec.push(row_vec);
        }

        Ok(QueryResult {
            columns: columns.into_iter().map(|c| (c, None)).collect(),
            rows: rows_vec,
        })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorPostgres::new(
            self.schema.clone(),
            None,
        ))
    }

    async fn table_exists(&self, _path: &str) -> Result<Option<bool>, String> {
        Ok(None) // not implemented
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use prost::bytes::Bytes;
    use quary_core::project::{
        parse_project, project_and_fs_to_sql_for_snapshots, project_and_fs_to_sql_for_views,
    };
    use quary_core::project_tests::return_tests_sql;
    use quary_proto::{File, FileSystem};
    use std::time::SystemTime;
    use testcontainers::{clients, RunnableImage};
    use testcontainers_modules::postgres::Postgres as TestcontainersPostgres;

    #[tokio::test]
    async fn run_build_with_project_twice() {
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let quary_postgres = Postgres::new(
            "localhost",
            Some(pg_port.to_string()),
            "postgres",
            "postgres",
            "postgres",
            "public",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let filesystem = FileSystem {
            files: vec![
                ("quary.yaml", "postgres: {schema: public}"),
                ("models/test_model.sql", "SELECT * FROM q.test_seed"),
                ("seeds/test_seed.csv", "id,name\n1,test\n2,rubbish"),
            ]
            .into_iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    File {
                        name: k.to_string(),
                        contents: Bytes::from(v),
                    },
                )
            })
            .collect(),
        };

        let project = parse_project(&filesystem, &quary_postgres.query_generator(), "")
            .await
            .unwrap();
        let sqls = project_and_fs_to_sql_for_views(
            &project,
            &filesystem,
            &quary_postgres.query_generator(),
            false,
            false,
        )
        .await
        .unwrap();

        for sql in &sqls {
            for sql in &sql.1 {
                quary_postgres.exec(&sql).await.unwrap();
            }
        }
        // Run twice
        for sql in &sqls {
            for sql in &sql.1 {
                quary_postgres.exec(&sql).await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn test_postgres_list_tables_and_views() {
        // Start a PostgreSQL container
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let quary_postgres = Postgres::new(
            "localhost",
            Some(pg_port.to_string()),
            "postgres",
            "postgres",
            "postgres",
            "public",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        quary_postgres
            .exec("CREATE TABLE wrong_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        quary_postgres
            .exec("CREATE TABLE test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        quary_postgres
            .exec("INSERT INTO test_table VALUES (1, 'test')")
            .await
            .unwrap();
        quary_postgres
            .exec("INSERT INTO test_table VALUES (2, 'rubbish')")
            .await
            .unwrap();
        quary_postgres
            .exec("CREATE VIEW test_view AS SELECT * FROM test_table")
            .await
            .unwrap();
        quary_postgres
            .exec("CREATE VIEW wrong_view AS SELECT * FROM test_table")
            .await
            .unwrap();

        let tables = quary_postgres.list_local_tables().await.unwrap();
        assert_eq!(
            tables,
            vec![
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "public.test_table".to_string(),
                },
                TableAddress {
                    name: "wrong_table".to_string(),
                    full_path: "public.wrong_table".to_string(),
                },
            ]
        );

        let views = quary_postgres.list_local_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "public.test_view".to_string(),
                },
                TableAddress {
                    name: "wrong_view".to_string(),
                    full_path: "public.wrong_view".to_string(),
                },
            ]
        );

        let columns = quary_postgres.list_columns("test_table").await.unwrap();
        assert_eq!(
            vec![
                ColumnWithDetails {
                    name: "id".to_string(),
                    description: None,
                    data_type: Some("integer".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    description: None,
                    data_type: Some("character varying".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(false),
                }
            ],
            columns
        );

        let result = quary_postgres
            .query("SELECT * FROM test_table")
            .await
            .unwrap();
        assert_eq!(
            result
                .columns
                .iter()
                .map(|(column, _)| column)
                .collect::<Vec<_>>(),
            vec!["id", "name"]
        );
        assert_eq!(result.rows, vec![vec!["1", "test"], vec!["2", "rubbish"]]);
    }

    #[tokio::test]
    async fn list_columns_in_table() {
        // Start a PostgreSQL container
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let database = Postgres::new(
            "localhost",
            Some(pg_port.to_string()),
            "postgres",
            "postgres",
            "postgres",
            "transform",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        database.exec("CREATE SCHEMA transform").await.unwrap();
        database
            .exec("CREATE TABLE transform.test_table (id INTEGER, name_transform VARCHAR(255))")
            .await
            .unwrap();
        database.exec("CREATE SCHEMA other_schema").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER NOT NULL UNIQUE, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("COMMENT ON COLUMN other_schema.test_table.id IS 'test comment'")
            .await
            .unwrap();

        let columns = database
            .list_columns("other_schema.test_table")
            .await
            .unwrap();
        assert_eq!(
            columns,
            vec![
                ColumnWithDetails {
                    name: "id".to_string(),
                    description: Some("test comment".to_string()),
                    data_type: Some("integer".to_string()),
                    is_nullable: Some(false),
                    is_unique: Some(true),
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    description: None,
                    data_type: Some("character varying".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(false),
                }
            ]
        );
        let columns = database.list_columns("transform.test_table").await.unwrap();
        assert_eq!(
            columns,
            vec![("id", "integer"), ("name_transform", "character varying"),]
                .into_iter()
                .map(|(name, data_type)| {
                    ColumnWithDetails {
                        name: name.to_string(),
                        is_nullable: Some(true),
                        is_unique: Some(false),
                        data_type: Some(data_type.to_string()),
                        ..Default::default()
                    }
                })
                .collect::<Vec<ColumnWithDetails>>()
        );
    }

    #[tokio::test]
    async fn test_postgres_foreign_relationship_test_with_schema() {
        // Start a PostgreSQL container
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let database = Postgres::new(
            "localhost",
            Some(pg_port.to_string()),
            "postgres",
            "postgres",
            "postgres",
            "transform",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        database.exec("CREATE SCHEMA other_schema").await.unwrap();
        database.exec("CREATE SCHEMA transform").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE transform.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO transform.test_table VALUES (3, 'test_3'), (4, 'rubbish_rubiish')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                ("quary.yaml", "postgres: {schema: transform}"),
                ("models/test_model.sql", "SELECT id FROM q.test_source"),
                (
                    "models/test_model_same_schema.sql",
                    "SELECT id FROM q.test_source_same_schema",
                ),
                ("models/test_model_out.sql", "SELECT id FROM q.test_model"),
                (
                    "models/schema.yaml",
                    "
sources:
    - name: test_source
      path: other_schema.test_table
    - name: test_source_same_schema
      path: transform.test_table
models:
  - name: test_model_out
    columns:
      - name: id
        tests:
          - type: relationship
            info:
              column: id
              model: test_model
          - type: relationship
            info:
              column: id
              model: test_source
  - name: test_model_same_schema
    columns:
      - name: id
        tests:
          - type: relationship
            info:
              column: id
              model: test_source_same_schema
                    ",
                ),
            ]
            .into_iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    File {
                        name: k.to_string(),
                        contents: Bytes::from(v),
                    },
                )
            })
            .collect(),
        };

        let project = parse_project(&file_system, &database.query_generator(), "")
            .await
            .unwrap();

        let tests = return_tests_sql(
            &database.query_generator(),
            &project,
            &file_system,
            true,
            None,
            None,
        )
        .await
        .unwrap();
        let tests = tests.iter().collect::<Vec<_>>();

        assert!(!tests.is_empty());

        for (name, test) in tests.iter() {
            let results = database.query(test).await.unwrap();

            assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
        }
    }

    #[tokio::test]
    async fn test_postgres_foreign_relationship_test_with_materialized_view_table() {
        // Start a PostgreSQL container
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let database = Postgres::new(
            "localhost",
            Some(pg_port.to_string()),
            "postgres",
            "postgres",
            "postgres",
            "transform",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        database.exec("CREATE SCHEMA other_schema").await.unwrap();
        database.exec("CREATE SCHEMA transform").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE transform.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO transform.test_table VALUES (3, 'test_3'), (4, 'rubbish_rubiish')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                ("quary.yaml", "postgres: {schema: transform}"),
                ("models/test_model.sql", "SELECT id FROM q.test_source"),
                (
                    "models/test_model_same_schema.sql",
                    "SELECT id FROM q.test_source_same_schema",
                ),
                ("models/test_model_out.sql", "SELECT id FROM q.test_model"),
                (
                    "models/schema.yaml",
                    "
sources:
    - name: test_source
      path: other_schema.test_table
    - name: test_source_same_schema
      path: transform.test_table
models:
  - name: test_model_out
    materialization: table
    columns:
      - name: id
        tests:
          - type: relationship
            info:
              column: id
              model: test_model
          - type: relationship
            info:
              column: id
              model: test_source
  - name: test_model_same_schema
    materialization: materialized_view
    columns:
      - name: id
        tests:
          - type: relationship
            info:
              column: id
              model: test_source_same_schema
                    ",
                ),
            ]
            .into_iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    File {
                        name: k.to_string(),
                        contents: Bytes::from(v),
                    },
                )
            })
            .collect(),
        };

        let project = parse_project(&file_system, &database.query_generator(), "")
            .await
            .unwrap();

        let sqls = project_and_fs_to_sql_for_views(
            &project,
            &file_system,
            &database.query_generator(),
            false,
            false,
        )
        .await
        .unwrap();
        for sql in &sqls {
            for sql in &sql.1 {
                database.exec(&sql).await.unwrap();
            }
        }
        // Run twice
        for sql in &sqls {
            for sql in &sql.1 {
                database.exec(&sql).await.unwrap();
            }
        }

        let tests = return_tests_sql(
            &database.query_generator(),
            &project,
            &file_system,
            false,
            None,
            None,
        )
        .await
        .unwrap();
        let tests = tests.iter().collect::<Vec<_>>();

        assert!(!tests.is_empty());

        for (name, test) in tests.iter() {
            let results = database.query(test).await.unwrap();

            assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
        }
    }

    #[tokio::test]
    async fn test_list_tables_outside_the_schema() {
        // Start a PostgreSQL container
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let database = Postgres::new(
            "localhost",
            Some(pg_port.to_string()),
            "postgres",
            "postgres",
            "postgres",
            "transform",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        database.exec("CREATE SCHEMA other_schema").await.unwrap();
        database.exec("CREATE SCHEMA transform").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE transform.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("CREATE VIEW transform.test_view AS SELECT * FROM transform.test_table")
            .await
            .unwrap();
        database
            .exec("CREATE VIEW other_schema.test_view AS SELECT * FROM other_schema.test_table")
            .await
            .unwrap();

        let tables = database.list_tables().await.unwrap();
        assert_eq!(
            tables,
            vec![
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "other_schema.test_table".to_string(),
                },
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "transform.test_table".to_string(),
                },
            ]
        );

        let views = database.list_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "other_schema.test_view".to_string(),
                },
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "transform.test_view".to_string(),
                },
            ]
        );
    }

    #[tokio::test]
    async fn test_list_columns_with_case_sensitive_columns() {
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let database = Postgres::new(
            "localhost",
            Some(pg_port.to_string()),
            "postgres",
            "postgres",
            "postgres",
            "transform",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        database.exec("CREATE SCHEMA transform").await.unwrap();
        database
            .exec("CREATE TABLE transform.test_table (\"ID\" INTEGER, \"Name\" VARCHAR(255), test VARCHAR(255), TESTTWO VARCHAR(255))")
            .await
            .unwrap();

        let columns = database.list_columns("transform.test_table").await.unwrap();
        assert_eq!(
            columns,
            vec![
                ColumnWithDetails {
                    name: "\"ID\"".to_string(),
                    description: None,
                    data_type: Some("integer".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "\"Name\"".to_string(),
                    description: None,
                    data_type: Some("character varying".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "test".to_string(),
                    description: None,
                    data_type: Some("character varying".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "testtwo".to_string(),
                    description: None,
                    data_type: Some("character varying".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(false),
                }
            ]
        );
    }

    #[tokio::test]
    async fn test_snapshot_with_no_time_override() {
        let schema = "analytics";
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);
        let database: Box<dyn DatabaseConnection> = Box::new(
            Postgres::new(
                "localhost",
                Some(pg_port.to_string()),
                "postgres",
                "postgres",
                "postgres",
                schema,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap(),
        );

        database.exec("CREATE SCHEMA analytics").await.unwrap();
        database.exec("CREATE SCHEMA jaffle_shop").await.unwrap();

        // Create orders table
        database
            .exec("CREATE TABLE jaffle_shop.raw_orders (order_id INTEGER, status VARCHAR(255), updated_at TIMESTAMP)")
            .await
            .unwrap();

        // Insert some initial data
        database
            .exec("INSERT INTO jaffle_shop.raw_orders VALUES (1, 'in_progress', '2023-01-01 00:00:00'), (2, 'completed', '2023-01-01 00:00:00')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                ("quary.yaml", "duckdbInMemory: {schema: analytics}"),
                (
                    "models/orders_snapshot.snapshot.sql",
                    "SELECT * FROM q.raw_orders",
                ),
                (
                    "models/schema.yaml",
                    "
sources:
  - name: raw_orders
    path: jaffle_shop.raw_orders
snapshots:
  - name: orders_snapshot
    unique_key: order_id
    strategy:
      timestamp:
        updated_at: updated_at
",
                ),
            ]
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    File {
                        name: k.to_string(),
                        contents: Bytes::from(v.to_string()),
                    },
                )
            })
            .collect(),
        };

        let project = parse_project(
            &file_system,
            &DatabaseQueryGeneratorPostgres::new(schema.to_string(), None),
            "",
        )
        .await
        .unwrap();

        let snapshots_sql = project_and_fs_to_sql_for_snapshots(
            &project,
            &file_system,
            &DatabaseQueryGeneratorPostgres::new(schema.to_string(), None),
            database.as_ref(),
        )
        .await
        .unwrap();
        for (_, sql) in snapshots_sql {
            for statement in sql {
                database.exec(statement.as_str()).await.unwrap()
            }
        }

        // assert the data has been created correctly in the snapshot table
        let data = database.query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM analytics.orders_snapshot").await.unwrap();
        assert_eq!(
            data.columns
                .iter()
                .map(|(column, _)| column)
                .collect::<Vec<_>>(),
            vec![
                "order_id",
                "status",
                "updated_at",
                "quary_valid_from",
                "quary_valid_to",
                "quary_scd_id"
            ]
        );
        assert_eq!(data.rows.len(), 2);
    }

    #[tokio::test]
    async fn test_snapshots_with_schema() {
        let schema = "analytics";

        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);
        let database: Box<dyn DatabaseConnection> = Box::new(
            Postgres::new(
                "localhost",
                Some(pg_port.to_string()),
                "postgres",
                "postgres",
                "postgres",
                schema,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap(),
        );

        database.exec("CREATE SCHEMA analytics").await.unwrap();
        database.exec("CREATE SCHEMA jaffle_shop").await.unwrap();

        let datetime_str = "2023-01-01 01:00:00";

        // Parse the string into a NaiveDateTime
        let naive_datetime =
            NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap();

        // Convert NaiveDateTime to DateTime<Utc>
        let datetime_utc = DateTime::<Utc>::from_utc(naive_datetime, Utc);

        // Convert DateTime<Utc> to SystemTime
        let system_time = SystemTime::from(datetime_utc);

        let db_generator =
            DatabaseQueryGeneratorPostgres::new(schema.to_string(), Some(system_time));

        // Create orders table
        database
            .exec("CREATE TABLE jaffle_shop.raw_orders (order_id INTEGER, status VARCHAR(255), updated_at TIMESTAMP)")
            .await
            .unwrap();

        // Insert some initial data
        database
            .exec("INSERT INTO jaffle_shop.raw_orders VALUES (1, 'in_progress', '2023-01-01 00:00:00'), (2, 'completed', '2023-01-01 00:00:00')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                ("quary.yaml", "postgres: {schema: analytics}"),
                (
                    "models/orders_snapshot.snapshot.sql",
                    "SELECT * FROM q.raw_orders",
                ),
                (
                    "models/schema.yaml",
                    "
sources:
  - name: raw_orders
    path: jaffle_shop.raw_orders
snapshots:
  - name: orders_snapshot
    unique_key: order_id
    strategy:
      timestamp:
        updated_at: updated_at
",
                ),
            ]
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    File {
                        name: k.to_string(),
                        contents: Bytes::from(v.to_string()),
                    },
                )
            })
            .collect(),
        };

        let project = parse_project(&file_system, &db_generator, "")
            .await
            .unwrap();

        let snapshots_sql =
            project_and_fs_to_sql_for_snapshots(&project, &file_system, &db_generator, database.as_ref())
                .await
                .unwrap();
        for (_, sql) in snapshots_sql {
            for statement in sql {
                database.exec(statement.as_str()).await.unwrap()
            }
        }

        // assert the data has been created correctly in the snapshot table
        let data = database
            .query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM analytics.orders_snapshot ORDER BY order_id, quary_valid_from")
            .await
            .unwrap();

        assert_eq!(
            data.columns
                .iter()
                .map(|(column, _)| column)
                .collect::<Vec<_>>(),
            vec![
                "order_id",
                "status",
                "updated_at",
                "quary_valid_from",
                "quary_valid_to",
                "quary_scd_id"
            ]
        );
        assert_eq!(
            data.rows,
            vec![
                vec![
                    "1",
                    "in_progress",
                    "2023-01-01T00:00:00",
                    "2023-01-01T01:00:00+00:00",
                    "NULL",
                    "77f50225cf5a52d15fecaa449be2dcc4"
                ],
                vec![
                    "2",
                    "completed",
                    "2023-01-01T00:00:00",
                    "2023-01-01T01:00:00+00:00",
                    "NULL",
                    "3bb5cc6bb5b432df7712d067f57a3780"
                ],
            ]
        );

        database
            .exec("UPDATE jaffle_shop.raw_orders SET status = 'completed', updated_at = CAST('2023-01-01 02:00:00' AS TIMESTAMP) WHERE order_id = 1")
            .await
            .unwrap();

        let datetime_str_updated = "2023-01-01 03:00:00";

        // Parse the string into a NaiveDateTime
        let naive_datetime_updated =
            NaiveDateTime::parse_from_str(datetime_str_updated, "%Y-%m-%d %H:%M:%S").unwrap();

        // Convert NaiveDateTime to DateTime<Utc>
        let datetime_utc_updated = DateTime::<Utc>::from_utc(naive_datetime_updated, Utc);

        // Convert DateTime<Utc> to SystemTime
        let system_time_updated = SystemTime::from(datetime_utc_updated);

        let db_generator_updated =
            DatabaseQueryGeneratorPostgres::new(schema.to_string(), Some(system_time_updated));

        let snapshots_sql = project_and_fs_to_sql_for_snapshots(
            &project,
            &file_system,
            &db_generator_updated,
            database.as_ref(),
        )
        .await
        .unwrap();

        for (_, sql) in &snapshots_sql {
            for statement in sql {
                database.exec(statement.as_str()).await.unwrap()
            }
        }

        // assert the data has been created correctly in the snapshot table
        let data = database
            .query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM analytics.orders_snapshot ORDER BY order_id, quary_valid_from")
            .await
            .unwrap();

        assert_eq!(
            data.columns
                .iter()
                .map(|(column, _)| column)
                .collect::<Vec<_>>(),
            vec![
                "order_id",
                "status",
                "updated_at",
                "quary_valid_from",
                "quary_valid_to",
                "quary_scd_id"
            ]
        );
        assert_eq!(
            data.rows,
            vec![
                vec![
                    "1",
                    "in_progress",
                    "2023-01-01T00:00:00",
                    "2023-01-01T01:00:00+00:00",
                    "2023-01-01T03:00:00+00:00",
                    "77f50225cf5a52d15fecaa449be2dcc4"
                ],
                vec![
                    "1",
                    "completed",
                    "2023-01-01T02:00:00",
                    "2023-01-01T03:00:00+00:00",
                    "NULL",
                    "f5c7798e30814925cd1a61e9e5ef6683"
                ],
                vec![
                    "2",
                    "completed",
                    "2023-01-01T00:00:00",
                    "2023-01-01T01:00:00+00:00",
                    "NULL",
                    "3bb5cc6bb5b432df7712d067f57a3780"
                ],
            ]
        );

        let columns = database
            .list_columns("analytics.orders_snapshot")
            .await
            .unwrap();
        assert_eq!(6, columns.len());
        assert_eq!(
            Some("timestamp with time zone".to_string()),
            columns
                .iter()
                .find(|c| c.name == "quary_valid_from")
                .unwrap()
                .data_type
        );
        assert_eq!(
            Some("timestamp with time zone".to_string()),
            columns
                .iter()
                .find(|c| c.name == "quary_valid_to")
                .unwrap()
                .data_type
        );
    }
}
