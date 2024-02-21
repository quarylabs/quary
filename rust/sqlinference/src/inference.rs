use crate::dialect::Dialect;
use crate::infer_tests::{infer_tests, InferenceReason};
use crate::test::Test;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TestRunnerAction {
    Run,
    // Test is the reason for skipping
    Skip(InferenceReason),
}

/// figure_out_skippable_tests returns a map of the tests that can be skipped as an array with an enum.
/// The enum is either Run or Skip.
///
/// The sql_map is a map of the model to the sql code that is generated.
pub fn figure_out_skippable_tests(
    dialect: &Dialect,
    tests: &HashSet<Test>,
    sql_map: &HashMap<String, String>,
) -> HashMap<Test, TestRunnerAction> {
    let all_inferred_tests = sql_map
        .iter()
        .flat_map(|(path, sql)| {
            if let Ok(tests) = infer_tests(dialect, path, sql, tests) {
                tests.into_iter().collect::<Vec<_>>()
            } else {
                Vec::<(Test, InferenceReason)>::new()
            }
        })
        .collect::<HashMap<Test, InferenceReason>>();

    tests
        .iter()
        .map(|test| {
            if let Some(reason) = all_inferred_tests.get(test) {
                (test.clone(), TestRunnerAction::Skip(reason.clone()))
            } else {
                (test.clone(), TestRunnerAction::Run)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::inference::TestRunnerAction::{Run, Skip};
    use crate::test::{AcceptedValuesTest, ComparisonTest, StandardTest};

    #[test]
    fn test_figure_out_skippable_steps() {
        struct TestStructure {
            name: &'static str,
            sql: HashMap<String, String>,
            input_tests: Vec<Test>,
            want: HashMap<Test, TestRunnerAction>,
        }

        let tests = vec![TestStructure {
            name: "simple all test types can be inferred",
            sql: HashMap::from([(
                "q.model_child".to_string(),
                "SELECT a FROM q.model_parent".to_string(),
            )]),
            input_tests: vec![
                Test::NotNull(StandardTest {
                    path: "q.model_parent".to_string(),
                    column: "a".to_string(),
                }),
                Test::NotNull(StandardTest {
                    path: "q.model_child".to_string(),
                    column: "a".to_string(),
                }),
                Test::AcceptedValues(AcceptedValuesTest {
                    path: "q.model_parent".to_string(),
                    column: "a".to_string(),
                    values: ["a", "b", "c"].iter().map(|s| s.to_string()).collect(),
                }),
                Test::AcceptedValues(AcceptedValuesTest {
                    path: "q.model_child".to_string(),
                    column: "a".to_string(),
                    values: ["a", "b", "c"].iter().map(|s| s.to_string()).collect(),
                }),
                Test::Unique(StandardTest {
                    path: "q.model_parent".to_string(),
                    column: "a".to_string(),
                }),
                Test::Unique(StandardTest {
                    path: "q.model_child".to_string(),
                    column: "a".to_string(),
                }),
            ],
            want: HashMap::from([
                (
                    Test::NotNull(StandardTest {
                        path: "q.model_parent".to_string(),
                        column: "a".to_string(),
                    }),
                    Run,
                ),
                (
                    Test::NotNull(StandardTest {
                        path: "q.model_child".to_string(),
                        column: "a".to_string(),
                    }),
                    Skip(InferenceReason::UnderlyingTest(Test::NotNull(
                        StandardTest {
                            path: "q.model_parent".to_string(),
                            column: "a".to_string(),
                        },
                    ))),
                ),
                (
                    Test::AcceptedValues(AcceptedValuesTest {
                        path: "q.model_parent".to_string(),
                        column: "a".to_string(),
                        values: ["a", "b", "c"].iter().map(|s| s.to_string()).collect(),
                    }),
                    Run,
                ),
                (
                    Test::AcceptedValues(AcceptedValuesTest {
                        path: "q.model_child".to_string(),
                        column: "a".to_string(),
                        values: ["a", "b", "c"].iter().map(|s| s.to_string()).collect(),
                    }),
                    Skip(InferenceReason::UnderlyingTest(Test::AcceptedValues(
                        AcceptedValuesTest {
                            path: "q.model_parent".to_string(),
                            column: "a".to_string(),
                            values: ["a", "b", "c"].iter().map(|s| s.to_string()).collect(),
                        },
                    ))),
                ),
                (
                    Test::Unique(StandardTest {
                        path: "q.model_parent".to_string(),
                        column: "a".to_string(),
                    }),
                    Run,
                ),
                (
                    Test::Unique(StandardTest {
                        path: "q.model_child".to_string(),
                        column: "a".to_string(),
                    }),
                    Skip(InferenceReason::UnderlyingTest(Test::Unique(
                        StandardTest {
                            path: "q.model_parent".to_string(),
                            column: "a".to_string(),
                        },
                    ))),
                ),
            ]),
        }];

        for test in tests {
            let actual = figure_out_skippable_tests(
                &Dialect::SQLite,
                &test.input_tests.into_iter().collect(),
                &test.sql,
            );

            assert_eq!(actual.len(), test.want.len(), "{}", test.name);
            assert_eq!(actual, test.want, "{}", test.name);
        }
    }

    #[test]
    fn test_figure_out_skippable_steps_count_star() {
        struct TestStructure {
            name: &'static str,
            sql: HashMap<String, String>,
            input_tests: Vec<Test>,
            want: HashMap<Test, TestRunnerAction>,
        }

        let tests = vec![
            TestStructure {
                name: "simple select count *",
                sql: HashMap::from([(
                    "q.model_child".to_string(),
                    "SELECT count(*) AS a FROM q.model_parent".to_string(),
                )]),
                input_tests: vec![Test::GreaterThanOrEqual(ComparisonTest {
                    path: "q.model_child".to_string(),
                    column: "a".to_string(),
                    value: "0".to_string(),
                })],
                want: HashMap::from([(
                    Test::GreaterThanOrEqual(ComparisonTest {
                        path: "q.model_child".to_string(),
                        column: "a".to_string(),
                        value: "0".to_string(),
                    }),
                    Skip(InferenceReason::CountStar),
                )]),
            },
            TestStructure {
                name: "simple select count * in with ",
                sql: HashMap::from([(
                    "q.model_child".to_string(),
                    "WITH test AS (SELECT count(*) AS counter FROM q.model_parent) SELECT counter from test"
                        .to_string(),
                )]),
                input_tests: vec![
                    Test::GreaterThanOrEqual(ComparisonTest {
                        path: "q.model_child".to_string(),
                        column: "counter".to_string(),
                        value: "0".to_string(),
                    }),
                ],
                want: HashMap::from([(
                    Test::GreaterThanOrEqual(ComparisonTest {
                        path: "q.model_child".to_string(),
                        column: "counter".to_string(),
                        value: "0".to_string(),
                    }),
                    Skip(InferenceReason::CountStar),
                )]),
            },
        ];

        for test in tests {
            let actual = figure_out_skippable_tests(
                &Dialect::SQLite,
                &test.input_tests.into_iter().collect(),
                &test.sql,
            );

            assert_eq!(actual.len(), test.want.len(), "{}", test.name);
            assert_eq!(actual, test.want, "{}", test.name);
        }
    }

    #[test]
    fn test_figure_out_skippable_steps_init_example() {
        struct TestStructure {
            name: &'static str,
            sql: HashMap<String, String>,
            input_tests: Vec<Test>,
            want: HashMap<Test, TestRunnerAction>,
        }

        let tests: Vec<TestStructure> = vec![TestStructure {
            name: "stg_shifts",
            sql: HashMap::from([(
                "q.shifts_by_month".to_string(),
                "SELECT
        employee_id,
        strftime('%Y-%m', shift_date) AS shift_month,
        COUNT(*)                     AS total_shifts
    FROM q.stg_shifts
    GROUP BY employee_id, shift_month"
                    .to_string(),
            )]),
            input_tests: vec![
                Test::NotNull(StandardTest {
                    column: "employee_id".to_string(),
                    path: "q.stg_shifts".to_string(),
                }),
                Test::NotNull(StandardTest {
                    column: "employee_id".to_string(),
                    path: "q.shifts_by_month".to_string(),
                }),
            ],
            want: HashMap::from([
                (
                    Test::NotNull(StandardTest {
                        column: "employee_id".to_string(),
                        path: "q.stg_shifts".to_string(),
                    }),
                    Run,
                ),
                (
                    Test::NotNull(StandardTest {
                        column: "employee_id".to_string(),
                        path: "q.shifts_by_month".to_string(),
                    }),
                    Skip(InferenceReason::UnderlyingTest(Test::NotNull(
                        StandardTest {
                            column: "employee_id".to_string(),
                            path: "q.stg_shifts".to_string(),
                        },
                    ))),
                ),
            ]),
        }];

        for test in tests {
            println!("Running test: {}", test.name);

            let actual = figure_out_skippable_tests(
                &Dialect::SQLite,
                &test.input_tests.into_iter().collect::<HashSet<_>>(),
                &test.sql,
            );

            assert_eq!(actual, test.want);
        }
    }
}
