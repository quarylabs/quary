use crate::databases::DatabaseQueryGenerator;
use crate::file_system::FileSystem;
use crate::inference::{Inference, InferenceTestRunnerAction};
use crate::project_tests::{return_model_tests_sql, return_tests_sql};
use crate::rpc_proto_defined_functions::infer_skippable_tests_internal;
use quary_proto::test_result::TestResult::{Failed, Passed};
use quary_proto::{failed, passed, InferredChain, InferredChainWithOperation, TestResult};
use quary_proto::{Project, TestResults, TestRunner};
use sqlinference::dialect::Dialect;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;

pub type RunStatementFunc =
    Box<dyn Fn(&str) -> Pin<Box<dyn Future<Output = Result<bool, String>>>>>;

fn recursive_search_for_test(
    whether_to_skip: &HashMap<String, Inference>,
    test_of_interest: &str,
) -> Result<Vec<String>, String> {
    let test = whether_to_skip
        .get(test_of_interest)
        .map_or_else(|| Err("Test not found".to_string()), Ok)?;
    match &test.action {
        InferenceTestRunnerAction::SkipBecauseInferredFromTest(test_causing_skip) => {
            let mut tests = vec![test_causing_skip.clone()];
            let mut further_tests = recursive_search_for_test(whether_to_skip, test_causing_skip)?;
            tests.append(&mut further_tests);
            Ok(tests)
        }
        InferenceTestRunnerAction::SkipBecauseInferredFromTestThroughOperation { test, .. } => {
            let mut tests = vec![test.clone()];
            let mut further_tests = recursive_search_for_test(whether_to_skip, test)?;
            tests.append(&mut further_tests);
            Ok(tests)
        }
        _ => Ok(vec![]),
    }
}

