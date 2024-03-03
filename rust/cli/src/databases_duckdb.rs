use duckdb::arrow::array::array;
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::{params, Connection};
use quary_core::database_duckdb::DatabaseQueryGeneratorDuckDB;
use quary_core::databases::{
    DatabaseConnection, DatabaseQueryGenerator, QueryResult, TableAddress,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct DuckDB {
    schema: Option<String>,
    connection: Arc<Mutex<Connection>>,
}

impl DuckDB {
    pub fn new_in_memory(schema: Option<String>) -> Result<Self, String> {
        let connection = Connection::open_in_memory()
            .map_err(|e| format!("Failed to open DuckDB connection: {}", e))?;
        if let Some(scheme) = &schema {
            connection
                .execute(
                    &format!("CREATE SCHEMA IF NOT EXISTS {}", scheme),
                    params![],
                )
                .map_err(|e| format!("Failed to create schema {}: {}", scheme, e))?;
        };
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            schema,
        })
    }

    pub fn new_with_file(schema: Option<String>, path: &str) -> Result<Self, String> {
        let connection = Connection::open(path)
            .map_err(|e| format!("Failed to open DuckDB connection: {}", e))?;
        if let Some(scheme) = &schema {
            connection
                .execute(
                    &format!("CREATE SCHEMA IF NOT EXISTS {}", scheme),
                    params![],
                )
                .map_err(|e| format!("Failed to create schema {}: {}", scheme, e))?;
        };
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            schema,
        })
    }
}

#[async_trait::async_trait]
impl DatabaseConnection for DuckDB {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let results = if let Some(schema) = &self.schema {
            self.query(&format!(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type='BASE TABLE' ORDER BY table_name",
                schema
            ))
                .await?
        } else {
            self.query("SELECT table_name FROM information_schema.tables WHERE table_type='BASE TABLE' ORDER BY table_name")
                .await?
        };
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[0].clone(),
                full_path: if let Some(schema) = &self.schema {
                    format!("{}.{}", schema, row[0].clone())
                } else {
                    row[0].clone()
                },
            })
            .collect())
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let results = if let Some(schema) = &self.schema {
            self.query(&format!(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type='VIEW' ORDER BY table_name",
                schema
            ))
                .await?
        } else {
            self.query("SELECT table_name FROM information_schema.tables WHERE table_type='VIEW' ORDER BY table_name")
                .await?
        };
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[0].clone(),
                full_path: if let Some(schema) = &self.schema {
                    format!("{}.{}", schema, row[0].clone())
                } else {
                    row[0].clone()
                },
            })
            .collect())
    }

    async fn list_columns(&self, table: &str) -> Result<Vec<String>, String> {
        let results = self.query(&format!("PRAGMA table_info({})", table)).await?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| row[1].clone())
            .collect::<Vec<String>>())
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|e| format!("Failed to get connection lock: {}", e))?;
        conn.execute(query, params![])
            .map_err(|e| format!("Failed to execute query {}: {}", query, e))?;
        return Ok(());
    }

    async fn query(&self, query: &str) -> Result<QueryResult, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|e| format!("Failed to get connection lock: {}", e))?;

        let mut stmt = conn
            .prepare(query)
            .map_err(|e| format!("Failed to prepare query {}: {}", query, e))?;

        let rbs: Vec<RecordBatch> = stmt
            .query_arrow([])
            .map_err(|e| format!("Failed to execute query {}: {}", query, e))?
            .collect();

        if rbs.len() != 1 {
            return Ok(QueryResult {
                columns: vec![],
                rows: vec![],
            });
        }

        let columns = rbs[0]
            .schema()
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect::<Vec<String>>();

        let rows = convert_array_to_vec_string(rbs[0].columns())?;

        Ok(QueryResult { columns, rows })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorDuckDB::new(self.schema.clone()))
    }
}

