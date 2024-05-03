use chrono::{DateTime, Utc};
use duckdb::arrow::array::{array, Array};
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::{params, Connection};
use quary_core::database_duckdb::DatabaseQueryGeneratorDuckDB;
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::TableAddress;
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

    fn default_schema() -> &'static str {
        "main"
    }
}

#[async_trait::async_trait]
impl DatabaseConnection for DuckDB {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let results =  self.query("SELECT table_schema, table_name FROM information_schema.tables WHERE table_type='BASE TABLE' ORDER BY table_schema, table_name")
                .await
                .map_err(
                    |e| format!("Failed to get tables from DuckDB: {:?}", e))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[0].clone(),
                full_path: format!("{}.{}", row[0].clone(), row[1].clone()),
            })
            .collect())
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let results = self.query("SELECT table_schema, table_name FROM information_schema.tables WHERE table_type='VIEW' ORDER BY table_schema, table_name")
            .await
            .map_err(
                |e| format!("Failed to get views from DuckDB: {:?}", e),
            )?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[0].clone(),
                full_path: format!("{}.{}", row[0].clone(), row[1].clone()),
            })
            .collect())
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let schema = self
            .schema
            .clone()
            .unwrap_or(DuckDB::default_schema().to_string());
        let results =
            self.query(&format!(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type='BASE TABLE' ORDER BY table_name",
                schema
            )).await.map_err(|e| format!("Failed to get views from DuckDB: {:?}", e))?;
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

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let schema = self
            .schema
            .clone()
            .unwrap_or(DuckDB::default_schema().to_string());
        let results =
            self.query(&format!(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type='VIEW' ORDER BY table_name",
                schema
            )).await.map_err(|e| format!("Failed to get views from DuckDB: {:?}", e))?;
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

    async fn list_columns(&self, table: &str) -> Result<Vec<ColumnWithDetails>, String> {
        let results = self
            .query(&format!("PRAGMA table_info({})", table))
            .await
            .map_err(|e| format!("Failed to get columns for table {}: {:?}", table, e))?;
        let columns = results
            .rows
            .into_iter()
            .map(|row| (row[1].clone(), Some(row[2].clone())))
            .map(|(name, data_type)| ColumnWithDetails {
                name,
                data_type,
                ..Default::default()
            })
            .collect::<Vec<_>>();
        Ok(columns)
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

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let conn = self.connection.lock().map_err(|e| {
            QueryError::new(
                query.to_string(),
                format!("Failed to get connection lock: {}", e),
            )
        })?;

        let mut stmt = conn.prepare(query).map_err(|e| {
            QueryError::new(query.to_string(), format!("Failed to prepare query: {}", e))
        })?;

        let rbs: Vec<RecordBatch> = stmt
            .query_arrow([])
            .map_err(|e| {
                QueryError::new(query.to_string(), format!("Failed to execute query: {}", e))
            })?
            .collect();

        if rbs.is_empty() {
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

        let mut rows = Vec::new();
        for rb in &rbs {
            let batch_rows = convert_array_to_vec_string(rb.columns())
                .map_err(|e| QueryError::new(query.to_string(), e))?;
            rows.extend(batch_rows);
        }

        Ok(QueryResult {
            columns: columns.into_iter().map(|c| (c, None)).collect(),
            rows,
        })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorDuckDB::new(self.schema.clone(), None))
    }

    async fn table_exists(&self, _path: &str) -> Result<Option<bool>, String> {
        Ok(None) // not implemented
    }
}

