use crate::project_file::{
    STANDARD_TEST_TYPE_ACCEPTED_VALUES, STANDARD_TEST_TYPE_GREATER_THAN,
    STANDARD_TEST_TYPE_GREATER_THAN_OR_EQUAL, STANDARD_TEST_TYPE_LESS_THAN,
    STANDARD_TEST_TYPE_LESS_THAN_OR_EQUAL, STANDARD_TEST_TYPE_RELATIONSHIP,
    STANDARD_TEST_TYPE_SQL_NOT_NULL, STANDARD_TEST_TYPE_SQL_UNIQUE,
};
use quary_proto::test::TestType;
use quary_proto::ColumnTest;
use sqlinference::test::{
    AcceptedValuesTest, ComparisonTest, RelationshipTest, StandardTest, Test,
};
use std::collections::HashMap;

/// map_test_to_sql_inference maps the given test to a sql inference test
/// ```
/// let test = quary_proto::Test {
///    test_type: Some(quary_proto::test::TestType::NotNull(quary_proto::TestNotNull {
///    file_path: "models/test.yaml".to_string(),
///    model: "model".to_string(),
///    path: "schema_name_chosen.model".to_string(),
///    column: "column".to_string(),
///  })),
/// };
///
/// let want = Some(sqlinference::test::Test::NotNull(sqlinference::test::StandardTest {
///   column: "column".to_string(),
///   path: "schema_name_chosen.model".to_string(),
/// }));
///
/// let out = quary_core::sql_inference_translator::map_test_to_sql_inference("schema_name_chosen", test);
///
/// assert_eq!(want, out);
///
pub fn map_test_to_sql_inference(modelling_prefix: &str, test: quary_proto::Test) -> Option<Test> {
    test.test_type.and_then(|test| match test {
        TestType::NotNull(test) => Some(Test::NotNull(StandardTest {
            path: prefix_model(&test.model, modelling_prefix),
            column: test.column.to_string(),
        })),
        TestType::Unique(test) => Some(Test::Unique(StandardTest {
            path: prefix_model(&test.model, modelling_prefix),
            column: test.column.to_string(),
        })),
        TestType::AcceptedValues(test) => Some(Test::AcceptedValues(AcceptedValuesTest {
            path: prefix_model(&test.model, modelling_prefix),
            column: test.column.to_string(),
            values: test.accepted_values,
        })),
        TestType::GreaterThanOrEqual(test) => Some(Test::GreaterThanOrEqual(ComparisonTest {
            path: prefix_model(&test.model, modelling_prefix),
            column: test.column.to_string(),
            value: test.value.to_string(),
        })),
        TestType::LessThanOrEqual(test) => Some(Test::LessThanOrEqual(ComparisonTest {
            path: prefix_model(&test.model, modelling_prefix),
            column: test.column.to_string(),
            value: test.value.to_string(),
        })),
        TestType::LessThan(test) => Some(Test::LessThan(ComparisonTest {
            path: prefix_model(&test.model, modelling_prefix),
            column: test.column.to_string(),
            value: test.value.to_string(),
        })),
        TestType::GreaterThan(test) => Some(Test::GreaterThan(ComparisonTest {
            path: prefix_model(&test.model, modelling_prefix),
            column: test.column.to_string(),
            value: test.value.to_string(),
        })),
        TestType::Relationship(test) => Some(Test::Relationship(RelationshipTest {
            path: prefix_model(&test.source_model, modelling_prefix),
            column: test.source_column.to_string(),
            target_reference: prefix_model(&test.target_model, modelling_prefix),
            target_column: test.target_column.to_string(),
        })),
        _ => None,
    })
}

/// prefix_model prefixes the model with the given prefix, e.g. schema_name_chosen
/// ```
/// let prefixed = quary_core::sql_inference_translator::prefix_model("model", "prefix");
///
/// assert_eq!(prefixed, "prefix.model".to_string());
/// ```
pub fn prefix_model(model: &str, prefix: &str) -> String {
    format!("{}.{}", prefix, model)
}

/// unprefix_model unprefixed the model with the given prefix, e.g. schema_name_chosen. If it cannot be
/// unprefixed, an error is returned.
/// ```
/// let valid = quary_core::sql_inference_translator::unprefix_model("prefix.model", "prefix");
/// assert_eq!(valid,Ok("model".to_string()));
///
/// let invalid = quary_core::sql_inference_translator::unprefix_model("prefix.model", "prefix2");
/// assert_eq!(invalid, Err("Model prefix.model does not start with prefix prefix2".to_string()));
/// ```
pub fn unprefix_model(model: &str, prefix: &str) -> Result<String, String> {
    if model.starts_with(prefix) {
        if let Some(model) = model.get(prefix.len() + 1..) {
            Ok(model.to_string())
        } else {
            Err(format!(
                "Invalid getting model {} from prefix {}",
                model, prefix
            ))
        }
    } else {
        Err(format!(
            "Model {} does not start with prefix {}",
            model, prefix
        ))
    }
}

