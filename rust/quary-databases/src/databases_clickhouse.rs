use std::fmt::Debug;

use async_trait::async_trait;
use clickhouse_rs::types::SqlType;
use clickhouse_rs::Pool;
use futures_util::StreamExt;

use quary_core::database_clickhouse::DatabaseQueryGeneratorClickhouse;
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::TableAddress;

#[derive(Debug)]
pub struct Clickhouse {
    pool: Pool,
    database: String,
}

impl Clickhouse {
    // TODO CLEAN UP INITIALISER
    pub async fn new(
        host: &str,
        port: Option<&str>,
        user: Option<&str>,
        password: Option<&str>,
        database: Option<&str>,
    ) -> Result<Self, String> {
        let port = port.unwrap_or("9000");
        let user = user.unwrap_or("default");
        let database = database.unwrap_or("default");

        let credentials = match password {
            Some(password) => format!("{}:{}", user, password),
            None => user.to_string(),
        };

        let host = format!("tcp://{}@{}:{}", credentials, host, port);

        let pool = Pool::new(host);
        let mut client = pool
            .get_handle()
            .await
            .map_err(|e| format!("Failed to get client handle: {}", e))?;
        client
            .ping()
            .await
            .map_err(|e| format!("Failed to ping Clickhouse server: {}", e))?;

        Ok(Self {
            pool,
            database: database.to_string(),
        })
    }
}

