use crate::databases::DatabaseQueryGenerator;
use crate::file_system::FileSystem;
use crate::inference::{Inference, InferenceTestRunnerAction};
use crate::project::{
    project_and_fs_to_query_sql, project_and_fs_to_sql_for_views, PATH_FOR_MODELS,
};
use crate::schema_name::DEFAULT_SCHEMA_PREFIX;
use crate::sql_inference_translator::{map_test_to_sql_inference, map_tests_to_column_tests};
use futures::AsyncReadExt;
use prost::bytes::Bytes;
use quary_proto::{ColumnTest, Edge, File, Project, Test};
use sqlinference::dialect::Dialect;
use sqlinference::infer_tests::{infer_tests, InferenceReason};
use sqlinference::inference::{figure_out_skippable_tests, TestRunnerAction};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

/// infer_tests_internal returns a pointer of column to tests to put in a project file
pub fn infer_tests_internal(
    dialect: &Dialect,
    model_map: HashMap<String, String>,
    modelling_prefix: &str,
    model_of_interest: &str,
    tests: HashMap<String, Test>,
) -> Result<HashMap<String, Vec<ColumnTest>>, String> {
    let model = model_map.get(model_of_interest).ok_or(format!(
        "Model of interest {} not found in model map",
        model_of_interest
    ))?;
    let test_map = tests
        .iter()
        .filter_map(|(name, test)| {
            map_test_to_sql_inference(modelling_prefix, test.clone())
                .map(|mapped_test| (mapped_test, name.clone()))
        })
        .collect::<HashMap<_, _>>();
    let tests = test_map.keys().cloned().collect::<HashSet<_>>();

    let model_path = format!("{}.{}", modelling_prefix, model_of_interest);
    let tests = infer_tests(dialect, model_path.as_str(), model.as_str(), &tests)?;

    let out = map_tests_to_column_tests(modelling_prefix, tests.into_keys().collect())?;
    let out = out
        .get(model_of_interest)
        .ok_or(format!(
            "Model {} not found in column test map {:?}",
            model_of_interest, out
        ))?
        .clone();

    Ok(out)
}

pub async fn infer_skippable_tests_internal(
    dialect: &Dialect,
    project: &Project,
    file_system: &impl FileSystem,
    project_root: &str,
) -> Result<HashMap<String, Inference>, String> {
    let test_map = project
        .tests
        .iter()
        .filter_map(|(name, test)| {
            map_test_to_sql_inference(DEFAULT_SCHEMA_PREFIX, test.clone())
                .map(|mapped_test| (name.clone(), mapped_test))
        })
        .collect::<HashMap<_, _>>();
    let reverse_test_map = test_map
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect::<HashMap<_, _>>();

    let model_map = name_to_raw_model_map_internal(project, file_system, project_root).await?;

    let inferred_tests = figure_out_skippable_tests(
        dialect,
        &test_map.values().cloned().collect::<HashSet<_>>(),
        &model_map
            .iter()
            .map(|(k, v)| (format!("{}.{}", DEFAULT_SCHEMA_PREFIX, k), v.clone()))
            .collect::<HashMap<_, _>>(),
    );

    let out_tests = project
        .tests
        .iter()
        .map(|(test_name, test)| {
            if let Some(mapped_test) = test_map.get(test_name) {
                let inferred_test_action = inferred_tests.get(mapped_test).ok_or(format!(
                    "Could not find inferred test for {:?}",
                    mapped_test
                ))?;
                match inferred_test_action {
                    TestRunnerAction::Run => Ok((
                        test_name.to_string(),
                        Inference {
                            test: test.clone(),
                            action: InferenceTestRunnerAction::Run,
                        },
                    )),
                    TestRunnerAction::Skip(InferenceReason::UnderlyingTest(test_causing_skip)) => {
                        let test_causing_skip = reverse_test_map.get(test_causing_skip).ok_or(
                            format!("Could not find source test for {:?}", test_causing_skip),
                        )?;
                        Ok((
                            test_name.to_string(),
                            Inference {
                                test: test.clone(),
                                action: InferenceTestRunnerAction::SkipBecauseInferredFromTest(
                                    test_causing_skip.clone(),
                                ),
                            },
                        ))
                    }
                    TestRunnerAction::Skip(InferenceReason::CountStar) => Ok((
                        test_name.to_string(),
                        Inference {
                            test: test.clone(),
                            action: InferenceTestRunnerAction::SkipBecauseCountStar,
                        },
                    )),
                    TestRunnerAction::Skip(InferenceReason::UnderlyingTestWithOperation(
                                               test_causing_skip,
                                               operation,
                                           )) => {
                        let test_causing_skip = reverse_test_map.get(test_causing_skip).ok_or(
                            format!("Could not find source test for {:?}", test_causing_skip),
                        )?;
                        Ok((
                            test_name.to_string(),
                            Inference {
                                test: test.clone(),
                                action: InferenceTestRunnerAction::SkipBecauseInferredFromTestThroughOperation {
                                    test: test_causing_skip.to_string(),
                                    // TODO Probably could clean up over time
                                    operation: format!(
                                        "{:?}",
                                        operation.0
                                    ),
                                },
                            },
                        ))
                    }
                }
            } else {
                Ok((
                    test_name.to_string(),
                    Inference {
                        test: test.clone(),
                        action: InferenceTestRunnerAction::Run,
                    },
                ))
            }
        })
        .collect::<Result<Vec<(String, Inference)>, String>>()?;
    let out_tests = out_tests
        .into_iter()
        .collect::<HashMap<String, Inference>>();
    Ok(out_tests)
}

