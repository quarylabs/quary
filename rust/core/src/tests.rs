use quary_proto::test::TestType;
use quary_proto::{
    Project, Test, TestAcceptedValues, TestGreaterThan, TestGreaterThanOrEqual, TestLessThan,
    TestLessThanOrEqual, TestNotNull, TestRelationship, TestUnique,
};
use std::cmp::Ordering;
use std::path::PathBuf;

pub fn test_to_name(test: &Test) -> Result<String, String> {
    match &test.clone().test_type.ok_or(
        "test_type is not set. This is a bug in the code. Please report this to the developers.",
    )? {
        TestType::NotNull(test) => Ok(format!("test_{}_{}_not_null", test.model, test.column)),
        TestType::Unique(test) => Ok(format!("test_{}_{}_unique", test.model, test.column)),
        TestType::Relationship(test) => Ok(format!(
            "test_{}_{}_relationship_{}_{}",
            test.source_model, test.source_column, test.target_model, test.target_column
        )),
        TestType::AcceptedValues(test) => Ok(format!(
            "test_{}_{}_accepted_values",
            test.model, test.column
        )),
        TestType::GreaterThanOrEqual(test) => Ok(format!(
            "test_{}_{}_greater_than_or_equal",
            test.model, test.column,
        )),
        TestType::LessThanOrEqual(test) => Ok(format!(
            "test_{}_{}_less_than_or_equal",
            test.model, test.column,
        )),
        TestType::GreaterThan(test) => {
            Ok(format!("test_{}_{}_greater_than", test.model, test.column,))
        }
        TestType::LessThan(test) => Ok(format!("test_{}_{}_less_than", test.model, test.column,)),
        TestType::Sql(test) => {
            let file_path = test.file_path.to_string();
            let path_stem = PathBuf::from(file_path.clone())
                .file_stem()
                .ok_or(format!("Could not get file stem for path {}", file_path))?
                .to_str()
                .ok_or(format!(
                    "Could not convert file stem to string for path {}",
                    file_path
                ))?
                .to_string();

            Ok(format!("test_sql_{}", path_stem))
        }
    }
}

pub(crate) trait ToSql {
    fn to_sql(&self, limit: Option<usize>) -> String;
}

impl ToSql for TestNotNull {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM {} WHERE {} IS NULL{}",
            self.path, self.column, limit
        )
    }
}

impl ToSql for TestUnique {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM (SELECT {} FROM {} WHERE {} IS NOT NULL GROUP BY {} HAVING COUNT(*) > 1){}",
            self.column, self.path, self.column, self.column, limit
        )
    }
}

impl ToSql for TestRelationship {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM {} WHERE {} IS NOT NULL AND {} NOT IN (SELECT {} FROM {}){}",
            self.source_path,
            self.source_column,
            self.source_column,
            self.target_column,
            self.target_path,
            limit
        )
    }
}

impl ToSql for TestAcceptedValues {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM {} WHERE {} IS NOT NULL AND {} NOT IN ({}){}",
            self.path,
            self.column,
            self.column,
            generate_sql_in_list(self.accepted_values.clone()),
            limit
        )
    }
}

impl ToSql for TestGreaterThanOrEqual {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM {} WHERE {} < {}{}",
            self.path, self.column, self.value, limit
        )
    }
}

impl ToSql for TestLessThanOrEqual {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM {} WHERE {} > {}{}",
            self.path, self.column, self.value, limit
        )
    }
}

impl ToSql for TestGreaterThan {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM {} WHERE {} <= {}{}",
            self.path, self.column, self.value, limit
        )
    }
}

impl ToSql for TestLessThan {
    fn to_sql(&self, limit: Option<usize>) -> String {
        let limit = limit
            .map(|limit| format!(" LIMIT {}", limit))
            .unwrap_or_default();
        format!(
            "SELECT * FROM {} WHERE {} >= {}{}",
            self.path, self.column, self.value, limit
        )
    }
}

fn generate_sql_in_list(values: Vec<String>) -> String {
    let mut result = String::new();
    for value in values {
        result.push_str(&format!("'{}',", value));
    }
    result.pop();
    result
}

/// return_test_for_model_column finds tests that are relevant to a specified model and column
// TODO may want to attach this to a project directly
pub fn return_test_for_model_column<'a>(
    project: &'a Project,
    model: &'a str,
    column: &'a str,
) -> impl Iterator<Item = &'a Test> + 'a {
    project.tests.values().filter(move |test| {
        if let Some(test) = &test.test_type {
            match test {
                TestType::Sql(_) => false,
                TestType::Unique(test) => test.model == model && test.column == column,
                TestType::NotNull(test) => test.model == model && test.column == column,
                TestType::Relationship(test) => {
                    test.source_model == model && test.source_column == column
                }
                TestType::AcceptedValues(test) => test.model == model && test.column == column,
                TestType::GreaterThanOrEqual(test) => test.model == model && test.column == column,
                TestType::LessThanOrEqual(test) => test.model == model && test.column == column,
                TestType::GreaterThan(test) => test.model == model && test.column == column,
                TestType::LessThan(test) => test.model == model && test.column == column,
            }
        } else {
            false
        }
    })
}

