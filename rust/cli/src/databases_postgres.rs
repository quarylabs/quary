use async_trait::async_trait;
use quary_core::database_postgres::DatabaseQueryGeneratorPostgres;
use quary_core::databases::{
    DatabaseConnection, DatabaseQueryGenerator, QueryResult, TableAddress,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Column, Pool, Row};
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
        params: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}{}",
            user,
            password,
            host,
            port,
            database,
            params.unwrap_or("")
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
            format!("SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type = 'BASE TABLE' ORDER BY table_name DESC", self.schema).as_str(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut tables = Vec::new();

        for row in rows {
            let table_name: String = row.get(0);
            tables.push(TableAddress {
                name: table_name.clone(),
                full_path: table_name,
            });
        }

        Ok(tables)
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let rows = sqlx::query(
            format!("SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type = 'VIEW' ORDER BY table_name DESC", self.schema).as_str(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut tables = Vec::new();

        for row in rows {
            let table_name: String = row.get(0);
            tables.push(TableAddress {
                name: table_name.clone(),
                full_path: table_name,
            });
        }

        Ok(tables)
    }

    async fn list_columns(&self, table: &str) -> Result<Vec<String>, String> {
        let rows = sqlx::query(&format!(
            "SELECT column_name FROM information_schema.columns WHERE table_name = '{}'",
            table
        ))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut columns = Vec::new();

        for row in rows {
            let column_name: String = row.get(0);
            columns.push(column_name);
        }

        Ok(columns)
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        let query = sqlx::query(query);
        query.execute(&self.pool).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, String> {
        let query_builder = sqlx::query(query);

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

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
        )
        .await
        .expect("Failed to instantiate Quary Postgres");

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

        let tables = quary_postgres.list_tables().await.unwrap();
        assert_eq!(
            tables,
            vec![
                TableAddress {
                    name: "wrong_table".to_string(),
                    full_path: "wrong_table".to_string(),
                },
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "test_table".to_string(),
                },
            ]
        );

        let views = quary_postgres.list_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "wrong_view".to_string(),
                    full_path: "wrong_view".to_string(),
                },
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "test_view".to_string(),
                },
            ]
        );

        let columns = quary_postgres.list_columns("test_table").await.unwrap();
        assert_eq!(columns, vec!["id", "name"]);

        let result = quary_postgres
            .query("SELECT * FROM test_table")
            .await
            .unwrap();
        assert_eq!(result.columns, vec!["id", "name"]);
        assert_eq!(result.rows, vec![vec!["1", "test"], vec!["2", "rubbish"]]);
    }

    //    TODO understand why relationship test is failing:  SELECT * FROM (SELECT id FROM test_model)) WHERE id IS NOT NULL AND id NOT IN (SELECT id FROM (WITH test_source AS
    // (SELECT * FROM other_schema.test_table) SELECT * FROM (SELECT id FROM test_source))): "error returned from database: subquery in FROM must have an alias"
    //     #[tokio::test]
    //     async fn test_postgres_foreign_relationship_test_with_schema() {
    //         // Start a PostgreSQL container
    //         let docker = clients::Cli::default();
    //         let postgres_image = RunnableImage::from(TestcontainersPostgres::default());
    //         let pg_container = docker.run(postgres_image);
    //         let pg_port = pg_container.get_host_port_ipv4(5432);

    //         let database = Postgres::new(
    //             "localhost",
    //             &pg_port.to_string(),
    //             "postgres",
    //             "postgres",
    //             "postgres",
    //             "transform",
    //             None,
    //         )
    //         .await
    //         .expect("Failed to instantiate Quary Postgres");

    //         database.exec("CREATE SCHEMA other_schema").await.unwrap();
    //         database.exec("CREATE SCHEMA transform").await.unwrap();
    //         database
    //             .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255))")
    //             .await
    //             .unwrap();
    //         database
    //             .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
    //             .await
    //             .unwrap();
    //         database
    //             .exec("CREATE TABLE transform.test_table (id INTEGER, name VARCHAR(255))")
    //             .await
    //             .unwrap();
    //         database
    //             .exec("INSERT INTO transform.test_table VALUES (3, 'test_3'), (4, 'rubbish_rubiish')")
    //             .await
    //             .unwrap();

    //         let file_system = FileSystem {
    //             files: vec![
    //                 ("models/test_model.sql", "SELECT id FROM q.test_source"),
    //                 (
    //                     "models/test_model_same_schema.sql",
    //                     "SELECT id FROM q.test_source_same_schema",
    //                 ),
    //                 ("models/test_model_out.sql", "SELECT id FROM q.test_model"),
    //                 (
    //                     "models/schema.yaml",
    //                     "
    // sources:
    //     - name: test_source
    //       path: other_schema.test_table
    //     - name: test_source_same_schema
    //       path: test_schema.test_table
    // models:
    //   - name: test_model_out
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
    //     columns:
    //       - name: id
    //         tests:
    //           - type: relationship
    //             info:
    //               column: id
    //               model: test_source_same_schema
    //                 ",
    //                 ),
    //             ]
    //             .into_iter()
    //             .map(|(k, v)| {
    //                 (
    //                     k.to_string(),
    //                     File {
    //                         name: k.to_string(),
    //                         contents: Bytes::from(v),
    //                     },
    //                 )
    //             })
    //             .collect(),
    //         };

    //         let project = parse_project(&file_system, &database.query_generator(), "").unwrap();

    //         let tests = return_tests_sql(
    //             &database.query_generator(),
    //             &project,
    //             &file_system,
    //             true,
    //             None,
    //         )
    //         .unwrap();
    //         let tests = tests.iter().collect::<Vec<_>>();

    //         assert!(!tests.is_empty());

    //         for (name, test) in tests.iter() {
    //             let results = database.query(test).await.expect(&format!("Error running query {}", test));

    //             assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
    //         }
    //     }
}
