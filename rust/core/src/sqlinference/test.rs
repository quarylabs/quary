#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Test {
    /// NotNull tests assert that none of the values in a particular column are
    /// null.
    NotNull(StandardTest),
    /// Unique tests assert that all of the values in a particular column are
    /// Unique. It ignores Null entries.
    ///
    /// For testing truly unique values, a 'Unique' test should be combined with
    /// a 'NotNull' test.
    Unique(StandardTest),
    /// Relationship tests asser that all the values in a particular column
    /// are present in the referenced relationship. It ignores Null entries.
    ///
    /// For testing truly present values, a 'Relationship' test should be
    /// combined with a 'NotNull' test.
    Relationship(RelationshipTest),
    /// Relationship tests asser that all the values in a particular column
    /// are one of the specified values. It ignores Null entries.
    ///
    /// For testing exact matches only, a 'Relationship' test should be combined
    /// with a 'NotNull' test.
    AcceptedValues(AcceptedValuesTest),
    /// GreaterThanOrEqual tests asser that all the values in a particular
    /// column are  greater than or equal the specified value. It ignores
    /// Null entries.
    ///
    /// For only greater than or equal values with no nulls, a 'NotNull' test
    /// should be combined with the test.
    GreaterThanOrEqual(ComparisonTest),
    /// GreaterThan tests asser that all the values in a particular column
    /// are greater than the specified value. It ignores Null entries.
    ///
    /// For only greater than values with no nulls, a 'NotNull' test should be
    /// combined with the test.
    GreaterThan(ComparisonTest),
    /// LessThanOrEqual tests asser that all the values in a particular
    /// column are less than or equal than the specified value. It ignores
    /// Null entries.
    ///
    /// For only less than or equal values with no nulls, a 'NotNull' test
    /// should be combined with the test.
    LessThanOrEqual(ComparisonTest),
    /// GreaterThanOrEqual tests asser that all the values in a particular
    /// column are less than specified value. It ignores Null entries.
    ///
    /// For only less than values with no nulls, a 'NotNull' test should be
    /// combined with the test.
    LessThan(ComparisonTest),
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct StandardTest {
    pub path: String,
    pub column: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct ComparisonTest {
    pub path: String,
    pub column: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct RelationshipTest {
    pub path: String,
    pub column: String,
    pub target_reference: String,
    pub target_column: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct AcceptedValuesTest {
    pub path: String,
    pub column: String,
    pub values: Vec<String>,
}

impl Test {
    /// get_ordering_key returns an ordering key for the test types. This is
    /// used to order them logically in a UI and consistently.
    pub fn get_ordering_key(&self) -> usize {
        match self {
            Test::NotNull(_) => 0,
            Test::Unique(_) => 1,
            Test::Relationship(_) => 2,
            Test::AcceptedValues(_) => 3,
            Test::GreaterThanOrEqual(_) => 4,
            Test::GreaterThan(_) => 5,
            Test::LessThanOrEqual(_) => 6,
            Test::LessThan(_) => 7,
        }
    }

    /// get_column returns the column the test applies to.
    pub fn get_column(&self) -> &str {
        match self {
            Test::NotNull(test) => &test.column,
            Test::Unique(test) => &test.column,
            Test::Relationship(test) => &test.column,
            Test::AcceptedValues(test) => &test.column,
            Test::GreaterThanOrEqual(test) => &test.column,
            Test::GreaterThan(test) => &test.column,
            Test::LessThanOrEqual(test) => &test.column,
            Test::LessThan(test) => &test.column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_column_test() {
        let column = "gibberish_column";

        let tests = [
            Test::NotNull(StandardTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
            }),
            Test::Unique(StandardTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
            }),
            Test::Relationship(RelationshipTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
                target_reference: "gibberish_target_reference".to_string(),
                target_column: "gibberish_target_column".to_string(),
            }),
            Test::AcceptedValues(AcceptedValuesTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
                values: vec!["gibberish_value".to_string()],
            }),
            Test::GreaterThanOrEqual(ComparisonTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
                value: "gibberish_value".to_string(),
            }),
            Test::GreaterThan(ComparisonTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
                value: "gibberish_value".to_string(),
            }),
            Test::LessThanOrEqual(ComparisonTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
                value: "gibberish_value".to_string(),
            }),
            Test::LessThan(ComparisonTest {
                path: "gibberish_path".to_string(),
                column: column.to_string(),
                value: "gibberish_value".to_string(),
            }),
        ];

        for test in tests.iter() {
            assert_eq!(test.get_column(), column);
        }
    }
}