pub trait ShortTestString {
    /// short_test_string is a short string that can be used to identify a test and can be used for
    /// quick visuals, for example in a test report. An example would be for a GreaterThanEqual Test
    /// it would be ">= 100".
    fn short_test_string(&self) -> Result<String, String>;
}

use sqlinference::test::Test as SqlInferenceTest;

impl ShortTestString for SqlInferenceTest {
    // TODO Need to add tests for this
    fn short_test_string(&self) -> Result<String, String> {
        Ok(match self {
            sqlinference::test::Test::Unique(_) => "unique".to_string(),
            sqlinference::test::Test::NotNull(_) => "not null".to_string(),
            sqlinference::test::Test::Relationship(test) => format!(
                "relationship ({}, {})",
                test.target_reference, test.target_column
            ),
            sqlinference::test::Test::AcceptedValues(test) => {
                format!("one of ({})", test.values.join("; ")).to_string()
            }
            sqlinference::test::Test::GreaterThanOrEqual(test) => {
                format!("≥ {}", test.value).to_string()
            }
            sqlinference::test::Test::LessThanOrEqual(test) => {
                format!("≤ {}", test.value).to_string()
            }
            sqlinference::test::Test::GreaterThan(test) => format!("> {}", test.value).to_string(),
            sqlinference::test::Test::LessThan(test) => format!("< {}", test.value).to_string(),
        })
    }
}

impl ShortTestString for Test {
    // TODO The following should probably rely on the implementation for sql_inference::test::Test
    fn short_test_string(&self) -> Result<String, String> {
        match &self.test_type {
            Some(TestType::Sql(_)) => Ok("sql test".to_string()),
            Some(TestType::Unique(_)) => Ok("unique".to_string()),
            Some(TestType::NotNull(_)) => Ok("not null".to_string()),
            Some(TestType::Relationship(test)) => Ok(format!(
                "relationship ({}, {})",
                test.target_model, test.target_column
            )),
            Some(TestType::AcceptedValues(test)) => {
                Ok(format!("one of ({})", test.accepted_values.join("; ")).to_string())
            }
            Some(TestType::GreaterThanOrEqual(test)) => Ok(format!("≥ {}", test.value).to_string()),
            Some(TestType::LessThanOrEqual(test)) => Ok(format!("≤ {}", test.value).to_string()),
            Some(TestType::GreaterThan(test)) => Ok(format!("> {}", test.value).to_string()),
            Some(TestType::LessThan(test)) => Ok(format!("< {}", test.value).to_string()),
            None => Err("test_type is not set".to_string()),
        }
    }
}

// test_by_type is the order to show the tests in a report. It is the standard ordering of tests.
fn test_by_type(a: &Test, b: &Test) -> Ordering {
    fn test_order(test: &Test) -> usize {
        match test.test_type {
            Some(TestType::NotNull(_)) => 1,
            Some(TestType::Unique(_)) => 2,
            Some(TestType::AcceptedValues(_)) => 3,
            Some(TestType::Relationship(_)) => 4,
            Some(TestType::GreaterThanOrEqual(_)) => 5,
            Some(TestType::GreaterThan(_)) => 6,
            Some(TestType::LessThanOrEqual(_)) => 7,
            Some(TestType::LessThan(_)) => 8,
            Some(TestType::Sql(_)) => 9,
            None => 10,
        }
    }

    test_order(a).cmp(&test_order(b))
}

#[cfg(test)]
mod test {
    use crate::test_helpers::ToTest;
    use crate::tests::{test_by_type, ShortTestString, ToSql};
    use quary_proto::test::TestType;
    use quary_proto::{
        TestAcceptedValues, TestGreaterThanOrEqual, TestLessThanOrEqual, TestNotNull,
        TestRelationship, TestSqlFile, TestUnique,
    };