use futures::future::try_join_all;

pub async fn name_to_raw_model_map_internal(
    project: &Project,
    file_system: &impl FileSystem,
    project_root: &str,
) -> Result<HashMap<String, String>, String> {
    let futures = project.models.iter().map(|(name, model)| async move {
        let mut file = file_system
            .read_file({
                let mut path = PathBuf::from(project_root);
                path.push(model.file_path.as_str());
                path.to_str()
                    .ok_or(
                        "Could not convert path to string. This is a bug. Please report it."
                            .to_string(),
                    )?
                    .to_string()
                    .as_str()
            })
            .await
            .map_err(|_| "Failed to read file. This is a bug. Please report it.".to_string())?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| "failed to read string")?;

        Ok::<(_, _), String>((name.to_string(), contents))
    });

    let results = try_join_all(futures)
        .await
        .map_err(|e| format!("Error reading models: {}", e))?;

    Ok(results.into_iter().collect())
}

pub async fn render_schema_internal(
    database: &impl DatabaseQueryGenerator,
    project: Option<Project>,
    file_system: Option<quary_proto::FileSystem>,
) -> Result<String, String> {
    let schema = project_and_fs_to_sql_for_views(
        &project.ok_or("No project provided")?,
        &file_system.ok_or("No file system provided")?,
        database,
        false,
        true,
    )
    .await?;

    let schema = schema
        .iter()
        .flat_map(|(_, s)| s.clone())
        .collect::<Vec<String>>()
        .join("");

    Ok(schema)
}

pub async fn return_full_sql_for_new_model(
    file_system: quary_proto::FileSystem,
    project_root: &str,
    database: &impl DatabaseQueryGenerator,
    model_sql: &str,
    model_name: &str,
) -> Result<(Vec<Edge>, BTreeSet<String>), String> {
    let mut file_system = file_system;
    // TODO This will be complicated to do properly when needing to put it in the right place
    // TODO This can be moved out of here and should be done more cleanly
    // Add new model to the file system
    let mut path = PathBuf::from("/");
    path.push(PATH_FOR_MODELS);
    path.push(&format!("{}.sql", &model_name));
    let fp = path.to_str().ok_or("Failed to convert path to string")?;
    file_system.files.insert(
        fp.to_string(),
        File {
            name: fp.to_string(),
            contents: Bytes::from(model_sql.to_string()),
        },
    );

    // Parse the project
    // TODO Don't think this would work properly with sources as they won't be qualified
    let project = crate::project::parse_project(&file_system, database, project_root).await?;
    let (_, (nodes, edges)) =
        project_and_fs_to_query_sql(database, &project, &file_system, model_name, None).await?;

    let out_edges = edges
        .into_iter()
        .map(|(from, to)| Edge { from, to })
        .collect();

    Ok((out_edges, nodes))
}