/// Runs tests on all models
/// limit applies a SQL limit to the end of each test. By adding a limit to the end of each test, we can run them more
/// quickly and not have to wait for an number of results, greater than the limit to be returned. This is useful for
/// CI environments where we want to run tests quickly and not wait for a large number of results to be returned.
pub async fn run_tests_internal(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    project: &Project,
    project_root: &str,
    dialect: &Dialect,
    test_runner: TestRunner,
    run_statement: RunStatementFunc,
    whether_to_include_model_to_source: bool,
    limit: Option<usize>,
) -> Result<TestResults, String> {
    async fn run_test_all(
        tests_name_to_sql: BTreeMap<String, String>,
        run_statement: RunStatementFunc,
    ) -> Result<TestResults, String> {
        let mut results = Vec::new();
        for (test_name, sql) in tests_name_to_sql {
            let test_result = run_statement(sql.as_str()).await;
            match test_result {
                Ok(test_result) => {
                    if test_result {
                        results.push(TestResult {
                            test_name,
                            query: sql.clone(),
                            test_result: Some(Passed(quary_proto::Passed {
                                reason: Some(passed::Reason::Ran(Default::default())),
                            })),
                        });
                    } else {
                        results.push(TestResult {
                            test_name,
                            query: sql.clone(),
                            test_result: Some(Failed(quary_proto::Failed {
                                reason: Some(failed::Reason::Ran(Default::default())),
                            })),
                        });
                    }
                }
                Err(e) => return Err(format!("Failed to run test {} with error {}", test_name, e)),
            }
        }
        Ok(TestResults { results })
    }

    async fn run_test_skip(
        tests_name_to_sql: BTreeMap<String, String>,
        whether_to_skip: HashMap<String, Inference>,
        run_statement: RunStatementFunc,
    ) -> Result<TestResults, String> {
        let tests_to_run = tests_name_to_sql
            .keys()
            .filter_map(|test_name| match whether_to_skip.get(test_name) {
                Some(inference) => match &inference.action {
                    InferenceTestRunnerAction::Run => Some(Ok(test_name)),
                    _ => None,
                },
                None => Some(Err(format!("Test {} not found", test_name))),
            })
            .collect::<Result<HashSet<&String>, String>>()?;
        let tests_to_just_mark_right = tests_name_to_sql
            .keys()
            .filter_map(|test_name| match whether_to_skip.get(test_name) {
                Some(inference) => match &inference.action {
                    InferenceTestRunnerAction::SkipBecauseCountStar => Some(Ok(test_name)),
                    _ => None,
                },
                None => Some(Err(format!("Test {} not found", test_name))),
            })
            .collect::<Result<HashSet<&String>, String>>()?;
        let tests_to_skip_for_inference = tests_name_to_sql
            .keys()
            .filter_map(|test_name| match whether_to_skip.get(test_name) {
                Some(inference) => match &inference.action {
                    InferenceTestRunnerAction::SkipBecauseInferredFromTest(_) => {
                        Some(Ok(test_name))
                    }
                    _ => None,
                },
                None => Some(Err(format!("Test {} not found", test_name))),
            })
            // Test to source test
            .collect::<Result<HashSet<&String>, String>>()?;
        let tests_to_skip_for_inference_with_operation = tests_name_to_sql
            .keys()
            .filter_map(|test_name| match whether_to_skip.get(test_name) {
                Some(inference) => match &inference.action {
                    InferenceTestRunnerAction::SkipBecauseInferredFromTestThroughOperation {
                        operation,
                        ..
                    } => Some(Ok((test_name, operation))),
                    _ => None,
                },
                None => Some(Err(format!("Test {} not found", test_name,))),
            })
            // Test to source test
            .collect::<Result<HashMap<&String, &String>, String>>()?;

        let mut results = HashMap::<&String, TestResult>::new();
        for test_name in tests_to_run {
            let sql = tests_name_to_sql.get(test_name).ok_or(
                format!(
                    "Test {} not found, tests_name_to_sql keys {:?}",
                    test_name,
                    tests_name_to_sql.keys()
                )
                .to_string(),
            )?;
            let test_result = run_statement(sql.as_str()).await?;
            results.insert(
                test_name,
                TestResult {
                    test_name: test_name.clone(),
                    query: sql.clone(),
                    test_result: Some(if test_result {
                        Passed(quary_proto::Passed {
                            reason: Some(passed::Reason::Ran(Default::default())),
                        })
                    } else {
                        Failed(quary_proto::Failed {
                            reason: Some(failed::Reason::Ran(Default::default())),
                        })
                    }),
                },
            );
        }

        // Doing the inferences that are static
        let tests_to_mark_just_right_results: HashMap<&String, TestResult> =
            tests_to_just_mark_right
                .into_iter()
                .map(|test_name| {
                    let sql = tests_name_to_sql
                        .get(test_name)
                        .ok_or(format!("Test {} not found", test_name))?;
                    Ok((
                        test_name,
                        TestResult {
                            test_name: test_name.clone(),
                            query: sql.clone(),
                            test_result: Some(Passed(quary_proto::Passed {
                                // TODO Might need to make this more generic in the future.
                                reason: Some(passed::Reason::InferredFromLogic(
                                    "inferred from count(*)".to_string(),
                                )),
                            })),
                        },
                    ))
                })
                .collect::<Result<HashMap<&String, TestResult>, String>>()?;
        results.extend(tests_to_mark_just_right_results);

        let mut skip_results = HashMap::<&String, TestResult>::new();
        for test_name in tests_to_skip_for_inference {
            let sql = tests_name_to_sql.get(test_name).ok_or(
                format!(
                    "Test {} not found, tests_name_to_sql keys {:?}",
                    test_name,
                    tests_name_to_sql.keys()
                )
                .to_string(),
            )?;
            let test_sources = recursive_search_for_test(&whether_to_skip, test_name)?;
            let intermediary_test_source = &test_sources
                .last()
                .ok_or("Failed to find last test".to_string())?;
            let test_source = results
                .get(intermediary_test_source)
                .ok_or(format!(
                    "Failed to find test source, looking at the test {:?} which was skipped and has tests sources {:?}, more specifically the last one in ran results with keys {:?}", test_name, test_sources, results.keys()
                ))?;
            match test_source.test_result {
                Some(Passed(_)) => {
                    skip_results.insert(
                        test_name,
                        TestResult {
                            test_name: test_name.clone(),
                            query: sql.clone(),
                            test_result: Some(Passed(quary_proto::Passed {
                                reason: Some(passed::Reason::InferredFromTests(InferredChain {
                                    inferred_chain: test_sources,
                                })),
                            })),
                        },
                    );
                }
                Some(Failed(_)) => {
                    skip_results.insert(
                        test_name,
                        TestResult {
                            test_name: test_name.clone(),
                            query: sql.clone(),
                            test_result: Some(Failed(quary_proto::Failed {
                                reason: Some(failed::Reason::InferredFromTests(InferredChain {
                                    inferred_chain: test_sources,
                                })),
                            })),
                        },
                    );
                }
                _ => {
                    return Err(format!(
                        "Failed to find test source, looking at the test {:?} which was skipped and has tests sources {:?}, more specifically the last one in ran results with keys {:?}", test_name, test_sources, results.keys()
                    ))
                }
            }
        }

        for (test_name, operation) in tests_to_skip_for_inference_with_operation {
            let sql = tests_name_to_sql.get(test_name).ok_or(
                format!(
                    "Test {} not found, tests_name_to_sql keys {:?}",
                    test_name,
                    tests_name_to_sql.keys()
                )
                .to_string(),
            )?;
            let test_sources = recursive_search_for_test(&whether_to_skip, test_name)?;
            if test_sources.is_empty() {
                return Err(format!(
                    "Failed to find test source, looking at the test {:?} which was skipped and has tests sources {:?}, more specifically the last one in ran results with keys {:?}",
                    test_name, test_sources, results.keys()
                ));
            }
            let intermediary_test_source = &test_sources.last()
                .ok_or(format!(
                    "Failed to find test source, looking at the test {:?} which was skipped and has tests sources {:?}, more specifically the last one in ran results with keys {:?}", test_name, test_sources, results.keys()
                ))?;
            let test_source = results
                .get(intermediary_test_source)
                .ok_or(format!(
                    "Failed to find test source, looking at the test {:?} which was skipped and has tests sources {:?}, more specifically the last one in ran results with keys {:?}", test_name, test_sources, results.keys()
                ))?;
            match test_source.test_result {
                Some(Passed(_)) => {
                    skip_results.insert(
                        test_name,
                        TestResult {
                            test_name: test_name.clone(),
                            query: sql.clone(),
                            test_result: Some(Passed(quary_proto::Passed {
                                reason: Some(passed::Reason::InferredThroughTestsOperation(
                                    InferredChainWithOperation {
                                        inferred_chain: test_sources,
                                        operation: operation.clone(),
                                    },
                                )),
                            })),
                        },
                    );
                }
                Some(Failed(_)) => {
                    skip_results.insert(
                        test_name,
                        TestResult {
                            test_name: test_name.clone(),
                            query: sql.clone(),
                            test_result: Some(Failed(quary_proto::Failed {
                                reason: Some(failed::Reason::InferredThroughTestsOperation(
                                    InferredChainWithOperation {
                                        inferred_chain: test_sources,
                                        operation: operation.clone(),
                                    },
                                )),
                            })),
                        },
                    );
                }
                _ => return Err("test source is empty".to_string()),
            }
        }

        results.extend(skip_results);
        Ok(TestResults {
            results: results.into_values().collect(),
        })
    }

    match test_runner {
        TestRunner::Skip => {
            let tests = return_tests_sql(
                database,
                project,
                file_system,
                whether_to_include_model_to_source,
                limit,
            )?;
            let whether_to_skip =
                infer_skippable_tests_internal(dialect, project, file_system, project_root)?;
            run_test_skip(tests, whether_to_skip, run_statement).await
        }
        TestRunner::All => {
            let tests = return_tests_sql(
                database,
                project,
                file_system,
                whether_to_include_model_to_source,
                limit,
            )?;
            run_test_all(tests, run_statement).await
        }
        _ => Err("Invalid test runner".to_string()),
    }
}