#[async_trait]
impl DatabaseConnection for Clickhouse {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let query = "SELECT database, name FROM system.tables WHERE engine !='View' AND database != 'system' ORDER BY database, name";
        let results = self
            .query(query)
            .await
            .map_err(|e| format!("Failed to list tables: {:?}", e))?
            .rows
            .into_iter()
            .map(|row| {
                let database = row[0].clone();
                let name = row[1].clone();
                TableAddress {
                    name: name.clone(),
                    full_path: format!("{}.{}", database, name),
                }
            })
            .collect::<Vec<_>>();
        Ok(results)
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let query = "SELECT database, name FROM system.tables WHERE engine ='View' AND database != 'system' AND database != 'INFORMATION_SCHEMA' AND database != 'information_schema' ORDER BY database, name";
        let results = self
            .query(query)
            .await
            .map_err(|e| format!("Failed to list tables: {:?}", e))?
            .rows
            .into_iter()
            .map(|row| {
                let database = row[0].clone();
                let name = row[1].clone();
                TableAddress {
                    name: name.clone(),
                    full_path: format!("{}.{}", database, name),
                }
            })
            .collect::<Vec<_>>();
        Ok(results)
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let query = format!(
            "SELECT name FROM system.tables WHERE database = '{}' AND engine !='View' ORDER BY name",
            self.database
        );
        let results = self
            .query(&query)
            .await
            .map_err(|e| format!("Failed to list tables: {:?}", e))?
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[0].clone(),
                full_path: format!("{}.{}", self.database, row[0].clone()),
            })
            .collect::<Vec<_>>();
        Ok(results)
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let query = format!(
            "SELECT name FROM system.tables WHERE database = '{}' AND engine ='View'",
            self.database
        );
        let results = self
            .query(&query)
            .await
            .map_err(|e| format!("Failed to list tables: {:?}", e))?
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[0].clone(),
                full_path: format!("{}.{}", self.database, row[0].clone()),
            })
            .collect::<Vec<_>>();
        Ok(results)
    }

    async fn list_columns(&self, path: &str) -> Result<Vec<ColumnWithDetails>, String> {
        let path = match path.split('.').collect::<Vec<_>>()[..] {
            [name] => Ok::<(&str, &str), String>((self.database.as_str(), name)),
            [schema, name] => Ok((schema, name)),
            _ => return Err(format!("Invalid path: {}", path)),
        }?;
        let query = format!(
            "
SELECT
  name,
  type,
  comment
FROM system.columns
WHERE database = '{}' AND table = '{}'
ORDER BY position;",
            path.0, path.1
        );

        let results = self
            .query(&query)
            .await
            .map_err(|e| format!("Failed to list columns: {:?}", e))?
            .rows
            .into_iter()
            .map(|row| ColumnWithDetails {
                name: row[0].clone(),
                description: if row[2].is_empty() {
                    None
                } else {
                    Some(row[2].clone())
                },
                data_type: Some(row[1].clone()),
                is_nullable: None,
                is_unique: None,
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        let mut client = self
            .pool
            .get_handle()
            .await
            .map_err(|e| format!("Failed to get client handle: {}", e))?;
        client
            .execute(query)
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let mut client = self.pool.get_handle().await.map_err(|e| {
            QueryError::new(
                query.to_string(),
                format!("Failed to get client handle: {}", e),
            )
        })?;

        let mut stream = client.query(query).stream();
        let mut columns = vec![];
        let mut rows = vec![];

        while let Some(row) = stream.next().await {
            let row = row.map_err(|e| {
                QueryError::new(query.to_string(), format!("Failed to get row: {}", e))
            })?;

            let row_length = row.len();
            // TODO Optimise only do this once
            columns = (0..row_length)
                .map(|i| {
                    (
                        row.name(i).unwrap().to_string(),
                        Some(row.sql_type(i).unwrap().to_string().to_string()),
                    )
                })
                .collect();

            let row: Vec<String> = (0..row_length)
                .map(|i| {
                    let sql_type = row.sql_type(i).unwrap();
                    match sql_type {
                        SqlType::String => row.get::<String, usize>(i).unwrap(),
                        SqlType::UInt8 => row.get::<u8, usize>(i).unwrap().to_string(),
                        SqlType::UInt16 => row.get::<u16, usize>(i).unwrap().to_string(),
                        SqlType::UInt32 => row.get::<u32, usize>(i).unwrap().to_string(),
                        SqlType::UInt64 => row.get::<u64, usize>(i).unwrap().to_string(),
                        SqlType::Int8 => row.get::<i8, usize>(i).unwrap().to_string(),
                        SqlType::Int16 => row.get::<i16, usize>(i).unwrap().to_string(),
                        SqlType::Int32 => row.get::<i32, usize>(i).unwrap().to_string(),
                        SqlType::Int64 => row.get::<i64, usize>(i).unwrap().to_string(),
                        SqlType::Float32 => row.get::<f32, usize>(i).unwrap().to_string(),
                        SqlType::Float64 => row.get::<f64, usize>(i).unwrap().to_string(),
                        SqlType::Date => {
                            row.get::<chrono::NaiveDate, usize>(i).unwrap().to_string()
                        }
                        _ => {
                            todo!(
                                "{} {}",
                                query.to_string(),
                                format!("Unsupported type: {:?}", sql_type)
                            );
                        }
                    }
                })
                .collect::<Vec<String>>();
            rows.push(row);
        }
        Ok(QueryResult { columns, rows })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorClickhouse::new(
            self.database.to_string(),
            None,
        ))
    }

    async fn table_exists(&self, _path: &str) -> Result<Option<bool>, String> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::bytes::Bytes;
    use quary_core::project::parse_project;
    use quary_core::project_tests::return_tests_sql;
    use quary_proto::{File, FileSystem};
    use testcontainers::runners::AsyncRunner;
    use testcontainers::ContainerRequest;
    use testcontainers_modules::clickhouse::ClickHouse as TestcontainersClickHouse;

    #[tokio::test]
    async fn test_clickhouse_list_tables_and_views() {
        let container = ContainerRequest::from(TestcontainersClickHouse::default())
            .start()
            .await
            .unwrap();
        let port = container
            .get_host_port_ipv4(9000)
            .await
            .unwrap()
            .to_string();
        let host = container.get_host().await.unwrap().to_string();
        let database = Clickhouse::new(host.as_str(), Some(&port), None, None, None)
            .await
            .unwrap();

        database
            .exec("CREATE TABLE wrong_table (id INTEGER, name VARCHAR(255)) ENGINE = MergeTree ORDER BY id;")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE test_table (id INTEGER, name VARCHAR(255)) ENGINE = MergeTree ORDER BY id;")
            .await
            .unwrap();
        database
            .exec("INSERT INTO test_table VALUES (1, 'test')")
            .await
            .unwrap();
        database
            .exec("INSERT INTO test_table VALUES (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE VIEW test_view AS SELECT * FROM test_table")
            .await
            .unwrap();
        database
            .exec("CREATE VIEW wrong_view AS SELECT * FROM test_table")
            .await
            .unwrap();

        let tables = database.list_local_tables().await.unwrap();
        assert_eq!(
            tables,
            vec![
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "default.test_table".to_string(),
                },
                TableAddress {
                    name: "wrong_table".to_string(),
                    full_path: "default.wrong_table".to_string(),
                },
            ]
        );

        let views = database.list_local_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "default.test_view".to_string(),
                },
                TableAddress {
                    name: "wrong_view".to_string(),
                    full_path: "default.wrong_view".to_string(),
                },
            ]
        );

        let columns = database.list_columns("test_table").await.unwrap();
        assert_eq!(
            vec![
                ColumnWithDetails {
                    name: "id".to_string(),
                    description: None,
                    data_type: Some("Int32".to_string()),
                    is_nullable: None,
                    is_unique: None,
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    description: None,
                    data_type: Some("String".to_string()),
                    is_nullable: None,
                    is_unique: None,
                }
            ],
            columns,
        );

        let result = database
            .query("SELECT * FROM test_table ORDER BY id")
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
    async fn test_clickhouse_list_columns_in_table() {
        let container = ContainerRequest::from(TestcontainersClickHouse::default())
            .start()
            .await
            .unwrap();
        let port = container
            .get_host_port_ipv4(9000)
            .await
            .unwrap()
            .to_string();
        let host = container.get_host().await.unwrap().to_string();
        let database = Clickhouse::new(host.as_str(), Some(&port), None, None, None)
            .await
            .unwrap();

        database
            .exec("CREATE DATABASE IF NOT EXISTS transform")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS transform.test_table (id INTEGER, name_transform VARCHAR(255)) ENGINE = MergeTree ORDER BY id")
            .await
            .unwrap();
        database
            .exec("CREATE DATABASE IF NOT EXISTS other_schema")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS other_schema.test_table (id INTEGER NOT NULL, name VARCHAR(255)) ENGINE = MergeTree ORDER BY id")
            .await
            .unwrap();
        database
            .exec("ALTER TABLE other_schema.test_table COMMENT COLUMN id 'test comment'")
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
                    data_type: Some("Int32".to_string()),
                    is_nullable: None,
                    is_unique: None,
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    description: None,
                    data_type: Some("String".to_string()),
                    is_nullable: None,
                    is_unique: None,
                }
            ]
        );
        let columns = database.list_columns("transform.test_table").await.unwrap();
        assert_eq!(
            columns,
            vec![
                ColumnWithDetails {
                    name: "id".to_string(),
                    description: None,
                    data_type: Some("Int32".to_string()),
                    is_nullable: None,
                    is_unique: None,
                },
                ColumnWithDetails {
                    name: "name_transform".to_string(),
                    description: None,
                    data_type: Some("String".to_string()),
                    is_nullable: None,
                    is_unique: None,
                }
            ]
        );
    }

    #[tokio::test]
    async fn test_clickhouse_foreign_relationship_test_with_schema() {
        let container = ContainerRequest::from(TestcontainersClickHouse::default())
            .start()
            .await
            .unwrap();
        let port = container
            .get_host_port_ipv4(9000)
            .await
            .unwrap()
            .to_string();
        let host = container.get_host().await.unwrap().to_string();
        let database = Clickhouse::new(host.as_str(), Some(&port), None, None, None)
            .await
            .unwrap();

        database
            .exec("CREATE DATABASE IF NOT EXISTS other_schema")
            .await
            .unwrap();
        database
            .exec("CREATE DATABASE IF NOT EXISTS transform")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS other_schema.test_table (id INTEGER, name VARCHAR(255)) ENGINE = MergeTree ORDER BY id")
            .await
            .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE IF NOT EXISTS transform.test_table (id INTEGER, name VARCHAR(255)) ENGINE = MergeTree ORDER BY id")
            .await
            .unwrap();
        database
            .exec("INSERT INTO transform.test_table VALUES (3, 'test_3'), (4, 'rubbish_rubiish')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                ("quary.yaml", "clickhouse: {database: transform}"),
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
    //
    // // TEST FAILS in Redshift: I think this is because we execute the command twice too quickly?
    // // Possibly becuase of a table lock that occurs in Redshift when a materialized view is created?
    // #[tokio::test]
    // async fn test_clickhouse_foreign_relationship_test_with_materialized_view_table() {
    //     let database = Clickhouse::new("", None, "", "", "", "", None, None, None, None, None)
    //         .await
    //         .unwrap();
    //
    //     database
    //         .exec("CREATE SCHEMA IF NOT EXISTS other_schema")
    //         .await
    //         .unwrap();
    //     database
    //         .exec("CREATE SCHEMA IF NOT EXISTS transform")
    //         .await
    //         .unwrap();
    //     database
    //         .exec("CREATE TABLE IF NOT EXISTS other_schema.test_table (id INTEGER, name VARCHAR(255))")
    //         .await
    //         .unwrap();
    //     database
    //         .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
    //         .await
    //         .unwrap();
    //     database
    //         .exec("CREATE TABLE IF NOT EXISTS transform.test_table (id INTEGER, name VARCHAR(255))")
    //         .await
    //         .unwrap();
    //     database
    //         .exec("INSERT INTO transform.test_table VALUES (3, 'test_3'), (4, 'rubbish_rubiish')")
    //         .await
    //         .unwrap();
    //
    //     let file_system = FileSystem {
    //         files: vec![
    //             ("quary.yaml", "postgres: {schema: transform}"),
    //             ("models/test_model.sql", "SELECT id FROM q.test_source"),
    //             (
    //                 "models/test_model_same_schema.sql",
    //                 "SELECT id FROM q.test_source_same_schema",
    //             ),
    //             ("models/test_model_out.sql", "SELECT id FROM q.test_model"),
    //             (
    //                 "models/schema.yaml",
    //                 "
    // sources:
    //     - name: test_source
    //       path: other_schema.test_table
    //     - name: test_source_same_schema
    //       path: transform.test_table
    // models:
    //   - name: test_model_out
    //     materialization: table
    //     columns:
    //       - name: id
    //         tests:
    //           - type: relationship
    //             info:
    //               column: id
    //               model: test_model
    //           - type: relationship
    //             info:
    //               column: id
    //               model: test_source
    //   - name: test_model_same_schema
    //     materialization: materialized_view
    //     columns:
    //       - name: id
    //         tests:
    //           - type: relationship
    //             info:
    //               column: id
    //               model: test_source_same_schema
    //                     ",
    //             ),
    //         ]
    //         .into_iter()
    //         .map(|(k, v)| {
    //             (
    //                 k.to_string(),
    //                 File {
    //                     name: k.to_string(),
    //                     contents: Bytes::from(v),
    //                 },
    //             )
    //         })
    //         .collect(),
    //     };
    //
    //     let project = parse_project(&file_system, &database.query_generator(), "")
    //         .await
    //         .unwrap();
    //
    //     let sqls = project_and_fs_to_sql_for_views(
    //         &project,
    //         &file_system,
    //         &database.query_generator(),
    //         false,
    //         false,
    //     )
    //     .await
    //     .unwrap();
    //     for sql in &sqls {
    //         for sql in &sql.1 {
    //             database.exec(&sql).await.unwrap();
    //         }
    //     }
    //     // Run twice
    //     for sql in &sqls {
    //         for sql in &sql.1 {
    //             database.exec(&sql).await.unwrap();
    //         }
    //     }
    //
    //     let tests = return_tests_sql(
    //         &database.query_generator(),
    //         &project,
    //         &file_system,
    //         false,
    //         None,
    //         None,
    //     )
    //     .await
    //     .unwrap();
    //     let tests = tests.iter().collect::<Vec<_>>();
    //
    //     assert!(!tests.is_empty());
    //
    //     for (name, test) in tests.iter() {
    //         let results = database.query(test).await.unwrap();
    //
    //         assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
    //     }
    // }

    #[tokio::test]
    async fn test_list_tables_outside_the_schema() {
        let container = ContainerRequest::from(TestcontainersClickHouse::default())
            .start()
            .await
            .unwrap();
        let port = container
            .get_host_port_ipv4(9000)
            .await
            .unwrap()
            .to_string();
        let host = container.get_host().await.unwrap().to_string();
        let database = Clickhouse::new(host.as_str(), Some(&port), None, None, None)
            .await
            .unwrap();

        database.exec("CREATE DATABASE other_schema").await.unwrap();
        database.exec("CREATE DATABASE transform").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255)) ENGINE = MergeTree ORDER BY id")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE transform.test_table (id INTEGER, name VARCHAR(255)) ENGINE = MergeTree ORDER BY id")
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
    //
    // // TEST FAILS IN REDSHIFT: In Redshift column names are case-insensitive by default.
    // #[tokio::test]
    // async fn test_clickhouse_list_columns_with_case_sensitive_columns() {
    //     let database = Clickhouse::new("", None, "", "", "", "", None, None, None, None, None)
    //         .await
    //         .unwrap();
    //
    //     database.exec("CREATE SCHEMA transform").await.unwrap();
    //     database
    //         .exec("CREATE TABLE transform.test_table (\"ID\" INTEGER, \"Name\" VARCHAR(255), test VARCHAR(255), TESTTWO VARCHAR(255))")
    //         .await
    //         .unwrap();
    //
    //     let columns = database.list_columns("transform.test_table").await.unwrap();
    //     assert_eq!(
    //         columns,
    //         vec![
    //             ColumnWithDetails {
    //                 name: "\"ID\"".to_string(),
    //                 description: None,
    //                 data_type: None,
    //                 is_nullable: Some(true),
    //                 is_unique: Some(false),
    //             },
    //             ColumnWithDetails {
    //                 name: "\"Name\"".to_string(),
    //                 description: None,
    //                 data_type: None,
    //                 is_nullable: Some(true),
    //                 is_unique: Some(false),
    //             },
    //             ColumnWithDetails {
    //                 name: "test".to_string(),
    //                 description: None,
    //                 data_type: None,
    //                 is_nullable: Some(true),
    //                 is_unique: Some(false),
    //             },
    //             ColumnWithDetails {
    //                 name: "testtwo".to_string(),
    //                 description: None,
    //                 data_type: None,
    //                 is_nullable: Some(true),
    //                 is_unique: Some(false),
    //             }
    //         ]
    //     );
    // }
    //
    // #[tokio::test]
    // async fn test_clickhouse_snapshots_with_schema() {
    //     let schema = "analytics";
    //
    //     let container = ContainerRequest::from(TestcontainersClickHouse::default())
    //         .start()
    //         .await
    //         .unwrap();
    //     let port = container.get_host_port_ipv4(9000).await.unwrap();
    //     let host = container.get_host().await.unwrap();
    //     let host = format!("tcp://default@{}:{}", host, port);
    //     let database = Clickhouse::new(
    //         host.as_str(),
    //         Some(port.to_string()),
    //         "default",
    //         "",
    //         schema,
    //         "default",
    //     )
    //         .await
    //         .unwrap();
    //     let database = Box::new(database);
    //
    //     database.exec("CREATE DATABASE analytics").await.unwrap();
    //     database.exec("CREATE DATABASE jaffle_shop").await.unwrap();
    //
    //     let datetime_str = "2023-01-01 01:00:00";
    //
    //     // Parse the string into a NaiveDateTime
    //     let naive_datetime =
    //         NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap();
    //
    //     // Convert NaiveDateTime to DateTime<Utc>
    //     let datetime_utc = DateTime::<Utc>::from_utc(naive_datetime, Utc);
    //
    //     // Convert DateTime<Utc> to SystemTime
    //     let system_time = SystemTime::from(datetime_utc);
    //
    //     let db_generator =
    //         DatabaseQueryGeneratorClickhouse::new(schema.to_string(), Some(system_time));
    //
    //     // Create orders table
    //     database
    //         .exec("CREATE TABLE jaffle_shop.raw_orders (order_id INTEGER, status VARCHAR(255), updated_at TIMESTAMP) ENGINE = MergeTree ORDER BY order_id")
    //         .await
    //         .unwrap();
    //
    //     // Insert some initial data
    //     database
    //         .exec("INSERT INTO jaffle_shop.raw_orders VALUES (1, 'in_progress', '2023-01-01 00:00:00'), (2, 'completed', '2023-01-01 00:00:00')")
    //         .await
    //         .unwrap();
    //
    //     let file_system = FileSystem {
    //         files: vec![
    //             ("quary.yaml", "clickhouse: {database: analytics}"),
    //             (
    //                 "models/orders_snapshot.snapshot.sql",
    //                 "SELECT * FROM q.raw_orders",
    //             ),
    //             (
    //                 "models/schema.yaml",
    //                 "
    // sources:
    //   - name: raw_orders
    //     path: jaffle_shop.raw_orders
    // snapshots:
    //   - name: orders_snapshot
    //     unique_key: order_id
    //     strategy:
    //       timestamp:
    //         updated_at: updated_at
    // ",
    //             ),
    //         ]
    //         .iter()
    //         .map(|(k, v)| {
    //             (
    //                 k.to_string(),
    //                 File {
    //                     name: k.to_string(),
    //                     contents: Bytes::from(v.to_string()),
    //                 },
    //             )
    //         })
    //         .collect(),
    //     };
    //
    //     let project = parse_project(&file_system, &db_generator, "")
    //         .await
    //         .unwrap();
    //
    //     let snapshots_sql = project_and_fs_to_sql_for_snapshots(
    //         &project,
    //         &file_system,
    //         &db_generator,
    //         database.as_ref(),
    //     )
    //     .await
    //     .unwrap();
    //     for (_, sql) in snapshots_sql {
    //         for statement in sql {
    //             println!("{}", statement.as_str());
    //             database.exec(statement.as_str()).await.unwrap()
    //         }
    //     }
    //
    //     // assert the data has been created correctly in the snapshot table
    //     let data = database
    //         .query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM analytics.orders_snapshot ORDER BY order_id, quary_valid_from")
    //         .await
    //         .unwrap();
    //
    //     assert_eq!(
    //         data.columns
    //             .iter()
    //             .map(|(column, _)| column)
    //             .collect::<Vec<_>>(),
    //         vec![
    //             "order_id",
    //             "status",
    //             "updated_at",
    //             "quary_valid_from",
    //             "quary_valid_to",
    //             "quary_scd_id"
    //         ]
    //     );
    //     assert_eq!(
    //         data.rows,
    //         vec![
    //             vec![
    //                 "1",
    //                 "in_progress",
    //                 "2023-01-01T00:00:00",
    //                 "2023-01-01T01:00:00+00:00",
    //                 "NULL",
    //                 "77f50225cf5a52d15fecaa449be2dcc4"
    //             ],
    //             vec![
    //                 "2",
    //                 "completed",
    //                 "2023-01-01T00:00:00",
    //                 "2023-01-01T01:00:00+00:00",
    //                 "NULL",
    //                 "3bb5cc6bb5b432df7712d067f57a3780"
    //             ],
    //         ]
    //     );
    //
    //     database
    //         .exec("UPDATE jaffle_shop.raw_orders SET status = 'completed', updated_at = CAST('2023-01-01 02:00:00' AS TIMESTAMP) WHERE order_id = 1")
    //         .await
    //         .unwrap();
    //
    //     let datetime_str_updated = "2023-01-01 03:00:00";
    //
    //     // Parse the string into a NaiveDateTime
    //     let naive_datetime_updated =
    //         NaiveDateTime::parse_from_str(datetime_str_updated, "%Y-%m-%d %H:%M:%S").unwrap();
    //
    //     // Convert NaiveDateTime to DateTime<Utc>
    //     let datetime_utc_updated = DateTime::<Utc>::from_utc(naive_datetime_updated, Utc);
    //
    //     // Convert DateTime<Utc> to SystemTime
    //     let system_time_updated = SystemTime::from(datetime_utc_updated);
    //
    //     let db_generator_updated =
    //         DatabaseQueryGeneratorClickhouse::new(schema.to_string(), Some(system_time_updated));
    //
    //     let snapshots_sql = project_and_fs_to_sql_for_snapshots(
    //         &project,
    //         &file_system,
    //         &db_generator_updated,
    //         database.as_ref(),
    //     )
    //     .await
    //     .unwrap();
    //
    //     for (_, sql) in &snapshots_sql {
    //         for statement in sql {
    //             database.exec(statement.as_str()).await.unwrap()
    //         }
    //     }
    //
    //     // assert the data has been created correctly in the snapshot table
    //     let data = database
    //         .query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM analytics.orders_snapshot ORDER BY order_id, quary_valid_from")
    //         .await
    //         .unwrap();
    //
    //     assert_eq!(
    //         data.columns
    //             .iter()
    //             .map(|(column, _)| column)
    //             .collect::<Vec<_>>(),
    //         vec![
    //             "order_id",
    //             "status",
    //             "updated_at",
    //             "quary_valid_from",
    //             "quary_valid_to",
    //             "quary_scd_id"
    //         ]
    //     );
    //     assert_eq!(
    //         data.rows,
    //         vec![
    //             vec![
    //                 "1",
    //                 "in_progress",
    //                 "2023-01-01T00:00:00",
    //                 "2023-01-01T01:00:00+00:00",
    //                 "2023-01-01T03:00:00+00:00",
    //                 "77f50225cf5a52d15fecaa449be2dcc4"
    //             ],
    //             vec![
    //                 "1",
    //                 "completed",
    //                 "2023-01-01T02:00:00",
    //                 "2023-01-01T03:00:00+00:00",
    //                 "NULL",
    //                 "f5c7798e30814925cd1a61e9e5ef6683"
    //             ],
    //             vec![
    //                 "2",
    //                 "completed",
    //                 "2023-01-01T00:00:00",
    //                 "2023-01-01T01:00:00+00:00",
    //                 "NULL",
    //                 "3bb5cc6bb5b432df7712d067f57a3780"
    //             ],
    //         ]
    //     );
    // }
}