/// map_tests_to_column_tests returns a map of model name (not reference) to column to column test for
/// the given tests
pub fn map_tests_to_column_tests(
    modelling_prefix: &str,
    tests: Vec<Test>,
) -> Result<HashMap<String, HashMap<String, Vec<ColumnTest>>>, String> {
    let outs: Vec<(String, String, ColumnTest)> = tests
        .into_iter()
        .map(|test| map_test_to_column_test(modelling_prefix, test))
        .collect::<Result<Vec<_>, String>>()?;

    let mut map: HashMap<String, HashMap<String, Vec<ColumnTest>>> = HashMap::new();
    for (model, column, test) in outs {
        let model_map = map.entry(model).or_default();
        let column_tests = model_map.entry(column).or_default();
        column_tests.push(test);
    }

    Ok(map)
}

// map_test_to_column_test maps the given test to a column test, returning the model name, column name and column test.
pub(crate) fn map_test_to_column_test(
    modelling_prefix: &str,
    test: Test,
) -> Result<(String, String, ColumnTest), String> {
    match test {
        Test::NotNull(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_SQL_NOT_NULL.to_string(),
                    info: Default::default(),
                },
            ))
        }
        Test::Unique(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_SQL_UNIQUE.to_string(),
                    info: Default::default(),
                },
            ))
        }
        Test::GreaterThanOrEqual(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_GREATER_THAN_OR_EQUAL.to_string(),
                    info: HashMap::from([("value".to_string(), test.value.to_string())]),
                },
            ))
        }
        Test::LessThanOrEqual(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_LESS_THAN_OR_EQUAL.to_string(),
                    info: HashMap::from([("value".to_string(), test.value.to_string())]),
                },
            ))
        }
        Test::GreaterThan(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_GREATER_THAN.to_string(),
                    info: HashMap::from([("value".to_string(), test.value.to_string())]),
                },
            ))
        }
        Test::LessThan(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_LESS_THAN.to_string(),
                    info: HashMap::from([("value".to_string(), test.value.to_string())]),
                },
            ))
        }
        Test::Relationship(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            let target_model = unprefix_model(&test.target_reference, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_RELATIONSHIP.to_string(),
                    info: HashMap::from([
                        ("model".to_string(), target_model),
                        ("column".to_string(), test.target_column.to_string()),
                    ]),
                },
            ))
        }
        Test::AcceptedValues(test) => {
            let model = unprefix_model(&test.path, modelling_prefix)?;
            Ok((
                model,
                test.column.to_string(),
                ColumnTest {
                    r#type: STANDARD_TEST_TYPE_ACCEPTED_VALUES.to_string(),
                    info: HashMap::from([("values".to_string(), test.values.join(","))]),
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quary_proto::{
        TestAcceptedValues, TestGreaterThan, TestGreaterThanOrEqual, TestLessThan,
        TestLessThanOrEqual, TestNotNull, TestRelationship, TestUnique,
    };

    #[test]
    fn map_test_to_column_test_test() {
        let modelling_prefix = "schema_name_chosen";

        let tests = [
            Test::NotNull(StandardTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
            }),
            Test::Unique(StandardTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column_1".to_string(),
            }),
            Test::GreaterThanOrEqual(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::LessThanOrEqual(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::GreaterThan(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::LessThan(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::Relationship(RelationshipTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                target_reference: "schema_name_chosen.target_model".to_string(),
                target_column: "target_column".to_string(),
            }),
            Test::AcceptedValues(AcceptedValuesTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                values: vec!["1".to_string(), "2".to_string()],
            }),
        ];
        let want = HashMap::from([(
            "model".to_string(),
            HashMap::from([
                (
                    "column".to_string(),
                    Vec::from([
                        ColumnTest {
                            r#type: STANDARD_TEST_TYPE_SQL_NOT_NULL.to_string(),
                            info: Default::default(),
                        },
                        ColumnTest {
                            r#type: STANDARD_TEST_TYPE_GREATER_THAN_OR_EQUAL.to_string(),
                            info: HashMap::from([("value".to_string(), "1".to_string())]),
                        },
                        ColumnTest {
                            r#type: STANDARD_TEST_TYPE_LESS_THAN_OR_EQUAL.to_string(),
                            info: HashMap::from([("value".to_string(), "1".to_string())]),
                        },
                        ColumnTest {
                            r#type: STANDARD_TEST_TYPE_GREATER_THAN.to_string(),
                            info: HashMap::from([("value".to_string(), "1".to_string())]),
                        },
                        ColumnTest {
                            r#type: STANDARD_TEST_TYPE_LESS_THAN.to_string(),
                            info: HashMap::from([("value".to_string(), "1".to_string())]),
                        },
                        ColumnTest {
                            r#type: STANDARD_TEST_TYPE_RELATIONSHIP.to_string(),
                            info: HashMap::from([
                                ("model".to_string(), "target_model".to_string()),
                                ("column".to_string(), "target_column".to_string()),
                            ]),
                        },
                        ColumnTest {
                            r#type: STANDARD_TEST_TYPE_ACCEPTED_VALUES.to_string(),
                            info: HashMap::from([("values".to_string(), "1,2".to_string())]),
                        },
                    ]),
                ),
                (
                    "column_1".to_string(),
                    Vec::from([ColumnTest {
                        r#type: STANDARD_TEST_TYPE_SQL_UNIQUE.to_string(),
                        info: Default::default(),
                    }]),
                ),
            ]),
        )]);

        let outs = map_tests_to_column_tests(modelling_prefix, tests.to_vec()).unwrap();

        assert_eq!(outs, want,);
    }

    #[test]
    fn map_test_to_sql_inference_test() {
        let input = [
            quary_proto::Test {
                test_type: Some(TestType::NotNull(TestNotNull {
                    file_path: "".to_string(),
                    model: "model".to_string(),
                    path: "schema_name_chosen.model".to_string(),
                    column: "column".to_string(),
                })),
            },
            quary_proto::Test {
                test_type: Some(TestType::Unique(TestUnique {
                    file_path: "".to_string(),
                    model: "model".to_string(),
                    path: "schema_name_chosen.model".to_string(),
                    column: "column".to_string(),
                })),
            },
            quary_proto::Test {
                test_type: Some(TestType::AcceptedValues(TestAcceptedValues {
                    file_path: "".to_string(),
                    model: "model".to_string(),
                    path: "schema_name_chosen.model".to_string(),
                    column: "column".to_string(),
                    accepted_values: vec!["1".to_string(), "2".to_string()],
                })),
            },
            quary_proto::Test {
                test_type: Some(TestType::GreaterThanOrEqual(TestGreaterThanOrEqual {
                    file_path: "".to_string(),
                    model: "model".to_string(),
                    path: "schema_name_chosen.model".to_string(),
                    column: "column".to_string(),
                    value: "1".to_string(),
                })),
            },
            quary_proto::Test {
                test_type: Some(TestType::LessThanOrEqual(TestLessThanOrEqual {
                    file_path: "".to_string(),
                    model: "model".to_string(),
                    path: "schema_name_chosen.model".to_string(),
                    column: "column".to_string(),
                    value: "1".to_string(),
                })),
            },
            quary_proto::Test {
                test_type: Some(TestType::LessThan(TestLessThan {
                    file_path: "".to_string(),
                    model: "model".to_string(),
                    path: "schema_name_chosen.model".to_string(),
                    column: "column".to_string(),
                    value: "1".to_string(),
                })),
            },
            quary_proto::Test {
                test_type: Some(TestType::GreaterThan(TestGreaterThan {
                    file_path: "".to_string(),
                    model: "model".to_string(),
                    path: "schema_name_chosen.model".to_string(),
                    column: "column".to_string(),
                    value: "1".to_string(),
                })),
            },
            quary_proto::Test {
                test_type: Some(TestType::Relationship(TestRelationship {
                    file_path: "".to_string(),
                    source_model: "model".to_string(),
                    source_path: "schema_name_chosen.model".to_string(),
                    source_column: "column".to_string(),
                    target_model: "target_model".to_string(),
                    target_path: "schema_name_chosen.target_model".to_string(),
                    target_column: "target_column".to_string(),
                })),
            },
        ];

        let want = [
            Test::NotNull(StandardTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
            }),
            Test::Unique(StandardTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
            }),
            Test::AcceptedValues(AcceptedValuesTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                values: vec!["1".to_string(), "2".to_string()],
            }),
            Test::GreaterThanOrEqual(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::LessThanOrEqual(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::LessThan(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::GreaterThan(ComparisonTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                value: "1".to_string(),
            }),
            Test::Relationship(RelationshipTest {
                path: "schema_name_chosen.model".to_string(),
                column: "column".to_string(),
                target_reference: "schema_name_chosen.target_model".to_string(),
                target_column: "target_column".to_string(),
            }),
        ];

        let outs = input
            .iter()
            .map(|test| {
                let test = map_test_to_sql_inference("schema_name_chosen", test.clone());
                test.unwrap()
            })
            .collect::<Vec<_>>();

        assert_eq!(want.to_vec(), outs);
    }
}