    #[test]
    fn test_test_to_name() {
        let test = vec![
            (
                TestType::NotNull(TestNotNull {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                })
                .to_test(),
                "test_users_id_not_null",
            ),
            (
                TestType::AcceptedValues(TestAcceptedValues {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    accepted_values: vec![],
                })
                .to_test(),
                "test_users_id_accepted_values",
            ),
            (
                TestType::Unique(TestUnique {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                })
                .to_test(),
                "test_users_id_unique",
            ),
            (
                TestType::LessThanOrEqual(TestLessThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
                })
                .to_test(),
                "test_users_id_less_than_or_equal",
            ),
            (
                TestType::GreaterThanOrEqual(TestGreaterThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
                })
                .to_test(),
                "test_users_id_greater_than_or_equal",
            ),
            (
                TestType::Relationship(TestRelationship {
                    file_path: "".to_string(),
                    source_model: "users".to_string(),
                    source_path: "".to_string(),
                    source_column: "id".to_string(),
                    target_model: "usersource".to_string(),
                    target_path: "".to_string(),
                    target_column: "idsource".to_string(),
                })
                .to_test(),
                "test_users_id_relationship_usersource_idsource",
            ),
            (
                TestType::Sql(TestSqlFile {
                    file_path: "/test/hello.sql".to_string(),
                    references: vec![],
                })
                .to_test(),
                "test_sql_hello",
            ),
        ];

        for (test, expected) in test {
            let got = super::test_to_name(&test);

            assert_eq!(Ok(expected.to_string()), got);
        }
    }

    #[test]
    fn test_test_to_sql() {
        let test = vec![
            (
                TestNotNull {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                }
                .to_sql(None),
                "SELECT * FROM users_123 WHERE id IS NULL",
            ),
            (
                TestNotNull {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                }
                .to_sql(Some(100)),
                "SELECT * FROM users_123 WHERE id IS NULL LIMIT 100",
            ),
            (
                TestAcceptedValues {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    accepted_values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
                }.to_sql(None),
                "SELECT * FROM users_123 WHERE id IS NOT NULL AND id NOT IN ('1','2','3')",
            ),
            (
                TestAcceptedValues {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    accepted_values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
                }.to_sql(Some(100)),
                "SELECT * FROM users_123 WHERE id IS NOT NULL AND id NOT IN ('1','2','3') LIMIT 100",
            ),
            (
                TestUnique {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
            }.to_sql(None),
                "SELECT * FROM (SELECT id FROM users_123 WHERE id IS NOT NULL GROUP BY id HAVING COUNT(*) > 1)",
            ),
            (
                TestUnique {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
            }.to_sql(Some(100)),
                "SELECT * FROM (SELECT id FROM users_123 WHERE id IS NOT NULL GROUP BY id HAVING COUNT(*) > 1) LIMIT 100",
            ),
            (
                TestLessThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
            }.to_sql(None),
    "SELECT * FROM users_123 WHERE id > 100",
            ),
            (
                TestLessThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
            }.to_sql(Some(100)),
    "SELECT * FROM users_123 WHERE id > 100 LIMIT 100",
            ),
            (
                TestGreaterThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
                }.to_sql(None),
                "SELECT * FROM users_123 WHERE id < 100",
            ),
            (
                TestGreaterThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
                }.to_sql(Some(100)),
                "SELECT * FROM users_123 WHERE id < 100 LIMIT 100",
            ),
            (
                TestRelationship {
                    file_path: "".to_string(),
                    source_model: "users".to_string(),
                    source_path: "users_123".to_string(),
                    source_column: "id".to_string(),
                    target_model: "usersource".to_string(),
                    target_path: "usersource_123".to_string(),
                    target_column: "idsource".to_string(),
                }.to_sql(None),
                "SELECT * FROM users_123 WHERE id IS NOT NULL AND id NOT IN (SELECT idsource FROM usersource_123)", 
            ),
            (
                TestRelationship {
                    file_path: "".to_string(),
                    source_model: "users".to_string(),
                    source_path: "users_123".to_string(),
                    source_column: "id".to_string(),
                    target_model: "usersource".to_string(),
                    target_path: "usersource_123".to_string(),
                    target_column: "idsource".to_string(),
                }.to_sql(Some(100)),
                "SELECT * FROM users_123 WHERE id IS NOT NULL AND id NOT IN (SELECT idsource FROM usersource_123) LIMIT 100", 
            ),
        ];

        for (test, expected) in test {
            assert_eq!(expected, test);
        }
    }

    #[test]
    fn test_short_test_string() {
        let tests = [
            (
                TestType::NotNull(TestNotNull {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                }),
                "not null",
            ),
            (
                TestType::AcceptedValues(TestAcceptedValues {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    accepted_values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
                }),
                "one of (1; 2; 3)",
            ),
            (
                TestType::Unique(TestUnique {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                }),
                "unique",
            ),
            (
                TestType::LessThanOrEqual(TestLessThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
                }),
                "≤ 100",
            ),
            (
                TestType::GreaterThanOrEqual(TestGreaterThanOrEqual {
                    file_path: "".to_string(),
                    model: "users".to_string(),
                    path: "users_123".to_string(),
                    column: "id".to_string(),
                    value: "100".to_string(),
                }),
                "≥ 100",
            ),
            (
                TestType::Relationship(TestRelationship {
                    file_path: "".to_string(),
                    source_model: "users".to_string(),
                    source_path: "users_123".to_string(),
                    source_column: "id".to_string(),
                    target_model: "usersource".to_string(),
                    target_path: "usersource_123".to_string(),
                    target_column: "idsource".to_string(),
                }),
                "relationship (usersource, idsource)",
            ),
        ];

        for (test, expected) in tests {
            let got = test.to_test().short_test_string();

            assert_eq!(Ok(expected.to_string()), got);
        }
    }

    #[test]
    fn test_sort_column_tests() {
        let mut tests = vec![
            TestType::Unique(Default::default()).to_test(),
            TestType::NotNull(Default::default()).to_test(),
            TestType::Relationship(Default::default()).to_test(),
            TestType::AcceptedValues(Default::default()).to_test(),
            TestType::LessThanOrEqual(Default::default()).to_test(),
            TestType::Sql(Default::default()).to_test(),
            TestType::GreaterThanOrEqual(Default::default()).to_test(),
        ];

        let want = vec![
            TestType::NotNull(Default::default()).to_test(),
            TestType::Unique(Default::default()).to_test(),
            TestType::AcceptedValues(Default::default()).to_test(),
            TestType::Relationship(Default::default()).to_test(),
            TestType::GreaterThanOrEqual(Default::default()).to_test(),
            TestType::LessThanOrEqual(Default::default()).to_test(),
            TestType::Sql(Default::default()).to_test(),
        ];

        tests.sort_by(test_by_type);

        assert_eq!(want, tests);
    }
}

