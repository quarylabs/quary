use quary_proto::{ColumnTest, ProjectFileColumn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

use crate::stats::ConvertDbtStats;

// DBT Project File Definition
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProjectFile {
    version: Option<i32>,
    models: Option<Vec<Model>>,
    sources: Option<Vec<Source>>,
}

impl ProjectFile {
    pub(crate) fn from_yaml(reader: impl io::Read) -> serde_yaml::Result<Self> {
        serde_yaml::from_reader(reader)
    }

    pub(crate) fn to_quary(
        &self,
        stats: &mut ConvertDbtStats,
    ) -> Result<quary_proto::ProjectFile, String> {
        let models = self
            .models
            .as_ref()
            .map_or(Ok(Vec::new()), |models| {
                models
                    .iter()
                    .map(|model| Model::to_quary(model, stats))
                    .collect()
            })
            .unwrap_or_default();

        let sources = self
            .sources
            .as_ref()
            .map(|sources| {
                sources
                    .iter()
                    .flat_map(|source| source.to_quary().unwrap())
                    .collect()
            })
            .unwrap_or_default();

        Ok(quary_proto::ProjectFile {
            sources,
            models,
            snapshots: vec![], // TODO: support conversion of snapshots (dbt -> Quary) QUA-468
        })
    }
}

// DBT Model Definition
#[derive(Debug, Serialize, Deserialize)]
struct Model {
    name: String,
    description: Option<String>,
    columns: Option<Vec<Column>>,
}

impl Model {
    fn to_quary(
        &self,
        stats: &mut ConvertDbtStats,
    ) -> Result<quary_proto::project_file::Model, String> {
        let columns = self.columns.as_ref().map_or(Ok(Vec::new()), |columns| {
            columns
                .iter()
                .map(|column| Column::to_quary(column, stats))
                .collect()
        })?;
        Ok(quary_proto::project_file::Model {
            database_config: None,
            name: self.name.clone(),
            description: self.description.clone(),
            tags: vec![],
            materialization: None,
            columns,
            tests: vec![],
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Column {
    name: String,
    description: Option<String>,
    tests: Option<Vec<Test>>,
}

impl Column {
    fn to_quary(&self, stats: &mut ConvertDbtStats) -> Result<ProjectFileColumn, String> {
        Ok(ProjectFileColumn {
            name: self.name.clone(),
            description: self.description.clone(),
            tests: self
                .tests
                .as_ref()
                .map(|tests| {
                    tests
                        .iter()
                        .filter_map(|test| match test.to_quary() {
                            Ok(test) => {
                                stats.tests_created += 1;
                                Some(test)
                            }
                            Err(err) => {
                                stats.errors.push(err.clone());
                                None
                            }
                        }) // Filter out Err results
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}

// DBT Source Definition
#[derive(Debug, Serialize, Deserialize)]
struct Source {
    name: String,
    database: Option<String>,
    schema: Option<String>,
    tables: Option<Vec<Table>>,
}

impl Source {
    fn to_quary(&self) -> Result<Vec<quary_proto::ProjectFileSource>, String> {
        let sources: Vec<quary_proto::ProjectFileSource> = self
            .tables
            .as_ref()
            .map(|tables| {
                tables
                    .iter()
                    .map(|table| quary_proto::ProjectFileSource {
                        name: format!("{}_{}", self.name, table.name.clone()),
                        path: format!(
                            "{}.{}.{}",
                            self.database.clone().unwrap_or_default(),
                            self.schema.clone().unwrap_or_default(),
                            table.name.clone()
                        ),
                        tags: vec![],
                        tests: vec![],
                        description: table.description.clone(),
                        columns: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(sources)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    name: String,
    description: Option<String>,
    tests: Option<Vec<Test>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Relationship {
    to: String,
    field: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Test {
    Simple(String),
    Relationship(RelationshipTest),
    AcceptedValuesString(AcceptedValuesTestString),
    AcceptedValuesNumber(AcceptedValuesTestNumber),
    // Quary does not support these tests
    ExpectColumnMeanToBeBetween(ExpectColumnMeanToBeBetweenTest),
    Complex(HashMap<String, Vec<String>>),
    // Handle hardcoded complex tests
    ComplexNotNullConfigSeverity(ComplexNotNullConfigSeverity),
    ComplexNotNullWhere(ComplexNotNullWhere),
}

impl Test {
    fn to_quary(&self) -> Result<ColumnTest, String> {
        match self {
            Test::Simple(test_type) => {
                match test_type.as_str() {
                    // TODO Should these be enums?
                    "not_null" => Ok(ColumnTest {
                        r#type: "not_null".to_string(),
                        info: Default::default(),
                    }),
                    "unique" => Ok(ColumnTest {
                        r#type: "unique".to_string(),
                        info: Default::default(),
                    }),
                    test => Err(format!("Unknown test type: {}", test)),
                }
            }
            Test::ComplexNotNullConfigSeverity(_) => {
                Err("Unsupported test type: not_null w/ config & severity skipped".to_string())
            }
            Test::ComplexNotNullWhere(_) => {
                Err("Unsupported test type: not_null w/ where skipped".to_string())
            }
            Test::Complex(_) => unimplemented!(),
            Test::Relationship(relationship) => {
                // convert ref('model') to model
                let model = relationship
                    .details
                    .to
                    .replace("ref('", "")
                    .replace("')", "");

                Ok(ColumnTest {
                    r#type: "relationship".to_string(),
                    info: HashMap::from([
                        ("model".to_string(), model),
                        ("column".to_string(), relationship.details.field.clone()),
                    ]),
                })
            }
            Test::AcceptedValuesString(test) => Ok(ColumnTest {
                r#type: "accepted_values".to_string(),
                info: HashMap::from([("values".to_string(), test.details.values.join(","))]),
            }),
            Test::AcceptedValuesNumber(test) => Ok(ColumnTest {
                r#type: "accepted_values".to_string(),
                info: HashMap::from([(
                    "values".to_string(),
                    test.details
                        .values
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                )]),
            }),
            Test::ExpectColumnMeanToBeBetween(_) => Err(
                "Unsupported test type: dbt_expectations.expect_column_mean_to_be_between skipped"
                    .to_string(),
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ExpectColumnMeanToBeBetweenTest {
    #[serde(rename = "dbt_expectations.expect_column_mean_to_be_between")]
    details: ExpectColumnMeanToBeBetweenDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExpectColumnMeanToBeBetweenDetails {
    min_value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct AcceptedValuesTestString {
    #[serde(rename = "accepted_values")]
    details: AcceptValuesDetailsString,
}

#[derive(Debug, Serialize, Deserialize)]
struct AcceptValuesDetailsString {
    #[serde(rename = "values")]
    values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AcceptedValuesTestNumber {
    #[serde(rename = "accepted_values")]
    details: AcceptValuesDetailsNumber,
}

#[derive(Debug, Serialize, Deserialize)]
struct AcceptValuesDetailsNumber {
    #[serde(rename = "values")]
    values: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelationshipTest {
    #[serde(rename = "relationships")]
    details: RelationshipDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelationshipDetails {
    to: String,
    field: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComplexNotNullConfigSeverity {
    not_null: ComplexNotNullConfigSeverityDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComplexNotNullConfigSeverityDetails {
    config: ComplexNotNullConfigSeverityDetailsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComplexNotNullConfigSeverityDetailsConfig {
    severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComplexNotNullWhere {
    not_null: ComplexNotNullWhereDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComplexNotNullWhereDetails {
    #[serde(rename = "where")]
    details: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_complex_not_null_severity_directly() {
        let yaml = r#"
    not_null:
      config:
        severity: warn
    "#;
        let deserialized: Result<ComplexNotNullConfigSeverity, _> = serde_yaml::from_str(yaml);
        assert!(deserialized.is_ok());
    }
    #[test]
    fn deserialize_complex_not_null_where_directly() {
        let yaml = r#"
    not_null:
        where: "raw_deal_date >= '2023-11-24'"
    "#;
        let deserialized: Result<ComplexNotNullWhere, _> = serde_yaml::from_str(yaml);
        assert!(deserialized.is_ok());
    }
}
