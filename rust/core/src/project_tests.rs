use crate::databases::DatabaseQueryGenerator;
use crate::file_system::FileSystem;
use crate::project::{
    create_path_map, project_and_fs_to_query_sql, replace_reference_string_found,
};
use crate::schema_name::DEFAULT_SCHEMA_PREFIX;
use crate::sql::return_reference_search;
use crate::tests::ToSql;
use futures::AsyncReadExt;
use quary_proto::test::TestType::{
    AcceptedValues, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, MultiColumnUnique,
    NotNull, Relationship, Sql, Unique,
};
use quary_proto::{Project, TestRelationship};
use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::pin::Pin;

/// Returns sql tests to run in no particular order but with the name pointing to the test.
///
/// whether_to_make_test_include_models_to_source if set to true allows users to run tests without
/// the full models being deployed to views in the database. Rather than rely on the created views
/// it inserts the required models in above queries to the sql query used for the test.
///
/// apply_limit_to_generated_tests if set to Some(usize) will apply a limit to the number of tests where
/// the tests are generated. This is useful for speeding up the tests to limit the number of returned values.
///
/// filter_by_model if set to Some(&str) will filter the tests to only return tests related to the model.
pub async fn return_tests_sql(
    database: &(impl DatabaseQueryGenerator + Sync),
    project: &Project,
    fs: &impl FileSystem,
    whether_to_make_test_include_models_to_source: bool,
    apply_limit_to_generated_tests: Option<usize>,
    filter_by_model: Option<&str>,
) -> Result<BTreeMap<String, String>, String> {
    let tests = if let Some(model_name) = filter_by_model {
        let model_specific_filter = is_test_related_to_model(model_name.to_string());
        project
            .tests
            .iter()
            .filter(|(_, test)| model_specific_filter(test))
            .collect::<HashMap<_, _>>()
    } else {
        project.tests.iter().collect::<HashMap<_, _>>()
    };

    if whether_to_make_test_include_models_to_source {
        let path_generator =
            |model: String| -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>> {
                Box::pin(async move {
                    let (sql, _) =
                        project_and_fs_to_query_sql(database, project, fs, model.as_str(), None)
                            .await?;
                    Ok(format!("({})", sql))
                })
            };

        let tests = tests
            .into_iter()
            .map(|(name, test)| -> Result<(_, _), String> {
                match &test.test_type {
                    Some(test) => Ok((name, test)),
                    None => Err(format!("test type {:?} is missing", name)),
                }
            })
            .collect::<Result<Vec<(_, _)>, String>>()?
            .into_iter()
            .map(|(name, test_type)| async move {
                let sql: String = match test_type {
                    Unique(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok::<String, String>(sql)
                    }
                    MultiColumnUnique(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    NotNull(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    AcceptedValues(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    GreaterThanOrEqual(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    GreaterThan(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    LessThanOrEqual(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    LessThan(test) => {
                        let mut temp_test = test.clone();
                        temp_test.path = path_generator(temp_test.model.to_string()).await?;
                        let sql = temp_test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    Relationship(test) => {
                        let target_path = path_generator(test.target_model.to_string()).await?;
                        let source_path = path_generator(test.source_model.to_string()).await?;
                        let test = TestRelationship {
                            target_path,
                            source_path,
                            ..test.clone()
                        };
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok(sql)
                    }
                    Sql(test) => {
                        let mut read = fs.read_file(&test.file_path).await.map_err(|err| {
                            format!(
                                "failed to read sql file {:?} for test {:?} with error {:?}",
                                test.file_path, name, err
                            )
                        })?;
                        let mut file = String::new();
                        read.read_to_string(&mut file).await.map_err(|err| {
                            format!(
                                "failed to read sql file {:?} for test {:?} with error {:?}",
                                test.file_path, name, err
                            )
                        })?;
                        let mut sources = project
                            .sources
                            .iter()
                            .map(|(name, source)| (name.clone(), source.path.clone()))
                            .collect::<HashMap<_, _>>();
                        for name in project.models.keys() {
                            let (sql, _) =
                                project_and_fs_to_query_sql(database, project, fs, name, None)
                                    .await?;
                            sources.insert(name.clone(), format!("({})", sql));
                        }
                        let reference_search = return_reference_search(DEFAULT_SCHEMA_PREFIX)
                            .map_err(|e| format!("failed to return reference search: {}", e))?;
                        let sql = reference_search.clone().replace_all(
                            file.as_str(),
                            replace_reference_string_found(&sources, &database),
                        );
                        Ok(sql.to_string())
                    }
                }?;
                Ok((name.to_string(), sql))
            });

        let test_results = futures::future::try_join_all(tests)
            .await
            .map_err(|e: String| format!("failed to collect test results: {}", e))?;

        let tests = test_results.into_iter().collect::<BTreeMap<_, _>>();
        Ok(tests)
    } else {
        let tests = tests
            .into_iter()
            .map(|(name, test)| -> Result<(_, _), String> {
                match &test.test_type {
                    Some(test) => Ok((name.clone(), test)),
                    None => Err(format!("test type {:?} is missing", name)),
                }
            })
            .collect::<Result<Vec<(_, _)>, String>>()?
            .into_iter()
            .map(|(name, test_type)| async move {
                match test_type {
                    Unique(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    NotNull(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    AcceptedValues(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    Relationship(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    GreaterThanOrEqual(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    GreaterThan(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    LessThanOrEqual(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    LessThan(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    MultiColumnUnique(test) => {
                        let sql = test.to_sql(apply_limit_to_generated_tests);
                        Ok((name, sql))
                    }
                    Sql(test) => {
                        let mut read = fs.read_file(&test.file_path).await.map_err(|err| {
                            format!(
                                "failed to read sql file {:?} for test {:?} with error {:?}",
                                test.file_path, name, err
                            )
                        })?;
                        let mut file = String::new();
                        read.read_to_string(&mut file).await.map_err(|err| {
                            format!(
                                "failed to read sql file {:?} for test {:?} with error {:?}",
                                test.file_path, name, err
                            )
                        })?;
                        let path_map = create_path_map(
                            database,
                            project.models.values().collect(),
                            project.sources.values().collect(),
                            project.snapshots.values().collect(),
                        );
                        let reference_search = return_reference_search(DEFAULT_SCHEMA_PREFIX)
                            .map_err(|e| format!("failed to return reference search: {}", e))?;
                        let sql = reference_search.replace_all(
                            &file,
                            replace_reference_string_found(&path_map, &database),
                        );
                        Ok((name, sql.to_string()))
                    }
                }
            });

        let test_results = futures::future::try_join_all(tests)
            .await
            .map_err(|e: String| format!("failed to collect test results: {}", e))?;

        let tests = test_results
            .into_iter()
            .map(|(name, sql)| (name.clone(), sql))
            .collect::<BTreeMap<_, _>>();

        Ok(tests)
    }
}

/// Returns a filter used to check if a particular test pertains to the provided model.
///
/// This function currently only checks the model at node-level and does not look for tests
/// which may be related up the dependency-tree.
fn is_test_related_to_model(model_name: String) -> impl Fn(&quary_proto::Test) -> bool {
    move |test: &quary_proto::Test| {
        if let Some(test_type) = &test.test_type {
            match test_type {
                Unique(test) => test.model == model_name,
                NotNull(test) => test.model == model_name,
                AcceptedValues(test) => test.model == model_name,
                GreaterThanOrEqual(test) => test.model == model_name,
                GreaterThan(test) => test.model == model_name,
                LessThanOrEqual(test) => test.model == model_name,
                LessThan(test) => test.model == model_name,
                Relationship(test) => test.source_model == model_name,
                MultiColumnUnique(test) => test.model == model_name,
                Sql(test) => test
                    .references
                    .iter()
                    .any(|ref_model| ref_model == &model_name),
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database_bigquery::DatabaseQueryGeneratorBigQuery;
    use crate::database_sqlite::DatabaseQueryGeneratorSqlite;
    use crate::project::parse_project;
    use crate::project_tests::{is_test_related_to_model, return_tests_sql};
    use crate::tests::test_to_name;
    use prost::bytes::Bytes;
    use quary_proto::test::TestType;
    use quary_proto::{
        File, FileSystem, Project, Test, TestAcceptedValues, TestNotNull, TestRelationship,
        TestSqlFile, TestUnique,
    };
    use std::collections::{BTreeMap, HashMap};

    #[tokio::test]
    async fn return_tests_sql_no_including_model() {
        let fs = FileSystem::default();
        let test = Test {
            test_type: Some(TestType::NotNull(TestNotNull {
                file_path: "models/schema.yaml".to_string(),
                model: "test_model".to_string(),
                path: "project_1.dataset_1.test_model".to_string(),
                column: "column_a".to_string(),
            })),
        };
        let project = Project {
            seeds: Default::default(),
            models: Default::default(),
            snapshots: Default::default(),
            tests: HashMap::from([(test_to_name(&test).unwrap(), test.clone())]),
            sources: Default::default(),
            project_files: Default::default(),
            connection_config: Default::default(),
        };
        let database = DatabaseQueryGeneratorSqlite::default();

        let results = return_tests_sql(&database, &project, &fs, false, None, None)
            .await
            .unwrap();

        assert_eq!(
            results,
            BTreeMap::from([(
                test_to_name(&test).unwrap(),
                "SELECT * FROM project_1.dataset_1.test_model WHERE column_a IS NULL".to_string()
            )])
        );
    }

    #[tokio::test]
    async fn return_test_sql_with_including_model_path_root_seed() {
        let fs = FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    File {
                        name: "models/schema.yaml".to_string(),
                        contents: Bytes::from(
                            "
models:
  - name: intermediary_model
    columns:
      - name: a
        tests:
          - type: not_null
",
                        ),
                    },
                ),
                (
                    "seeds/test_seed.csv".to_string(),
                    File {
                        name: "seeds/test_seed.csv".to_string(),
                        contents: Bytes::from("column_b,a\n1,1"),
                    },
                ),
                (
                    "models/intermediary_model.sql".to_string(),
                    File {
                        name: "models/intermediary_model.sql".to_string(),
                        contents: Bytes::from("SELECT a FROM q.test_seed"),
                    },
                ),
            ]),
        };
        let database = DatabaseQueryGeneratorSqlite::default();
        let project = parse_project(&fs, &database, "").await.unwrap();

        let results = return_tests_sql(&database, &project, &fs, true, None, None)
            .await
            .unwrap();

        assert_eq!(
            results,
            BTreeMap::from([(
                "test_intermediary_model_a_not_null".to_string(),
                "SELECT * FROM (WITH test_seed AS (SELECT column1 AS column_b,column2 AS a FROM (VALUES ('1','1'))) SELECT * FROM (SELECT a FROM `test_seed`) AS alias) WHERE a IS NULL".to_string()
            )])
        );
    }

    #[tokio::test]
    async fn return_test_sql_with_including_model_path_root_source() {
        let fs = FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    File {
                        name: "models/schema.yaml".to_string(),
                        contents: Bytes::from(
                            "
sources: 
  - name: source_1
    path: project_2.dataset_2.table_2
models: 
  - name: intermediary_model
    columns: 
      - name: a
        tests: 
          - type: not_null
",
                        ),
                    },
                ),
                (
                    "models/intermediary_model.sql".to_string(),
                    File {
                        name: "models/intermediary_model.sql".to_string(),
                        contents: Bytes::from("SELECT a FROM q.source_1"),
                    },
                ),
            ]),
        };
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_1".to_string(), "dataset_1".to_string());
        let project = parse_project(&fs, &database, "").await.unwrap();

        let results = return_tests_sql(&database, &project, &fs, true, None, None)
            .await
            .unwrap();

        assert_eq!(
            results,
            BTreeMap::from([(
                "test_intermediary_model_a_not_null".to_string(),
                "SELECT * FROM (WITH source_1 AS (SELECT * FROM project_2.dataset_2.table_2) SELECT * FROM (SELECT a FROM `source_1`) AS alias) WHERE a IS NULL".to_string(),
            )])
        );
    }

    #[tokio::test]
    async fn return_model_test_sql_with_including_model_path_root_seed() {
        let fs = FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    File {
                        name: "models/schema.yaml".to_string(),
                        contents: Bytes::from(
                            "
models:
  - name: model_a
    columns:
      - name: a
        tests:
          - type: not_null
",
                        ),
                    },
                ),
                (
                    "seeds/test_seed.csv".to_string(),
                    File {
                        name: "seeds/test_seed.csv".to_string(),
                        contents: Bytes::from("column_b,a\n1,1"),
                    },
                ),
                (
                    "models/model_a.sql".to_string(),
                    File {
                        name: "models/model_a.sql".to_string(),
                        contents: Bytes::from("SELECT a FROM q.test_seed"),
                    },
                ),
            ]),
        };
        let database = DatabaseQueryGeneratorSqlite::default();
        let project = parse_project(&fs, &database, "").await.unwrap();

        let results = return_tests_sql(&database, &project, &fs, true, None, Some("model_a"))
            .await
            .unwrap();

        assert_eq!(
            results,
            BTreeMap::from([(
                "test_model_a_a_not_null".to_string(),
                "SELECT * FROM (WITH test_seed AS (SELECT column1 AS column_b,column2 AS a FROM (VALUES ('1','1'))) SELECT * FROM (SELECT a FROM `test_seed`) AS alias) WHERE a IS NULL".to_string()
            )])
        );
    }

    #[tokio::test]
    async fn return_model_test_sql_with_including_model_path_root_source() {
        let fs = FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    File {
                        name: "models/schema.yaml".to_string(),
                        contents: Bytes::from(
                            "
sources: 
  - name: source_1
    path: project_2.dataset_2.table_2
models: 
  - name: model_a
    columns: 
      - name: a
        tests: 
          - type: not_null
",
                        ),
                    },
                ),
                (
                    "models/model_a.sql".to_string(),
                    File {
                        name: "models/model_a.sql".to_string(),
                        contents: Bytes::from("SELECT a FROM q.source_1"),
                    },
                ),
            ]),
        };
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_1".to_string(), "dataset_1".to_string());
        let project = parse_project(&fs, &database, "").await.unwrap();

        let results = return_tests_sql(&database, &project, &fs, true, None, Some("model_a"))
            .await
            .unwrap();

        assert_eq!(
            results,
            BTreeMap::from([(
                "test_model_a_a_not_null".to_string(),
                "SELECT * FROM (WITH source_1 AS (SELECT * FROM project_2.dataset_2.table_2) SELECT * FROM (SELECT a FROM `source_1`) AS alias) WHERE a IS NULL".to_string()
            )])
        );
    }

    #[tokio::test]
    async fn is_test_related_to_model_filters() {
        let tests_for_model_a = vec![
            Test {
                test_type: Some(TestType::Unique(TestUnique {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_a".to_string(),
                    ..Default::default()
                })),
            },
            Test {
                test_type: Some(TestType::Sql(TestSqlFile {
                    file_path: "sql_test_a.sql".to_string(),
                    references: vec!["model_a".to_string()], // References model_a
                    ..Default::default()
                })),
            },
            Test {
                test_type: Some(TestType::NotNull(TestNotNull {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_a".to_string(),
                    ..Default::default()
                })),
            },
            Test {
                test_type: Some(TestType::AcceptedValues(TestAcceptedValues {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_a".to_string(),
                    ..Default::default()
                })),
            },
            Test {
                test_type: Some(TestType::Relationship(TestRelationship {
                    source_model: "model_a".to_string(),
                    ..Default::default()
                })),
            },
        ];
        let tests_for_model_b = vec![
            Test {
                test_type: Some(TestType::NotNull(TestNotNull {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_b".to_string(),
                    ..Default::default()
                })),
            },
            Test {
                test_type: Some(TestType::Sql(TestSqlFile {
                    file_path: "sql_test_a.sql".to_string(),
                    references: vec!["model_b".to_string()], // References model_a
                    ..Default::default()
                })),
            },
        ];

        let combined_tests = [&tests_for_model_a[..], &tests_for_model_b[..]].concat();

        let filter_model_a = is_test_related_to_model("model_a".to_string());
        let filtered_tests_for_model_a: Vec<_> = combined_tests
            .iter()
            .filter(|test| filter_model_a(test))
            .collect();

        let filter_model_b = is_test_related_to_model("model_b".to_string());
        let filtered_tests_for_model_b: Vec<_> = combined_tests
            .iter()
            .filter(|test| filter_model_b(test))
            .collect();

        assert_eq!(
            filtered_tests_for_model_a,
            tests_for_model_a.iter().collect::<Vec<_>>()
        );
        assert_eq!(
            filtered_tests_for_model_b,
            tests_for_model_b.iter().collect::<Vec<_>>()
        );
    }

    // source_a which model_a which model a points to
    // source_b which model_b which model b points to
    // sql test joins in both model_a and model_b
    #[tokio::test]
    async fn return_test_sql_with_including_model_path_root_source_sql_test() {
        let fs = FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    File {
                        name: "models/schema.yaml".to_string(),
                        contents: Bytes::from(
                            "models:
  - name: model_a
  - name: model_b
sources:
  - name: source_a
    path: project_2.dataset_2.table_2
  - name: source_b
    path: project_3.dataset_3.table_3
")
                    }
                ),
                (
                    "models/model_a.sql".to_string(),
                    File {
                        name: "models/model_a.sql".to_string(),
                        contents: Bytes::from("SELECT * FROM q.source_a")
                    }
                ),
                (
                    "models/model_b.sql".to_string(),
                    File {
                        name: "models/model_b.sql".to_string(),
                        contents: Bytes::from("SELECT * FROM q.source_b")
                    }
                ),
                (
                    "tests/model_a_and_model_b.sql".to_string(),
                    File {
                        name: "tests/model_a_and_model_b.sql".to_string(),
                        contents: Bytes::from("SELECT * FROM q.model_a a JOIN q.model_b b ON a.column_a = b.column_b WHERE column_a IS NULL OR column_b IS NULL")
                    }
                )]
            )
        };
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_1".to_string(), "dataset_1".to_string());
        let project = parse_project(&fs, &database, "").await.unwrap();

        let results = return_tests_sql(&database, &project, &fs, true, None, None)
            .await
            .unwrap();

        assert_eq!(
            results,
            BTreeMap::from([(
                "test_sql_model_a_and_model_b".to_string(),
                "SELECT * FROM (WITH source_a AS (SELECT * FROM project_2.dataset_2.table_2) SELECT * FROM (SELECT * FROM `source_a`) AS alias) a JOIN (WITH source_b AS (SELECT * FROM project_3.dataset_3.table_3) SELECT * FROM (SELECT * FROM `source_b`) AS alias) b ON a.column_a = b.column_b WHERE column_a IS NULL OR column_b IS NULL".to_string(),
            )])
        );
    }

    #[tokio::test]
    async fn return_test_sql_with_including_model_path_root_source_sql_test_relationship_test() {
        let fs = FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    File {
                        name: "models/schema.yaml".to_string(),
                        contents: Bytes::from(
                            "models:
  - name: model_a
  - name: model_b
    columns:
      - name: column_a
        tests:
          - type: relationship
            info: 
              model: model_a
              column: column_a
sources:
  - name: source_a
    path: project_2.dataset_2.table_2
  - name: source_b
    path: project_3.dataset_3.table_3
",
                        ),
                    },
                ),
                (
                    "models/model_a.sql".to_string(),
                    File {
                        name: "models/model_a.sql".to_string(),
                        contents: Bytes::from("SELECT * FROM q.source_a"),
                    },
                ),
                (
                    "models/model_b.sql".to_string(),
                    File {
                        name: "models/model_b.sql".to_string(),
                        contents: Bytes::from("SELECT * FROM q.source_b"),
                    },
                ),
            ]),
        };
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_1".to_string(), "dataset_1".to_string());
        let project = parse_project(&fs, &database, "").await.unwrap();

        let results = return_tests_sql(&database, &project, &fs, true, None, None)
            .await
            .unwrap();

        assert_eq!(
            results,
            BTreeMap::from([(
                "test_model_b_column_a_relationship_model_a_column_a".to_string(),
                "SELECT * FROM (WITH source_b AS (SELECT * FROM project_3.dataset_3.table_3) SELECT * FROM (SELECT * FROM `source_b`) AS alias) AS alias WHERE column_a IS NOT NULL AND column_a NOT IN (SELECT column_a FROM (WITH source_a AS (SELECT * FROM project_2.dataset_2.table_2) SELECT * FROM (SELECT * FROM `source_a`) AS alias) AS alias)".to_string()
            )])
        );
    }
}