fn convert_array_to_vec_string(
    array: &[Arc<dyn array::Array>],
) -> Result<Vec<Vec<String>>, String> {
    let num_rows = array[0].len();
    let num_columns = array.len();
    let mut rows = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {
        let row = vec!["".to_string(); num_columns];
        rows.push(row);
    }

    for i in 0..num_rows {
        for j in 0..array.len() {
            let array = &array[j];
            if let Some(string_array) = array.as_any().downcast_ref::<array::StringArray>() {
                rows[i][j] = string_array.value(i).to_string();
            } else if let Some(int32_array) = array.as_any().downcast_ref::<array::Int32Array>() {
                rows[i][j] = int32_array.value(i).to_string();
            } else if let Some(int64_array) = array.as_any().downcast_ref::<array::Int64Array>() {
                rows[i][j] = int64_array.value(i).to_string();
            } else if let Some(float32_array) = array.as_any().downcast_ref::<array::Float32Array>()
            {
                rows[i][j] = float32_array.value(i).to_string();
            } else if let Some(float64_array) = array.as_any().downcast_ref::<array::Float64Array>()
            {
                rows[i][j] = float64_array.value(i).to_string();
            } else if let Some(boolean_array) = array.as_any().downcast_ref::<array::BooleanArray>()
            {
                rows[i][j] = boolean_array.value(i).to_string();
            } else if let Some(date_array) = array.as_any().downcast_ref::<array::Date64Array>() {
                rows[i][j] = date_array.value(i).to_string();
            } else if let Some(date_array) = array.as_any().downcast_ref::<array::Date32Array>() {
                rows[i][j] = date_array.value(i).to_string();
            } else {
                return Err("Unsupported array type".to_string());
            }
        }
    }

    // Example for a specific array type, e.g., StringArray
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::bytes::Bytes;
    use quary_core::project::{parse_project, project_and_fs_to_sql_for_views};
    use quary_core::project_tests::return_tests_sql;
    use quary_proto::{File, FileSystem};

    #[tokio::test]
    async fn test_create_table_without_schema() {
        let db = DuckDB::new_in_memory(None).unwrap();
        db.exec("CREATE TABLE wrong_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        db.exec("CREATE TABLE test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        db.exec("INSERT INTO test_table VALUES (1, 'test')")
            .await
            .unwrap();
        db.exec("INSERT INTO test_table VALUES (2, 'rubbish')")
            .await
            .unwrap();
        db.exec("CREATE VIEW test_view AS SELECT * FROM test_table")
            .await
            .unwrap();
        db.exec("CREATE VIEW wrong_view AS SELECT * FROM test_table")
            .await
            .unwrap();

        let tables = db.list_tables().await.unwrap();
        assert_eq!(
            tables,
            vec![
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "test_table".to_string(),
                },
                TableAddress {
                    name: "wrong_table".to_string(),
                    full_path: "wrong_table".to_string(),
                },
            ]
        );

        let views = db.list_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "test_view".to_string(),
                },
                TableAddress {
                    name: "wrong_view".to_string(),
                    full_path: "wrong_view".to_string(),
                },
            ]
        );

        let columns = db.list_columns("test_table").await.unwrap();
        assert_eq!(columns, vec!["id", "name"]);

        let result = db.query("SELECT * FROM test_table").await.unwrap();
        assert_eq!(result.columns, vec!["id", "name"]);
        assert_eq!(result.rows, vec![vec!["1", "test"], vec!["2", "rubbish"]]);
    }

    #[tokio::test]
    async fn test_create_table_with_schema() {
        let db = DuckDB::new_in_memory(Some("test_schema".to_string())).unwrap();

        db.exec("CREATE TABLE test_schema.wrong_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        db.exec("CREATE TABLE test_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        db.exec("INSERT INTO test_schema.test_table VALUES (1, 'test')")
            .await
            .unwrap();
        db.exec("INSERT INTO test_schema.test_table VALUES (2, 'rubbish')")
            .await
            .unwrap();
        db.exec("CREATE VIEW test_schema.test_view AS SELECT * FROM test_schema.test_table")
            .await
            .unwrap();
        db.exec("CREATE VIEW test_schema.wrong_view AS SELECT * FROM test_schema.test_table")
            .await
            .unwrap();

        let tables = db.list_tables().await.unwrap();
        assert_eq!(
            tables,
            vec![
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "test_schema.test_table".to_string(),
                },
                TableAddress {
                    name: "wrong_table".to_string(),
                    full_path: "test_schema.wrong_table".to_string(),
                },
            ]
        );

        let views = db.list_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "test_schema.test_view".to_string(),
                },
                TableAddress {
                    name: "wrong_view".to_string(),
                    full_path: "test_schema.wrong_view".to_string(),
                },
            ]
        );

        let columns = db.list_columns("test_schema.test_table").await.unwrap();
        assert_eq!(columns, vec!["id", "name"]);

        let result = db
            .query("SELECT * FROM test_schema.test_table")
            .await
            .unwrap();
        assert_eq!(result.columns, vec!["id", "name"]);
        assert_eq!(result.rows, vec![vec!["1", "test"], vec!["2", "rubbish"]]);
    }

    // TODO Need to test this with and without schema
    #[tokio::test]
    async fn test_foreign_relationship_test_with_schema() {
        let database = DuckDB::new_in_memory(Some("test_schema".to_string())).unwrap();

        database.exec("CREATE SCHEMA other_schema").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE test_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO test_schema.test_table VALUES (3, 'test_3'), (4, 'rubbish_rubiish')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                ("models/test_model.sql", "SELECT id FROM q.test_source"),
                (
                    "quary.yaml",
                    r#"
                    duckdbInMemory: {}
                "#,
                ),
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
      path: test_schema.test_table
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
                (
                    "tests/test_model_out_is_unique.sql",
                    "SELECT id, COUNT(*)
FROM q.test_model_out
GROUP BY id
HAVING COUNT(*) > 1;",
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

        let project = parse_project(&file_system, &database.query_generator(), "").unwrap();

        let tests = return_tests_sql(
            &database.query_generator(),
            &project,
            &file_system,
            true,
            None,
            None,
        )
        .unwrap();
        let tests = tests.iter().collect::<Vec<_>>();

        assert!(!tests.is_empty());

        for (name, test) in tests.iter() {
            let results = database.query(test).await.unwrap();

            assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
        }
    }

    #[tokio::test]
    async fn test_foreign_relationship_test_without_schema() {
        let database = DuckDB::new_in_memory(None).unwrap();

        database.exec("CREATE SCHEMA other_schema").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();
        database
            .exec("CREATE TABLE test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO test_table VALUES (3, 'test_3'), (4, 'rubbish_rubiish')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                (
                    "quary.yaml",
                    r#"
                    duckdbInMemory: {}
                "#,
                ),
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
      path: test_table
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
                (
                    "tests/test_model_out_is_unique.sql",
                    "SELECT id, COUNT(*)
FROM q.test_model_out
GROUP BY id
HAVING COUNT(*) > 1;",
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

        let project = parse_project(&file_system, &database.query_generator(), "").unwrap();

        let tests = return_tests_sql(
            &database.query_generator(),
            &project,
            &file_system,
            true,
            None,
            None,
        )
        .unwrap();
        let tests = tests.iter().collect::<Vec<_>>();

        assert!(!tests.is_empty());

        for (name, test) in tests.iter() {
            let results = database.query(test).await.unwrap();

            assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
        }
    }

    #[tokio::test]
    async fn test_with_schema_sql_test() {
        let database = DuckDB::new_in_memory(Some("finances".to_string())).unwrap();

        database.exec("CREATE SCHEMA other_schema").await.unwrap();
        database
            .exec("CREATE TABLE other_schema.test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        database
            .exec("INSERT INTO other_schema.test_table VALUES (1, 'test'), (2, 'rubbish')")
            .await
            .unwrap();

        let file_system = FileSystem {
            files: vec![
                ("quary.yaml", "duckdbInMemory: {schema: finances}"),
                ("models/test_model.sql", "SELECT id FROM q.test_source"),
                (
                    "tests/unique_for_column.sql",
                    "SELECT id, COUNT(*) FROM q.test_source GROUP BY id HAVING COUNT(*) > 1",
                ),
                (
                    "models/sources.yaml",
                    "
sources:
  - name: test_source
    path: other_schema.test_table
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

        let project = parse_project(&file_system, &database.query_generator(), "").unwrap();
        let sqls = project_and_fs_to_sql_for_views(
            &project,
            &file_system,
            &database.query_generator(),
            false,
            false,
        )
        .unwrap();
        for sql in sqls {
            for sql in sql.1 {
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
        .unwrap();
        assert_eq!(tests.len(), 1);

        for (name, test) in tests.iter() {
            println!("Running test {}: {}", name, test);
            let results = database.query(test).await.unwrap();
            assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
        }
    }
}
