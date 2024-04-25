use async_trait::async_trait;
use quary_core::database_postgres::DatabaseQueryGeneratorPostgres;
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::TableAddress;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Column, Pool, Row};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Postgres {
    pool: Pool<sqlx::Postgres>,
    schema: String,
}

impl Postgres {
    pub async fn new(
        host: &str,
        port: &str,
        user: &str,
        password: &str,
        database: &str,
        schema: &str,
        ssl_mode: Option<String>,
        ssl_cert: Option<String>,
        ssl_key: Option<String>,
        ssl_root_cert: Option<String>,
    ) -> Result<Self, sqlx::Error> {
        let params = HashMap::from([
            ("sslmode", ssl_mode),
            ("sslcert", ssl_cert),
            ("sslkey", ssl_key),
            ("sslrootcert", ssl_root_cert),
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

        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}{}",
            user,
            password,
            host,
            port,
            database,
            params.unwrap_or("".to_string())
        );

        let pool = PgPoolOptions::new().connect(&connection_string).await?;
        Ok(Self {
            pool,
            schema: schema.to_string(),
        })
    }
}

#[async_trait]
impl DatabaseConnection for Postgres {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let rows = sqlx::query(
            "
SELECT table_name, table_schema
FROM information_schema.tables
WHERE table_type = 'BASE TABLE' AND table_schema != 'information_schema' AND table_schema != 'pg_catalog'
ORDER BY table_schema ASC, table_name ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        let mut tables = Vec::new();
        for row in rows {
            let table_name: String = row.get(0);
            let table_schema: String = row.get(1);
            tables.push(TableAddress {
                name: table_name.clone(),
                full_path: format!("{}.{}", table_schema, table_name),
            });
        }
        Ok(tables)
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let rows = sqlx::query(
            "
SELECT table_name, table_schema
FROM information_schema.tables
WHERE table_type = 'VIEW' AND table_schema != 'information_schema' AND table_schema != 'pg_catalog'
ORDER BY table_schema ASC, table_name ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        let mut tables = Vec::new();
        for row in rows {
            let table_name: String = row.get(0);
            let table_schema: String = row.get(1);
            tables.push(TableAddress {
                name: table_name.clone(),
                full_path: format!("{}.{}", table_schema, table_name),
            });
        }
        Ok(tables)
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let rows = sqlx::query(
            format!("SELECT table_name, table_schema FROM information_schema.tables WHERE table_schema = '{}' AND table_type = 'BASE TABLE' ORDER BY table_schema DESC, table_name DESC",
                    self.schema).as_str(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut tables = Vec::new();

        for row in rows {
            let table_name: String = row.get(0);
            let table_schema: String = row.get(1);
            tables.push(TableAddress {
                name: table_name.clone(),
                full_path: format!("{}.{}", table_schema, table_name),
            });
        }
        Ok(tables)
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let rows = sqlx::query(
            format!("SELECT table_name, table_schema FROM information_schema.tables WHERE table_schema = '{}' AND table_type = 'VIEW' ORDER BY table_schema DESC, table_name DESC", self.schema).as_str(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut tables = Vec::new();

        for row in rows {
            let table_name: String = row.get(0);
            let table_schema: String = row.get(1);
            tables.push(TableAddress {
                name: table_name.clone(),
                full_path: format!("{}.{}", table_schema, table_name),
            });
        }
        Ok(tables)
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
            c.column_name,
            pgd.description AS column_comment,
            c.is_nullable,
            CASE
                WHEN tc.constraint_type = 'UNIQUE' THEN 'YES'
                ELSE 'NO'
            END AS is_unique
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

                ColumnWithDetails {
                    name: row.get(0),
                    description,
                    is_nullable: Some(is_nullable == "YES"),
                    is_unique: Some(is_unique == "YES"),
                    ..Default::default()
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
        for row in rows {
            let mut row_vec = Vec::new();
            for i in 0..row.len() {
                // Attempt to get the value as an i32, if it fails, try as a String
                let value: String = if let Ok(val) = row.try_get::<i32, _>(i) {
                    val.to_string()
                } else {
                    row.try_get::<String, _>(i)
                        .unwrap_or_else(|_| String::from(""))
                };
                row_vec.push(value);
            }
            rows_vec.push(row_vec);
        }

        Ok(QueryResult {
            columns,
            rows: rows_vec,
        })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorPostgres::new(self.schema.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::bytes::Bytes;
    use quary_core::project::{parse_project, project_and_fs_to_sql_for_views};
    use quary_core::project_tests::return_tests_sql;
    use quary_proto::{File, FileSystem};
    use testcontainers::{clients, RunnableImage};
    use testcontainers_modules::postgres::Postgres as TestcontainersPostgres;

    #[tokio::test]
    async fn test_postgres_list_tables_and_views() {
        // Start a PostgreSQL container
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let quary_postgres = Postgres::new(
            "localhost",
            &pg_port.to_string(),
            "postgres",
            "postgres",
            "postgres",
            "public",
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
                    name: "wrong_table".to_string(),
                    full_path: "public.wrong_table".to_string(),
                },
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "public.test_table".to_string(),
                },
            ]
        );

        let views = quary_postgres.list_local_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "wrong_view".to_string(),
                    full_path: "public.wrong_view".to_string(),
                },
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "public.test_view".to_string(),
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
        assert_eq!(result.columns, vec!["id", "name"]);
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
            &pg_port.to_string(),
            "postgres",
            "postgres",
            "postgres",
            "transform",
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
    async fn test_postgres_foreign_relationship_test_with_schema() {
        // Start a PostgreSQL container
        let docker = clients::Cli::default();
        let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
        let pg_container = docker.run(postgres_image);
        let pg_port = pg_container.get_host_port_ipv4(5432);

        let database = Postgres::new(
            "localhost",
            &pg_port.to_string(),
            "postgres",
            "postgres",
            "postgres",
            "transform",
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
            let results = database
                .query(test)
                .await
                .unwrap();

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
            &pg_port.to_string(),
            "postgres",
            "postgres",
            "postgres",
            "transform",
            None,
            None,
            None,
            None,
        )
        .await
        .expect("Failed to instantiate Quary Postgres");

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
            let results = database
                .query(test)
                .await
                .expect(&format!("Error running query {}", test));

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
            &pg_port.to_string(),
            "postgres",
            "postgres",
            "postgres",
            "transform",
            None,
            None,
            None,
            None,
        )
        .await
        .expect("Failed to instantiate Quary Postgres");

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
}