// // TestGenerateTestSqlAcceptedValues_ActuallyWorks tests that the generated SQL actually works.
// TODO This shall eventually need to be changed to work with all the databases.
// TODO Implement the following tests:
// func TestGenerateTestSqlAcceptedValues_ActuallyWorks(t *testing.T) {
// 	t.Parallel()
//
// 	tests := []struct {
// 		name         string
// 		databasePrep func(context context.Context, db databases.Database) error
// 		test         *servicev1.TestAcceptedValues
// 		wantRows     int
// 	}{
// 		{
// 			name: "simple example, valid values",
// 			databasePrep: func(ctx context.Context, db databases.Database) error {
// 				_, err := db.ExecContext(ctx, "CREATE TABLE users_123 (id INT)")
// 				if err != nil {
// 					return err
// 				}
// 				_, err = db.ExecContext(ctx, "INSERT INTO users_123 (id) VALUES (1), (2), (3)")
// 				if err != nil {
// 					return err
// 				}
// 				return nil
// 			},
// 			test: &servicev1.TestAcceptedValues{
// 				FilePath:       "test/path.yaml",
// 				Model:          "users",
// 				Path:           "users_123",
// 				Column:         "id",
// 				AcceptedValues: []string{"1", "2", "3"},
// 			},
// 			wantRows: 0,
// 		},
// 		{
// 			name: "2 invalid values",
// 			databasePrep: func(ctx context.Context, db databases.Database) error {
// 				_, err := db.ExecContext(ctx, "CREATE TABLE users_123 (id INT)")
// 				if err != nil {
// 					return err
// 				}
// 				_, err = db.ExecContext(ctx, "INSERT INTO users_123 (id) VALUES (1), (2), (3), (4), (5)")
// 				if err != nil {
// 					return err
// 				}
// 				return nil
// 			},
// 			test: &servicev1.TestAcceptedValues{
// 				FilePath:       "test/path.yaml",
// 				Model:          "users",
// 				Path:           "users_123",
// 				Column:         "id",
// 				AcceptedValues: []string{"1", "2", "3"},
// 			},
// 			wantRows: 2,
// 		},
// 	}
//
// 	for _, tt := range tests {
// 		t.Run(tt.name, func(t *testing.T) {
// 			ctx := context.Background()
// 			db, err := databasesImplementation.NewSqlLiteInMemory()
// 			require.NoError(t, err)
//
// 			err = tt.databasePrep(ctx, db)
// 			require.NoError(t, err)
//
// 			got := GenerateTestSqlAcceptedValues(tt.test)
//
// 			rows, err := db.QueryContext(ctx, got)
// 			require.NoError(t, err)
//
// 			defer rows.Close()
// 			count := 0
// 			for rows.Next() {
// 				count += 1
// 			}
//
// 			assert.Equal(t, tt.wantRows, count)
// 		})
// 	}
// }
