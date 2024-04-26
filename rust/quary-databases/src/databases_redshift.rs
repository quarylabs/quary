use async_trait::async_trait;
use quary_core::{
    database_redshift::DatabaseQueryGeneratorRedshift,
    databases::{
        ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
    },
};
use quary_proto::TableAddress;
use sqlx::Error;
use std::fmt::Debug;

use crate::databases_postgres::Postgres;

#[derive(Debug)]
pub struct Redshift {
    postgres: Postgres,
    schema: String,
}

impl Redshift {
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
    ) -> Result<Self, Error> {
        let postgres = Postgres::new(
            host,
            port,
            user,
            password,
            database,
            schema,
            ssl_mode,
            ssl_cert,
            ssl_key,
            ssl_root_cert,
            channel_binding,
            Some(2), // Set extra_float_digits to 2 for Redshift
        )
        .await?;
        Ok(Self {
            postgres,
            schema: schema.to_string(),
        })
    }
}

#[async_trait]
impl DatabaseConnection for Redshift {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = "table_type = 'BASE TABLE' AND table_schema NOT IN ('pg_catalog', 'pg_internal', 'information_schema')";
        self.postgres.list_table_like_query(where_clause).await
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause =
            "table_type = 'VIEW' AND table_schema NOT IN ('pg_catalog', 'pg_internal', 'information_schema')";
        self.postgres.list_table_like_query(where_clause).await
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        self.postgres.list_local_tables().await
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        self.postgres.list_local_views().await
    }

    async fn list_columns(&self, table: &str) -> Result<Vec<ColumnWithDetails>, String> {
        self.postgres.list_columns(table).await
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        self.postgres.exec(query).await
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        self.postgres.query(query).await
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorRedshift::new(
            self.schema.clone(),
            None,
        ))
    }

    async fn table_exists(&self, path: &str) -> Result<Option<bool>, String> {
        let parts: Vec<&str> = path.split('.').collect();
        let (schema, table) = match parts.len() {
            2 => (parts[0].to_string(), parts[1].to_string()),
            _ => (self.schema.clone(), parts[0].to_string()),
        };
        let result = self
            .postgres
            .list_table_like_query(&format!(
                "table_schema = '{schema}' AND table_name = '{table}'"
            ))
            .await?;
        Ok(Some(result.len() > 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use prost::bytes::Bytes;
    use quary_core::database_redshift::DatabaseQueryGeneratorRedshift;
    use quary_core::project::{
        parse_project, project_and_fs_to_sql_for_snapshots, project_and_fs_to_sql_for_views,
    };
    use quary_core::project_tests::return_tests_sql;
    use quary_proto::{File, FileSystem};
    use std::time::SystemTime;

    #[tokio::test]
    #[ignore]
    async fn test_redshift_list_tables_and_views() {
        let quary_postgres = Redshift::new("", None, "", "", "", "", None, None, None, None, None)
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
            columns,
            vec!["id", "name"]
                .into_iter()
                .map(|name| {
                    ColumnWithDetails {
                        name: name.to_string(),
                        is_nullable: Some(true),
                        is_unique: Some(false),
                        ..Default::default()
                    }
                })
                .collect::<Vec<ColumnWithDetails>>()
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
    #[ignore]
    async fn test_redshift_list_columns_in_table() {
        let database = Redshift::new("", None, "", "", "", "", None, None, None, None, None)
            .await
            .unwrap();

        database
            .exec("CREATE SCHEMA IF NOT EXISTS transform")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS transform.test_table (id INTEGER, name_transform VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("CREATE SCHEMA IF NOT EXISTS other_schema")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS other_schema.test_table (id INTEGER NOT NULL UNIQUE, name VARCHAR(255))")
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
                    data_type: None,
                    is_nullable: Some(false),
                    is_unique: Some(true),
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    description: None,
                    data_type: None,
                    is_nullable: Some(true),
                    is_unique: Some(false),
                }
            ]
        );
        let columns = database.list_columns("transform.test_table").await.unwrap();
        assert_eq!(
            columns,
            vec!["id", "name_transform"]
                .into_iter()
                .map(|name| {
                    ColumnWithDetails {
                        name: name.to_string(),
                        is_nullable: Some(true),
                        is_unique: Some(false),
                        ..Default::default()
                    }
                })
                .collect::<Vec<ColumnWithDetails>>()
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_redshift_foreign_relationship_test_with_schema() {
        let database = Redshift::new("", None, "", "", "", "", None, None, None, None, None)
            .await
            .unwrap();

        database
            .exec("CREATE SCHEMA IF NOT EXISTS other_schema")
            .await
            .unwrap();
        database
            .exec("CREATE SCHEMA IF NOT EXISTS transform")
            .await
            .unwrap();
        database
                .exec("CREATE TABLE IF NOT EXISTS other_schema.test_table (id INTEGER, name VARCHAR(255))")
                .await
                .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS transform.test_table (id INTEGER, name VARCHAR(255))")
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

    // TEST FAILS in Redshift: I think this is because we execute the command twice too quickly?
    // Possibly becuase of a table lock that occurs in Redshift when a materialized view is created?
    #[tokio::test]
    #[ignore]
    async fn test_redshift_foreign_relationship_test_with_materialized_view_table() {
        let database = Redshift::new("", None, "", "", "", "", None, None, None, None, None)
            .await
            .unwrap();

        database
            .exec("CREATE SCHEMA IF NOT EXISTS other_schema")
            .await
            .unwrap();
        database
            .exec("CREATE SCHEMA IF NOT EXISTS transform")
            .await
            .unwrap();
        database
                .exec("CREATE TABLE IF NOT EXISTS other_schema.test_table (id INTEGER, name VARCHAR(255))")
                .await
                .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS transform.test_table (id INTEGER, name VARCHAR(255))")
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
    #[ignore]
    async fn test_list_tables_outside_the_schema() {
        let database = Redshift::new("", None, "", "", "", "", None, None, None, None, None)
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

    // TEST FAILS IN REDSHIFT: In Redshift column names are case-insensitive by default.
    #[tokio::test]
    #[ignore]
    async fn test_redshift_list_columns_with_case_sensitive_columns() {
        let database = Redshift::new("", None, "", "", "", "", None, None, None, None, None)
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
                    data_type: None,
                    is_nullable: Some(true),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "\"Name\"".to_string(),
                    description: None,
                    data_type: None,
                    is_nullable: Some(true),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "test".to_string(),
                    description: None,
                    data_type: None,
                    is_nullable: Some(true),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "testtwo".to_string(),
                    description: None,
                    data_type: None,
                    is_nullable: Some(true),
                    is_unique: Some(false),
                }
            ]
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_redshift_snapshots_with_schema() {
        let schema = "analytics";

        let database: Box<dyn DatabaseConnection> = Box::new(
            Redshift::new("", None, "", "", "", schema, None, None, None, None, None)
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
            DatabaseQueryGeneratorRedshift::new(schema.to_string(), Some(system_time));

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
            project_and_fs_to_sql_for_snapshots(&project, &file_system, &db_generator, &database)
                .await
                .unwrap();
        for (_, sql) in snapshots_sql {
            for statement in sql {
                println!("{}", statement.as_str());
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
            DatabaseQueryGeneratorRedshift::new(schema.to_string(), Some(system_time_updated));

        let snapshots_sql = project_and_fs_to_sql_for_snapshots(
            &project,
            &file_system,
            &db_generator_updated,
            &database,
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
    }
}
