use crate::sql_inference_translator::{map_test_to_column_test, map_test_to_sql_inference};
use crate::tests::ShortTestString;
use quary_proto::row_description::{Description, PresentWithInference};
use quary_proto::table::present_in_schema::{present_row, PresentRow};
use quary_proto::table::{PresentInSchema, TableType};
use quary_proto::{row_test, Row, RowDescription, RowTest, RowTestDetails, Table};
use sqlinference::test::Test;
use std::collections::{HashMap, HashSet};

pub fn map_to_description_table(
    modelling_prefix: &str,
    project_columns: Option<Vec<String>>,
    inferred_columns: Option<Vec<String>>,
    defined_description_map: Option<HashMap<String, String>>,
    inferred_description_map: Option<HashMap<String, String>>,
    actual_tests: Option<Vec<quary_proto::Test>>,
    inferred_tests: Option<Vec<Test>>,
) -> Result<Table, String> {
    let test_map = map_tests_for_column(modelling_prefix, actual_tests, inferred_tests)?;
    let description_map = make_description_map(inferred_description_map, defined_description_map);
    let inferred_columns = inferred_columns.map(|columns| {
        columns
            .into_iter()
            .map(|column| column.to_string())
            .collect()
    });
    let actual_columns = project_columns.map(|columns| {
        columns
            .into_iter()
            .map(|column| column.to_string())
            .collect()
    });
    let columns = make_column_vector(inferred_columns, actual_columns);

    let rows = columns
        .into_iter()
        .map(|column| match column {
            present_row::Row::PresentInSqlAndDefinitions(row) => {
                let tests = test_map.get(&row.title).cloned().unwrap_or_default();
                let not_present = Description::NotPresent(Default::default());
                let description = description_map
                    .get(&row.title)
                    .or(Some(&not_present))
                    .cloned()
                    .map(|description| RowDescription {
                        description: Some(description),
                    });
                present_row::Row::PresentInSqlAndDefinitions(Row {
                    description,
                    tests,
                    ..row
                })
            }
            present_row::Row::MissingInDefinitions(row) => {
                let tests = test_map.get(&row.title).cloned().unwrap_or_default();
                let not_present = Description::NotPresent(Default::default());
                let description = description_map
                    .get(&row.title)
                    .or(Some(&not_present))
                    .cloned()
                    .map(|description| RowDescription {
                        description: Some(description),
                    });
                present_row::Row::MissingInDefinitions(Row {
                    description,
                    tests,
                    ..row
                })
            }
            present_row::Row::PresentInDefinitionsButNotRecognisableInSql(row) => {
                let tests = test_map.get(&row.title).cloned().unwrap_or_default();
                let not_present = Description::NotPresent(Default::default());
                let description = description_map
                    .get(&row.title)
                    .or(Some(&not_present))
                    .cloned()
                    .map(|description| RowDescription {
                        description: Some(description),
                    });
                present_row::Row::PresentInDefinitionsButNotRecognisableInSql(Row {
                    description,
                    tests,
                    ..row
                })
            }
        })
        .map(|row| PresentRow { row: Some(row) })
        .collect::<Vec<_>>();

    Ok(Table {
        table_type: Some(TableType::Present(PresentInSchema { rows })),
    })
}

