use async_trait::async_trait;
use quary_core::database_sqlite::DatabaseQueryGeneratorSqlite;
use quary_core::databases::{ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, IndexWithDetails, QueryError, QueryResult};
use quary_proto::TableAddress;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Column;
use sqlx::Pool;
use sqlx::Row;
use sqlx::TypeInfo;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Sqlite {
    pool: Pool<sqlx::Sqlite>,
}

impl Sqlite {
    pub async fn new_in_memory() -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
        Ok(Self { pool })
    }

    pub async fn new_with_file(path: &str) -> Result<Self, sqlx::Error> {
        let pool = Pool::connect(&format!("sqlite://{}", path)).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl DatabaseConnection for Sqlite {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        return self.list_local_tables().await;
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        return self.list_local_views().await;
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
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

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='view'")
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

    async fn list_columns(&self, table: &str) -> Result<Vec<ColumnWithDetails>, String> {
        let rows = sqlx::query(&format!("PRAGMA table_info({})", table))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut columns = Vec::new();

        for row in rows {
            let column_name: String = row.get(1);
            columns.push(column_name);
        }

        let columns = columns
            .into_iter()
            .map(|name| ColumnWithDetails {
                name,
                ..Default::default()
            })
            .collect::<Vec<ColumnWithDetails>>();

        Ok(columns)
    }

    async fn list_indexes(&self, _table: &str) -> Result<Vec<IndexWithDetails>, String> {
        todo!("X is unfamiliar with sqlite")
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        let query = sqlx::query(query);
        query.execute(&self.pool).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let query_builder = sqlx::query(query);

        let sqlite_rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| QueryError::new(query.to_string(), e.to_string()))?;

        if sqlite_rows.is_empty() {
            return Ok(QueryResult {
                columns: Vec::new(),
                rows: Vec::new(),
            });
        }

        let mut columns = Vec::new();
        for i in 0..sqlite_rows[0].len() {
            let column_name = sqlite_rows[0].column(i).name();
            columns.push(column_name.to_string());
        }

        let mut rows = Vec::<Vec<String>>::new();
        for row in sqlite_rows {
            let mut row_vec = Vec::new();
            for i in 0..row.len() {
                let value: String = match row.column(i).type_info().name() {
                    "INTEGER" => row.try_get::<i32, _>(i).unwrap_or(0).to_string(),
                    _ => row
                        .try_get::<String, _>(i)
                        .unwrap_or_else(|_| String::from("")),
                };
                row_vec.push(value);
            }
            rows.push(row_vec);
        }

        Ok(QueryResult {
            columns: columns.into_iter().map(|c| (c, None)).collect(),
            rows,
        })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorSqlite {})
    }

    async fn table_exists(&self, _path: &str) -> Result<Option<bool>, String> {
        Ok(None) // not implemented
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::bytes::Bytes;
    use quary_core::init::{
        init_to_file_system, Asset, INIT_FOLDER_NUMBER_OF_TESTS,
        INIT_FOLDER_NUMBER_OF_TESTS_THAT_ARE_RUN, INIT_FOLDER_NUMBER_OF_TESTS_THAT_FAIL,
    };
    use quary_core::project::parse_project;
    use quary_core::project_tests::return_tests_sql;
    use quary_core::project_to_sql::{
        project_and_fs_to_query_sql, project_and_fs_to_sql_for_views,
    };
    use quary_core::test_runner::{run_tests_internal, RunReturnResult, RunStatementFunc};
    use quary_proto::passed::Reason;
    use quary_proto::test_result::TestResult;
    use quary_proto::{File, TestRunner};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_new_database_in_memory() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        sqlite
            .exec("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .unwrap();

        let values = sqlite.list_local_tables().await.unwrap();

        assert_eq!(
            values,
            vec![TableAddress {
                name: "test".to_string(),
                full_path: "test".to_string(),
            }]
        );
    }

    #[tokio::test]
    async fn test_list_tables() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        sqlite
            .exec("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .unwrap();

        let values = sqlite.list_local_tables().await.unwrap();

        assert_eq!(
            values,
            vec![TableAddress {
                name: "test".to_string(),
                full_path: "test".to_string(),
            }]
        );
    }

    #[tokio::test]
    async fn test_list_views() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        sqlite.exec("CREATE VIEW test AS SELECT 1").await.unwrap();

        let values = sqlite.list_local_views().await.unwrap();

        assert_eq!(
            values,
            vec![TableAddress {
                name: "test".to_string(),
                full_path: "test".to_string(),
            }]
        );
    }

    #[tokio::test]
    async fn test_list_columns() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        sqlite
            .exec("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .unwrap();

        let values = sqlite.list_columns("test").await.unwrap();

        assert_eq!(
            values,
            vec!["id", "name"]
                .into_iter()
                .map(|name| {
                    ColumnWithDetails {
                        name: name.to_string(),
                        ..Default::default()
                    }
                })
                .collect::<Vec<ColumnWithDetails>>()
        );
    }

    #[tokio::test]
    async fn test_exec() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        sqlite
            .exec("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .unwrap();

        let values = sqlite.list_columns("test").await.unwrap();

        assert_eq!(
            values,
            vec!["id", "name"]
                .into_iter()
                .map(|name| {
                    ColumnWithDetails {
                        name: name.to_string(),
                        ..Default::default()
                    }
                })
                .collect::<Vec<ColumnWithDetails>>()
        );
    }

    #[tokio::test]
    async fn test_query() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        sqlite
            .exec("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .unwrap();

        sqlite
            .exec("INSERT INTO test (name) VALUES ('test')")
            .await
            .unwrap();

        let values = sqlite
            .query("SELECT * FROM test WHERE name = \"test\"")
            .await
            .unwrap();

        assert_eq!(
            values
                .columns
                .iter()
                .map(|(column, _)| column)
                .collect::<Vec<_>>(),
            vec!["id", "name"]
        );
        assert_eq!(values.rows, vec![vec!["1", "test"]]);
    }

    #[tokio::test]
    async fn test_projects_amd_fs_to_sql_for_views_applied_check_after_each_model() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        let file_system = Asset {};
        let query_generator = sqlite.query_generator();
        let project = parse_project(&file_system, &query_generator, "")
            .await
            .unwrap();

        let sqls =
            project_and_fs_to_sql_for_views(&project, &file_system, &query_generator, false, false)
                .await
                .unwrap();

        assert!(!sqls.is_empty());

        for (model_name, sql) in sqls {
            for sql in sql.clone() {
                sqlite.exec(&sql).await.unwrap();
            }
            let statement = format!("SELECT * FROM {}", model_name);
            sqlite.query(&statement).await.unwrap_or_else(|_| {
                panic!(
                    "for model {} querying statement '{}' having submitted sql {:?}",
                    model_name, statement, &sql
                )
            });
        }
    }

    #[tokio::test]
    async fn test_run_all_tests_with_source() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();
        let database = sqlite.query_generator();

        let file_system = Asset {};
        let query_generator = sqlite.query_generator();
        let project = parse_project(&file_system, &query_generator, "")
            .await
            .unwrap();

        let tests = return_tests_sql(&database, &project, &file_system, true, None, None)
            .await
            .unwrap();
        let tests = tests.iter().collect::<Vec<_>>();

        assert!(!tests.is_empty());

        for (name, test) in tests.iter() {
            let results = sqlite.query(test).await.unwrap();

            println!("{:?}", results);
            assert_eq!(results.rows.len(), 0, "test {} failed: {}", name, test);
        }
    }

    #[tokio::test]
    async fn test_projects_amd_fs_to_sql_for_views_applied_tested_run_all() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        let file_system = Asset {};
        let query_generator = sqlite.query_generator();
        let project = parse_project(&file_system, &query_generator, "")
            .await
            .unwrap();

        let sqls =
            project_and_fs_to_sql_for_views(&project, &file_system, &query_generator, false, false)
                .await
                .unwrap();

        assert!(!sqls.is_empty());

        for (_, sql) in sqls {
            for sql in sql.clone() {
                sqlite.exec(&sql).await.unwrap();
            }
        }

        let database = Arc::new(sqlite);
        let func: RunStatementFunc = Box::new(move |sql: &str| {
            let database = Arc::clone(&database);
            let sql = sql.to_owned();

            Box::pin(async move {
                let result = database.query(&sql).await;
                match result {
                    Ok(outs) => {
                        if outs.rows.is_empty() {
                            Ok(RunReturnResult::Passed)
                        } else {
                            let proto = outs.to_proto()?;
                            Ok(RunReturnResult::QueryResult(proto))
                        }
                    }
                    Err(error) => Err(format!("Error in query: \n{:?}\n{}", error, sql)),
                }
            })
        });

        let results = run_tests_internal(
            &query_generator,
            &file_system,
            &project,
            query_generator.get_dialect(),
            TestRunner::All,
            func,
            false,
            None,
        )
            .await
            .unwrap();

        assert_eq!(
            results
                .results
                .iter()
                .filter(|r| matches!(r.test_result, Some(TestResult::Failed(_),)))
                .count(),
            INIT_FOLDER_NUMBER_OF_TESTS_THAT_FAIL
        );
        assert_eq!(results.results.len(), INIT_FOLDER_NUMBER_OF_TESTS);
        let ran_tests = results
            .results
            .iter()
            .filter(|result| match &result.test_result {
                Some(TestResult::Passed(passed)) => matches!(passed.reason, Some(Reason::Ran(_))),
                _ => false,
            })
            .count();
        assert_eq!(ran_tests, results.results.len());
    }

    #[tokio::test]
    async fn test_projects_amd_fs_to_sql_for_views_applied_tested_run_skip() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        let file_system = Asset {};
        let query_generator = sqlite.query_generator();
        let project = parse_project(&file_system, &query_generator, "")
            .await
            .unwrap();

        let sqls =
            project_and_fs_to_sql_for_views(&project, &file_system, &query_generator, false, false)
                .await
                .unwrap();

        assert!(!sqls.is_empty());

        for (_, sql) in sqls {
            for sql in sql.clone() {
                sqlite.exec(&sql).await.unwrap();
            }
        }

        let database = Arc::new(sqlite);
        let func: RunStatementFunc = Box::new(move |sql: &str| {
            let database = Arc::clone(&database);
            let sql = sql.to_owned();

            Box::pin(async move {
                let result = database.query(&sql).await;
                match result {
                    Ok(outs) => {
                        if outs.rows.is_empty() {
                            Ok(RunReturnResult::Passed)
                        } else {
                            let proto = outs.to_proto()?;
                            Ok(RunReturnResult::QueryResult(proto))
                        }
                    }
                    Err(error) => Err(format!("Error in query: {:?}", error)),
                }
            })
        });

        let results = run_tests_internal(
            &query_generator,
            &file_system,
            &project,
            query_generator.get_dialect(),
            TestRunner::Skip,
            func,
            false,
            None,
        )
            .await
            .unwrap();

        let failed_results = results
            .results
            .iter()
            .filter(|r| matches!(r.test_result, Some(TestResult::Failed(_),)))
            .collect::<Vec<_>>();
        println!("{:?}", failed_results);
        assert_eq!(failed_results.len(), INIT_FOLDER_NUMBER_OF_TESTS_THAT_FAIL);
        assert_eq!(results.results.len(), INIT_FOLDER_NUMBER_OF_TESTS);
        let ran_tests = results
            .results
            .iter()
            .filter(|result| match &result.test_result {
                Some(TestResult::Passed(passed)) => matches!(passed.reason, Some(Reason::Ran(_))),
                _ => false,
            })
            .count();

        assert_eq!(INIT_FOLDER_NUMBER_OF_TESTS_THAT_ARE_RUN, ran_tests);
    }

    #[tokio::test]
    async fn test_working_with_sources() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();
        sqlite
            .exec("CREATE TABLE source_path (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .unwrap();
        let mut file_system = init_to_file_system();

        let new_model = "model_with_source";
        file_system.files.insert(
            "models/sources/schema.yaml".to_string(),
            File {
                name: "models/sources/schema.yaml".to_string(),
                contents: Bytes::from(
                    r#"
sources:
  - name: source_name_but_not_path
    description: description for source
    path: source_path
                        "#
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                ),
            },
        );
        file_system.files.insert(
            format!("models/sources/{}.sql", new_model),
            File {
                name: format!("models/sources/{}.sql", new_model),
                contents: Bytes::from(
                    r#"
                    SELECT * FROM q.source_name_but_not_path
                    "#
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                ),
            },
        );

        let project = parse_project(&file_system, &sqlite.query_generator(), "")
            .await
            .unwrap();

        // assertions about project
        // assertions about source
        assert_eq!(project.sources.len(), 1);
        assert_eq!(
            project.sources.iter().next().unwrap().1.name,
            "source_name_but_not_path"
        );
        assert_eq!(project.sources.iter().next().unwrap().1.path, "source_path");
        assert_eq!(
            project.sources.iter().next().unwrap().1.description,
            Some("description for source".to_string())
        );
        // assertions about model
        assert!(project.models.contains_key(new_model));
        assert_eq!(
            project.models.get(new_model).unwrap().references,
            vec!["source_name_but_not_path"]
        );

        let query_generator = sqlite.query_generator();
        let sqls =
            project_and_fs_to_sql_for_views(&project, &file_system, &query_generator, false, false)
                .await
                .unwrap();

        // assert
        // assert can find new model
        let new_models_sql: Vec<&Vec<String>> = sqls
            .iter()
            .filter(|(model_name, _)| model_name == new_model)
            .map(|(_, sql)| sql)
            .collect();
        assert_eq!(new_models_sql.len(), 1);

        // asser that can get and apply each successfully
        for (model_name, sql) in sqls {
            for sql in sql.clone() {
                sqlite.exec(&sql).await.unwrap();
            }
            let statement = format!("SELECT * FROM {}", model_name);
            sqlite.query(&statement).await.unwrap_or_else(|_| {
                panic!(
                    "for model {} querying statement '{}' having submitted sql {:?}",
                    model_name, statement, &sql
                )
            });
        }
    }

    #[tokio::test]
    async fn test_getting_each_model_source_seeds() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();
        sqlite
            .exec("CREATE TABLE source_path (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .unwrap();

        let mut file_system = init_to_file_system();

        let new_model = "model_with_source";
        file_system.files.insert(
            "models/sources/schema.yaml".to_string(),
            File {
                name: "models/sources/schema.yaml".to_string(),
                contents: Bytes::from(
                    r#"
sources:
  - name: source_name_but_not_path
    description: description for source
    path: source_path
                        "#
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                ),
            },
        );
        file_system.files.insert(
            format!("models/sources/{}.sql", new_model),
            File {
                name: format!("models/sources/{}.sql", new_model),
                contents: Bytes::from(
                    r#"
                    SELECT * FROM q.source_name_but_not_path
                    "#
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                ),
            },
        );

        let project = parse_project(&file_system, &sqlite.query_generator(), "")
            .await
            .unwrap();

        let model_names = project
            .seeds
            .keys()
            .chain(project.sources.keys())
            .chain(project.models.keys())
            .collect::<Vec<&String>>();

        // asser that can get and apply each successfully
        for model_name in model_names {
            let (sql, _) = project_and_fs_to_query_sql(
                &sqlite.query_generator(),
                &project,
                &file_system,
                model_name,
                None,
            )
                .await
                .unwrap();

            sqlite.exec(&sql).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_getting_each_model_where_one_level_deep_is_with() {
        let sqlite = Sqlite::new_in_memory().await.unwrap();

        let mut file_system = init_to_file_system();

        let new_model = "one_level_model_with_with";
        file_system.files.insert(
            "models/new_model/schema.yml".to_string(),
            File {
                name: "models/new_model/schema.yml".to_string(),
                contents: Bytes::from(
                    format!(
                        r#"
models:
  - name: {}
                        "#,
                        new_model
                    )
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                ),
            },
        );
        file_system.files.insert(
            format!("models/new_model/{}.sql", new_model),
            File {
                name: format!("models/sources/{}.sql", new_model),
                contents: Bytes::from(
                    r#"
                    WITH shifts_intermediary AS (SELECT * FROM q.raw_shifts) SELECT * FROM shifts_intermediary
                    "#
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                ),
            },
        );

        let project = parse_project(&file_system, &sqlite.query_generator(), "")
            .await
            .unwrap();

        // asser that can get and apply each successfully
        let (sql, _) = project_and_fs_to_query_sql(
            &sqlite.query_generator(),
            &project,
            &file_system,
            new_model,
            None,
        )
            .await
            .unwrap();

        sqlite.exec(&sql).await.unwrap();
    }
}