pub(crate) fn convert_array_to_vec_string(
    array: &[Arc<dyn Array>],
) -> Result<Vec<Vec<String>>, String> {
    let num_rows = array[0].len();
    let num_columns = array.len();
    let mut rows = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {
        let row = vec!["".to_string(); num_columns];
        rows.push(row);
    }

    for (i, row) in rows.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            let array = &array[j];
            if let Some(string_array) = array.as_any().downcast_ref::<array::StringArray>() {
                *value = string_array.value(i).to_string();
            } else if let Some(int32_array) = array.as_any().downcast_ref::<array::Int32Array>() {
                *value = int32_array.value(i).to_string();
            } else if let Some(int64_array) = array.as_any().downcast_ref::<array::Int64Array>() {
                *value = int64_array.value(i).to_string();
            } else if let Some(float32_array) = array.as_any().downcast_ref::<array::Float32Array>()
            {
                *value = float32_array.value(i).to_string();
            } else if let Some(float64_array) = array.as_any().downcast_ref::<array::Float64Array>()
            {
                *value = float64_array.value(i).to_string();
            } else if let Some(boolean_array) = array.as_any().downcast_ref::<array::BooleanArray>()
            {
                *value = boolean_array.value(i).to_string();
            } else if let Some(date_array) = array.as_any().downcast_ref::<array::Date64Array>() {
                *value = date_array.value(i).to_string();
            } else if let Some(date_array) = array.as_any().downcast_ref::<array::Date32Array>() {
                *value = date_array.value(i).to_string();
            } else if let Some(timestamp_array) = array
                .as_any()
                .downcast_ref::<array::TimestampMicrosecondArray>()
            {
                if timestamp_array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    let timestamp_micros = timestamp_array.value(i);
                    let datetime_utc = DateTime::<Utc>::from_timestamp(
                        timestamp_micros / 1_000_000,
                        (timestamp_micros % 1_000_000) as u32 * 1_000,
                    )
                    .ok_or("error converting timestamp to datetime")?;
                    *value = datetime_utc.format("%Y-%m-%d %H:%M:%S%.6f %Z").to_string();
                }
            } else {
                let array_type = array.data_type();
                return Err(format!("Unsupported array type: {:?}", array_type));
            }
        }
    }

    // Example for a specific array type, e.g., StringArray
    Ok(rows)
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

    #[tokio::test]
    async fn test_list_columns() {
        let db = DuckDB::new_in_memory(None).unwrap();
        db.exec("CREATE TABLE test_table (id INTEGER, name VARCHAR(255))")
            .await
            .unwrap();
        let columns = db.list_columns("test_table").await.unwrap();
        assert_eq!(
            columns,
            vec![
                ColumnWithDetails {
                    name: "id".to_string(),
                    data_type: Some("INTEGER".to_string()),
                    ..Default::default()
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    data_type: Some("VARCHAR".to_string()),
                    ..Default::default()
                }
            ]
        );
    }

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

        let tables = db.list_local_tables().await.unwrap();
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

        let views = db.list_local_views().await.unwrap();
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
        assert_eq!(
            vec![
                ColumnWithDetails {
                    name: "id".to_string(),
                    data_type: Some("INTEGER".to_string()),
                    ..Default::default()
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    data_type: Some("VARCHAR".to_string()),
                    ..Default::default()
                }
            ],
            columns,
        );

        let result = db.query("SELECT * FROM test_table").await.unwrap();
        assert_eq!(
            result
                .columns
                .iter()
                .map(|(column, _)| column)
                .collect::<Vec<_>>(),
            vec!["id", "name"]
        );
        assert_eq!(vec![vec!["1", "test"], vec!["2", "rubbish"]], result.rows);
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

        let tables = db.list_local_tables().await.unwrap();
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

        let views = db.list_local_views().await.unwrap();
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
        assert_eq!(
            vec![
                ColumnWithDetails {
                    name: "id".to_string(),
                    data_type: Some("INTEGER".to_string()),
                    ..Default::default()
                },
                ColumnWithDetails {
                    name: "name".to_string(),
                    data_type: Some("VARCHAR".to_string()),
                    ..Default::default()
                }
            ],
            columns,
        );

        let result = db
            .query("SELECT * FROM test_schema.test_table")
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
        .await
        .unwrap();
        assert_eq!(tests.len(), 1);

        for (name, test) in tests.iter() {
            println!("Running test {}: {}", name, test);
            let results = database.query(test).await.unwrap();
            assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
        }
    }

    #[tokio::test]
    async fn test_snapshot_with_no_time_override() {
        let target_schema = Some("analytics".to_string());
        let database: Box<dyn DatabaseConnection> =
            Box::new(DuckDB::new_in_memory(target_schema.clone()).unwrap());
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
            &DatabaseQueryGeneratorDuckDB::new(target_schema.clone(), None),
            "",
        )
        .await
        .unwrap();

        let snapshots_sql = project_and_fs_to_sql_for_snapshots(
            &project,
            &file_system,
            &DatabaseQueryGeneratorDuckDB::new(target_schema.clone(), None),
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
        // Create orders table

        let target_schema = Some("analytics".to_string());
        let database: Box<dyn DatabaseConnection> =
            Box::new(DuckDB::new_in_memory(target_schema.clone()).unwrap());
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
            DatabaseQueryGeneratorDuckDB::new(target_schema.clone(), Some(system_time));

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

        let project = parse_project(&file_system, &db_generator, "")
            .await
            .unwrap();

        let snapshots_sql = project_and_fs_to_sql_for_snapshots(
            &project,
            &file_system,
            &db_generator,
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
        assert_eq!(
            vec![
                vec![
                    "1",
                    "in_progress",
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "NULL",
                    "77f50225cf5a52d15fecaa449be2dcc4"
                ],
                vec![
                    "2",
                    "completed",
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "NULL",
                    "3bb5cc6bb5b432df7712d067f57a3780"
                ],
            ],
            data.rows,
        );

        database
        .exec("UPDATE jaffle_shop.raw_orders SET status = 'completed', updated_at = '2023-01-01 02:00:00' WHERE order_id = 1")
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
            DatabaseQueryGeneratorDuckDB::new(target_schema, Some(system_time_updated));

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
        let data = database.query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM analytics.orders_snapshot ORDER BY order_id, quary_valid_from").await.unwrap();

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
            vec![
                vec![
                    "1",
                    "in_progress",
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "2023-01-01 03:00:00.000000 UTC",
                    "77f50225cf5a52d15fecaa449be2dcc4"
                ],
                vec![
                    "1",
                    "completed",
                    "2023-01-01 02:00:00.000000 UTC",
                    "2023-01-01 03:00:00.000000 UTC",
                    "NULL",
                    "f5c7798e30814925cd1a61e9e5ef6683"
                ],
                vec![
                    "2",
                    "completed",
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "NULL",
                    "3bb5cc6bb5b432df7712d067f57a3780"
                ],
            ],
            data.rows,
        );

        let columns = database
            .list_columns("analytics.orders_snapshot")
            .await
            .unwrap();
        assert_eq!(6, columns.len());
        assert_eq!(
            Some("TIMESTAMP WITH TIME ZONE".to_string()),
            columns
                .iter()
                .find(|c| c.name == "quary_valid_from")
                .unwrap()
                .data_type
        );
        assert_eq!(
            Some("TIMESTAMP WITH TIME ZONE".to_string()),
            columns
                .iter()
                .find(|c| c.name == "quary_valid_to")
                .unwrap()
                .data_type
        );
    }

    #[tokio::test]
    async fn test_snapshots_without_schema() {
        // Create orders table

        let target_schema = None;
        let database: Box<dyn DatabaseConnection> =
            Box::new(DuckDB::new_in_memory(target_schema.clone()).unwrap());
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
            DatabaseQueryGeneratorDuckDB::new(target_schema.clone(), Some(system_time));

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

        let project = parse_project(&file_system, &db_generator, "")
            .await
            .unwrap();

        let snapshots_sql = project_and_fs_to_sql_for_snapshots(
            &project,
            &file_system,
            &db_generator,
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
        let data = database.query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM orders_snapshot").await.unwrap();

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
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "NULL",
                    "77f50225cf5a52d15fecaa449be2dcc4"
                ],
                vec![
                    "2",
                    "completed",
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "NULL",
                    "3bb5cc6bb5b432df7712d067f57a3780"
                ],
            ]
        );

        database
        .exec("UPDATE jaffle_shop.raw_orders SET status = 'completed', updated_at = '2023-01-01 02:00:00' WHERE order_id = 1")
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
            DatabaseQueryGeneratorDuckDB::new(target_schema, Some(system_time_updated));

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
        let data = database.query("SELECT order_id, status, updated_at, quary_valid_from, quary_valid_to, quary_scd_id FROM orders_snapshot ORDER BY order_id, quary_valid_from").await.unwrap();

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
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "2023-01-01 03:00:00.000000 UTC",
                    "77f50225cf5a52d15fecaa449be2dcc4"
                ],
                vec![
                    "1",
                    "completed",
                    "2023-01-01 02:00:00.000000 UTC",
                    "2023-01-01 03:00:00.000000 UTC",
                    "NULL",
                    "f5c7798e30814925cd1a61e9e5ef6683"
                ],
                vec![
                    "2",
                    "completed",
                    "2023-01-01 00:00:00.000000 UTC",
                    "2023-01-01 01:00:00.000000 UTC",
                    "NULL",
                    "3bb5cc6bb5b432df7712d067f57a3780"
                ],
            ]
        );

        let columns = database.list_columns("orders_snapshot").await.unwrap();
        assert_eq!(6, columns.len());
        assert_eq!(
            Some("TIMESTAMP WITH TIME ZONE".to_string()),
            columns
                .iter()
                .find(|c| c.name == "quary_valid_from")
                .unwrap()
                .data_type
        );
        assert_eq!(
            Some("TIMESTAMP WITH TIME ZONE".to_string()),
            columns
                .iter()
                .find(|c| c.name == "quary_valid_to")
                .unwrap()
                .data_type
        );
    }
}