/// make_column_vector takes the actual schema and the inferred schema and maps them to a vector of
/// columns. The present rows are empty and need to be filled. It returns the rows as empty in terms
/// of the description and tests.
pub fn make_column_vector(
    inferred_columns: Option<Vec<String>>,
    actual_columns: Option<Vec<String>>,
) -> Vec<present_row::Row> {
    let actual_columns = actual_columns.unwrap_or_default();
    let inferred_columns = inferred_columns.unwrap_or_default();
    let actual_columns_map = actual_columns.clone().into_iter().collect::<HashSet<_>>();
    let inferred_columns_map = inferred_columns.clone().into_iter().collect::<HashSet<_>>();

    actual_columns
        .into_iter()
        .map(|column| {
            if inferred_columns_map.get(&column).is_some() {
                present_row::Row::PresentInSqlAndDefinitions(Row {
                    title: column.to_string(),
                    ..Default::default()
                })
            } else {
                present_row::Row::PresentInDefinitionsButNotRecognisableInSql(Row {
                    title: column.to_string(),
                    ..Default::default()
                })
            }
        })
        .chain(inferred_columns.into_iter().filter_map(|column| {
            if actual_columns_map.get(&column).is_none() {
                Some(present_row::Row::MissingInDefinitions(Row {
                    title: column.to_string(),
                    ..Default::default()
                }))
            } else {
                None
            }
        }))
        .collect()
}

/// make_description_map turns the two maps into a single map that contains all the keys from both maps.
fn make_description_map(
    inferred_map: Option<HashMap<String, String>>,
    present_map: Option<HashMap<String, String>>,
) -> HashMap<String, Description> {
    let inferred_map = inferred_map.unwrap_or_default();
    let present_map = present_map.unwrap_or_default();
    inferred_map
        .keys()
        .chain(present_map.keys())
        .map(|key| match (inferred_map.get(key), present_map.get(key)) {
            (Some(inferred), Some(present)) => {
                if inferred == present {
                    (
                        key.to_string(),
                        Description::PresentAndInferredIdentical(present.to_string()),
                    )
                } else {
                    (
                        key.to_string(),
                        Description::PresentWithDifferentInference(PresentWithInference {
                            present: present.to_string(),
                            inferred: inferred.to_string(),
                        }),
                    )
                }
            }
            (Some(inferred), None) => {
                (key.to_string(), Description::Inferred(inferred.to_string()))
            }
            (None, Some(present)) => (key.to_string(), Description::Present(present.to_string())),
            (None, None) => (key.to_string(), Description::NotPresent(Default::default())),
        })
        .collect()
}