/// Runs tests on a specific model
/// limit applies a SQL limit to the end of each test. By adding a limit to the end of each test, we can run them more
/// quickly and not have to wait for an number of results, greater than the limit to be returned. This is useful for
/// CI environments where we want to run tests quickly and not wait for a large number of results to be returned.
pub async fn run_model_tests_internal(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    project: &Project,
    run_statement: RunStatementFunc,
    whether_to_include_model_to_source: bool,
    limit: Option<usize>,
    model_name: &str,
) -> Result<TestResults, String> {
    async fn run_test_all(
        tests_name_to_sql: BTreeMap<String, String>,
        run_statement: RunStatementFunc,
    ) -> Result<TestResults, String> {
        let mut results = Vec::new();
        for (test_name, sql) in tests_name_to_sql {
            let test_result = run_statement(sql.as_str()).await;
            match test_result {
                Ok(test_result) => {
                    if test_result {
                        results.push(TestResult {
                            test_name,
                            query: sql.clone(),
                            test_result: Some(Passed(quary_proto::Passed {
                                reason: Some(passed::Reason::Ran(Default::default())),
                            })),
                        });
                    } else {
                        results.push(TestResult {
                            test_name,
                            query: sql.clone(),
                            test_result: Some(Failed(quary_proto::Failed {
                                reason: Some(failed::Reason::Ran(Default::default())),
                            })),
                        });
                    }
                }
                Err(e) => return Err(format!("Failed to run test {} with error {}", test_name, e)),
            }
        }
        Ok(TestResults { results })
    }
    let tests = return_model_tests_sql(
        database,
        project,
        file_system,
        whether_to_include_model_to_source,
        limit,
        model_name,
    )?;
    run_test_all(tests, run_statement).await
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_recursive_search_for_test_single() {
        let whether_to_skip = HashMap::from([
            (
                "a".to_string(),
                Inference {
                    test: Default::default(),
                    action: InferenceTestRunnerAction::SkipBecauseInferredFromTest("b".to_string()),
                },
            ),
            (
                "b".to_string(),
                Inference {
                    test: Default::default(),
                    action: InferenceTestRunnerAction::Run,
                },
            ),
        ]);
        let test_of_interest = "a";

        let expected = vec!["b".to_string()];
        let actual = recursive_search_for_test(&whether_to_skip, test_of_interest).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_recursive_search_for_test_multiple() {
        let whether_to_skip = HashMap::from([
            (
                "a".to_string(),
                Inference {
                    test: Default::default(),
                    action: InferenceTestRunnerAction::SkipBecauseInferredFromTest("b".to_string()),
                },
            ),
            (
                "b".to_string(),
                Inference {
                    test: Default::default(),
                    action: InferenceTestRunnerAction::SkipBecauseInferredFromTest("c".to_string()),
                },
            ),
            (
                "c".to_string(),
                Inference {
                    test: Default::default(),
                    action: InferenceTestRunnerAction::Run,
                },
            ),
        ]);
        let test_of_interest = "a";

        let expected = vec!["b".to_string(), "c".to_string()];
        let actual = recursive_search_for_test(&whether_to_skip, test_of_interest);

        assert_eq!(actual, Ok(expected));
    }
}