/// map_tests_for_column takes the tests from the actual schema and the inferred schema and maps them
/// to a hashmap of column name to a vector of tuples of test name for rendering.
///
/// It returns a hashmap of column name to a vector of tuples of test name and whether the test is
/// present in the actual schema, inferred schema or both.
/// The order of the tests is the same is as
///   - actual tests first (as defined by the standard test order), any inferred tests are just
///     highlighted
///   - inferred tests second
fn map_tests_for_column(
    modelling_prefix: &str,
    actual_tests: Option<Vec<quary_proto::Test>>,
    inferred_tests: Option<Vec<Test>>,
) -> Result<HashMap<String, Vec<RowTest>>, String> {
    let inferred_tests: HashSet<Test> = inferred_tests
        .map(|tests| tests.into_iter().collect())
        .unwrap_or_default();
    let actual_tests: HashSet<Test> = actual_tests
        .unwrap_or_default()
        .into_iter()
        .filter_map(|test| map_test_to_sql_inference(modelling_prefix, test))
        .collect();
    let columns = actual_tests
        .iter()
        .chain(inferred_tests.iter())
        .map(Test::get_column)
        .collect::<HashSet<&str>>();

    type TestMap = HashMap<String, HashSet<Test>>;
    fn map_folder(mut map: TestMap, test: &Test) -> TestMap {
        let column = test.get_column();
        let tests = map.entry(column.to_string()).or_default();
        tests.insert(test.clone());
        map
    }
    let inferred_tests_map: TestMap = inferred_tests.iter().fold(HashMap::new(), map_folder);
    let actual_tests_map: TestMap = actual_tests.iter().fold(HashMap::new(), map_folder);

    enum TestPresence {
        Present,
        PresentAndInferred,
        Inferred,
    }

    let columns = columns
        .iter()
        .map(|column| -> Result<(&str, Vec<RowTest>), String> {
            let empty_inferred = HashSet::new();
            let empty_actual = HashSet::new();
            let inferred_tests = inferred_tests_map.get(*column).unwrap_or(&empty_inferred);
            let actual_tests = actual_tests_map.get(*column).unwrap_or(&empty_actual);

            let inferred_tests = inferred_tests
                .iter()
                .map(|test| (test.clone(), TestPresence::Inferred))
                .collect::<HashMap<Test, TestPresence>>();
            let actual_tests = actual_tests
                .iter()
                .map(|test| (test.clone(), TestPresence::Present))
                .collect::<HashMap<Test, TestPresence>>();
            let intersection = inferred_tests
                .keys()
                .filter(|test| actual_tests.contains_key(*test))
                .map(|test| (test.clone(), TestPresence::PresentAndInferred))
                .collect::<HashMap<Test, TestPresence>>();
            let mut tests = actual_tests
                .into_iter()
                .chain(inferred_tests)
                .chain(intersection)
                .collect::<HashMap<Test, TestPresence>>()
                .into_iter()
                .collect::<Vec<(Test, TestPresence)>>();

            tests.sort_by(|(test_a, presence_a), (test_b, presence_b)| {
                use TestPresence::*;
                match (presence_a, presence_b) {
                    (Present, Present) => test_a.get_ordering_key().cmp(&test_b.get_ordering_key()),
                    (Present, PresentAndInferred) => {
                        test_a.get_ordering_key().cmp(&test_b.get_ordering_key())
                    }
                    (Present, Inferred) => std::cmp::Ordering::Less,
                    (PresentAndInferred, Present) => {
                        test_a.get_ordering_key().cmp(&test_b.get_ordering_key())
                    }
                    (PresentAndInferred, PresentAndInferred) => {
                        test_a.get_ordering_key().cmp(&test_b.get_ordering_key())
                    }
                    (PresentAndInferred, Inferred) => std::cmp::Ordering::Less,
                    (Inferred, Present) => std::cmp::Ordering::Greater,
                    (Inferred, PresentAndInferred) => std::cmp::Ordering::Greater,
                    (Inferred, Inferred) => {
                        test_a.get_ordering_key().cmp(&test_b.get_ordering_key())
                    }
                }
            });

            let tests = tests
                .into_iter()
                .map(|(test, presence)| -> Result<row_test::Test, String> {
                    let short_string = test.short_test_string()?;
                    let (_, _, column_test) = map_test_to_column_test(modelling_prefix, test)?;

                    let test_details = RowTestDetails {
                        text: short_string,
                        column_test: Some(column_test),
                    };

                    let result_test = match presence {
                        TestPresence::Present => {
                            row_test::Test::PresentAndNotInferred(test_details)
                        }
                        TestPresence::PresentAndInferred => {
                            row_test::Test::PresentAndInferred(test_details)
                        }
                        TestPresence::Inferred => {
                            row_test::Test::NotPresentButInferred(test_details)
                        }
                    };

                    Ok(result_test)
                })
                .collect::<Result<Vec<row_test::Test>, String>>()?;

            let tests = tests
                .into_iter()
                .map(|test| RowTest { test: Some(test) })
                .collect::<Vec<RowTest>>();

            Ok((column, tests))
        })
        .collect::<Result<Vec<(&str, Vec<RowTest>)>, String>>()?
        .into_iter()
        .map(|(column, tests)| (column.to_string(), tests))
        .collect();

    Ok(columns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::ToTest;
    use quary_proto::test::TestType;
    use quary_proto::{
        ColumnTest, TestGreaterThan, TestLessThan, TestNotNull, TestRelationship, TestUnique,
    };
    use sqlinference::test::{ComparisonTest, RelationshipTest, StandardTest};

    #[test]
    fn make_column_vector_inferred_and_definitions() {
        let inferred_columns = Some(
            vec!["d", "a"]
                .into_iter()
                .map(|column| column.to_string())
                .collect(),
        );
        let actual_columns = Some(
            vec!["a", "b", "c"]
                .into_iter()
                .map(|column| column.to_string())
                .collect(),
        );

        let want = vec![
            present_row::Row::PresentInSqlAndDefinitions(Row {
                title: "a".to_string(),
                ..Default::default()
            }),
            present_row::Row::PresentInDefinitionsButNotRecognisableInSql(Row {
                title: "b".to_string(),
                ..Default::default()
            }),
            present_row::Row::PresentInDefinitionsButNotRecognisableInSql(Row {
                title: "c".to_string(),
                ..Default::default()
            }),
            present_row::Row::MissingInDefinitions(Row {
                title: "d".to_string(),
                ..Default::default()
            }),
        ];

        let out = make_column_vector(inferred_columns, actual_columns);

        assert_eq!(want, out);
    }

    #[test]
    fn make_column_vector_no_inferred() {
        let inferred_columns = None;
        let actual_columns = Some(
            vec!["a", "b", "c"]
                .into_iter()
                .map(|column| column.to_string())
                .collect(),
        );

        let want = vec![
            present_row::Row::PresentInDefinitionsButNotRecognisableInSql(Row {
                title: "a".to_string(),
                ..Default::default()
            }),
            present_row::Row::PresentInDefinitionsButNotRecognisableInSql(Row {
                title: "b".to_string(),
                ..Default::default()
            }),
            present_row::Row::PresentInDefinitionsButNotRecognisableInSql(Row {
                title: "c".to_string(),
                ..Default::default()
            }),
        ];

        let out = make_column_vector(inferred_columns, actual_columns);

        assert_eq!(want, out);
    }

    #[test]
    fn make_column_vector_no_definition() {
        let inferred_columns = Some(
            vec!["a", "b", "c"]
                .into_iter()
                .map(|column| column.to_string())
                .collect(),
        );
        let actual_columns = None;

        let want = vec![
            present_row::Row::MissingInDefinitions(Row {
                title: "a".to_string(),
                ..Default::default()
            }),
            present_row::Row::MissingInDefinitions(Row {
                title: "b".to_string(),
                ..Default::default()
            }),
            present_row::Row::MissingInDefinitions(Row {
                title: "c".to_string(),
                ..Default::default()
            }),
        ];

        let out = make_column_vector(inferred_columns, actual_columns);

        assert_eq!(want, out);
    }

    #[test]
    fn make_description_map_test_no_inferred() {
        let inferred_map = None;
        let present_map = Some(
            [("a", "a desc"), ("b", "b desc"), ("c", "d desc")]
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<HashMap<String, String>>(),
        );

        let want = [("a", "a desc"), ("b", "b desc"), ("c", "d desc")]
            .into_iter()
            .map(|(key, value)| (key.to_string(), Description::Present(value.to_string())))
            .collect::<HashMap<String, Description>>();

        let out = make_description_map(inferred_map, present_map);

        assert_eq!(want, out);
    }

    #[test]
    fn make_description_map_test_no_present() {
        let present_map = None;
        let inferred_map = Some(
            [("a", "a desc"), ("b", "b desc"), ("c", "d desc")]
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<HashMap<String, String>>(),
        );

        let want = [("a", "a desc"), ("b", "b desc"), ("c", "d desc")]
            .into_iter()
            .map(|(key, value)| (key.to_string(), Description::Inferred(value.to_string())))
            .collect::<HashMap<String, Description>>();

        let out = make_description_map(inferred_map, present_map);

        assert_eq!(want, out);
    }

    #[test]
    fn make_description_map_test() {
        let inferred_descriptions = [("a", "a"), ("b", "b"), ("c", "c")];
        let present_descriptions = [("a", "a"), ("b", "c"), ("d", "d")];

        let expected = [
            (
                "a",
                Description::PresentAndInferredIdentical("a".to_string()),
            ),
            (
                "b",
                Description::PresentWithDifferentInference(PresentWithInference {
                    present: "c".to_string(),
                    inferred: "b".to_string(),
                }),
            ),
            ("c", Description::Inferred("c".to_string())),
            ("d", Description::Present("d".to_string())),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<HashMap<String, Description>>();

        let to_map = |descriptions: &[(&str, &str)]| {
            descriptions
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<HashMap<String, String>>()
        };
        let inferred_map = Some(to_map(&inferred_descriptions));
        let present_map = Some(to_map(&present_descriptions));

        let out = make_description_map(inferred_map, present_map);

        assert_eq!(expected, out);
    }

    #[test]
    fn map_tests_for_column_test_only_inferred() {
        let modelling_prefix = "modelling_prefix";

        let actual_tests = None;
        let inferred_tests = Some(vec![
            Test::NotNull(StandardTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "a".to_string(),
            }),
            Test::LessThan(ComparisonTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "a".to_string(),
                value: "100".to_string(),
            }),
            Test::GreaterThan(ComparisonTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "a".to_string(),
                value: "2".to_string(),
            }),
            Test::Unique(StandardTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "a".to_string(),
            }),
            Test::Unique(StandardTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "b".to_string(),
            }),
            Test::Relationship(RelationshipTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "b".to_string(),
                target_reference: "modelling_prefix.target_path".to_string(),
                target_column: "b".to_string(),
            }),
        ]);

        let expect = [
            (
                "a",
                vec![
                    row_test::Test::NotPresentButInferred(RowTestDetails {
                        text: "not null".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "not_null".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::NotPresentButInferred(RowTestDetails {
                        text: "unique".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::NotPresentButInferred(RowTestDetails {
                        text: "> 2".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "gt".to_string(),
                            info: [("value", "2")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                    row_test::Test::NotPresentButInferred(RowTestDetails {
                        text: "< 100".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "lt".to_string(),
                            info: [("value", "100")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                ],
            ),
            (
                "b",
                vec![
                    row_test::Test::NotPresentButInferred(RowTestDetails {
                        text: "unique".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::NotPresentButInferred(RowTestDetails {
                        text: "relationship (modelling_prefix.target_path, b)".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "relationship".to_string(),
                            info: [("model", "target_path"), ("column", "b")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                ],
            ),
        ]
        .into_iter()
        .map(|(key, tests)| {
            (
                key.to_string(),
                tests
                    .into_iter()
                    .map(|test| RowTest { test: Some(test) })
                    .collect(),
            )
        })
        .collect::<HashMap<String, Vec<_>>>();

        let out = map_tests_for_column(modelling_prefix, actual_tests, inferred_tests).unwrap();

        assert_eq!(expect, out);
    }

    #[test]
    fn map_tests_for_column_test_no_inferred() {
        let modelling_prefix = "modelling_prefix";
        let actual_tests = Some(
            [
                TestType::NotNull(TestNotNull {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "a".to_string(),
                    ..Default::default()
                }),
                TestType::LessThan(TestLessThan {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "a".to_string(),
                    value: "100".to_string(),
                    ..Default::default()
                }),
                TestType::GreaterThan(TestGreaterThan {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "a".to_string(),
                    value: "2".to_string(),
                    ..Default::default()
                }),
                TestType::Unique(TestUnique {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "a".to_string(),
                    ..Default::default()
                }),
                TestType::Unique(TestUnique {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "b".to_string(),
                    ..Default::default()
                }),
                TestType::Relationship(TestRelationship {
                    source_path: "modelling_prefix.model_path".to_string(),
                    source_model: "model_path".to_string(),
                    source_column: "b".to_string(),
                    target_path: "modelling_prefix.target_path".to_string(),
                    target_model: "target_path".to_string(),
                    target_column: "c".to_string(),
                    ..Default::default()
                }),
            ]
            .iter()
            .map(|test| test.to_test())
            .collect(),
        );
        let inferred_tests = None;

        let expect = [
            (
                "a",
                vec![
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "not null".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "not_null".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "unique".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "> 2".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "gt".to_string(),
                            info: [("value", "2")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "< 100".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "lt".to_string(),
                            info: [("value", "100")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                ],
            ),
            (
                "b",
                vec![
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "unique".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "relationship (modelling_prefix.target_path, c)".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "relationship".to_string(),
                            info: [("model", "target_path"), ("column", "c")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                ],
            ),
        ]
        .into_iter()
        .map(|(key, tests)| {
            (
                key.to_string(),
                tests
                    .into_iter()
                    .map(|test| RowTest { test: Some(test) })
                    .collect(),
            )
        })
        .collect::<HashMap<String, Vec<_>>>();

        let out = map_tests_for_column(modelling_prefix, actual_tests, inferred_tests).unwrap();

        assert_eq!(expect, out);
    }

    #[test]
    fn map_tests_for_column_test_inferred_and_present() {
        let modelling_prefix = "modelling_prefix";

        let actual_tests = Some(
            [
                TestType::LessThan(TestLessThan {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "a".to_string(),
                    value: "100".to_string(),
                    ..Default::default()
                }),
                TestType::GreaterThan(TestGreaterThan {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "a".to_string(),
                    value: "2".to_string(),
                    ..Default::default()
                }),
                TestType::Unique(TestUnique {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "a".to_string(),
                    ..Default::default()
                }),
                TestType::Unique(TestUnique {
                    path: "modelling_prefix.model_path".to_string(),
                    model: "model_path".to_string(),
                    column: "b".to_string(),
                    ..Default::default()
                }),
                TestType::Relationship(TestRelationship {
                    source_path: "modelling_prefix.model_path".to_string(),
                    source_model: "model_path".to_string(),
                    source_column: "b".to_string(),
                    target_path: "modelling_prefix.target_path".to_string(),
                    target_model: "target_path".to_string(),
                    target_column: "c".to_string(),
                    ..Default::default()
                }),
            ]
            .iter()
            .map(|test| test.to_test())
            .collect(),
        );
        let inferred_tests = Some(vec![
            Test::NotNull(StandardTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "a".to_string(),
            }),
            Test::LessThan(ComparisonTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "a".to_string(),
                value: "100".to_string(),
            }),
            Test::Unique(StandardTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "b".to_string(),
            }),
            Test::Relationship(RelationshipTest {
                path: "modelling_prefix.model_path".to_string(),
                column: "b".to_string(),
                target_reference: "modelling_prefix.target_path".to_string(),
                target_column: "c".to_string(),
            }),
        ]);

        let expect = [
            (
                "a",
                vec![
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "unique".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::PresentAndNotInferred(RowTestDetails {
                        text: "> 2".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "gt".to_string(),
                            info: [("value", "2")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                    row_test::Test::PresentAndInferred(RowTestDetails {
                        text: "< 100".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "lt".to_string(),
                            info: [("value", "100")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                    row_test::Test::NotPresentButInferred(RowTestDetails {
                        text: "not null".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "not_null".to_string(),
                            info: Default::default(),
                        }),
                    }),
                ],
            ),
            (
                "b",
                vec![
                    row_test::Test::PresentAndInferred(RowTestDetails {
                        text: "unique".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        }),
                    }),
                    row_test::Test::PresentAndInferred(RowTestDetails {
                        text: "relationship (modelling_prefix.target_path, c)".to_string(),
                        column_test: Some(ColumnTest {
                            r#type: "relationship".to_string(),
                            info: [("model", "target_path"), ("column", "c")]
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect::<HashMap<String, String>>(),
                        }),
                    }),
                ],
            ),
        ]
        .into_iter()
        .map(|(key, tests)| {
            (
                key.to_string(),
                tests
                    .into_iter()
                    .map(|test| RowTest { test: Some(test) })
                    .collect(),
            )
        })
        .collect::<HashMap<String, Vec<_>>>();

        let out = map_tests_for_column(modelling_prefix, actual_tests, inferred_tests);

        assert_eq!(Ok(expect), out);
    }
}
