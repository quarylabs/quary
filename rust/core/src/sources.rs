use quary_proto::DatabaseSource;
use quary_proto::ProjectFile;
use std::collections::HashMap;

fn create_project_file_source(
    source: &DatabaseSource,
    name_prefix: Option<&str>,
) -> quary_proto::ProjectFileSource {
    quary_proto::ProjectFileSource {
        name: match name_prefix {
            Some(prefix) => format!("{}{}", prefix, source.name),
            None => source.name.to_string(),
        },
        description: None,
        tags: vec![],
        path: source.path.to_string(),
        tests: vec![],
        columns: source
            .columns
            .iter()
            .map(|column| quary_proto::ProjectFileColumn {
                name: column.to_string(),
                description: None,
                tests: vec![],
            })
            .collect(),
    }
}

/// Converts a list of sources to a `ProjectFile (sources only)`.
pub fn sources_to_project_file(sources: &[DatabaseSource]) -> ProjectFile {
    ProjectFile {
        sources: sources
            .iter()
            .map(|source| create_project_file_source(source, None))
            .collect(),
        ..ProjectFile::default()
    }
}

/// Builds a complete staging schema `ProjectFile` (sources + models) from sources.
pub fn build_staging_schema_file_from_sources(
    project_file: ProjectFile,
    sources: &[DatabaseSource],
) -> ProjectFile {
    ProjectFile {
        sources: project_file
            .sources
            .into_iter()
            .chain(
                sources
                    .iter()
                    .map(|source| create_project_file_source(source, Some("raw_"))),
            )
            .collect(),
        models: project_file
            .models
            .into_iter()
            .chain(sources.iter().map(|source| {
                quary_proto::project_file::Model {
                    name: format!("stg_{}", source.name),
                    description: Some(format!(
                        "Replace this with your description for {}",
                        source.name
                    )),
                    materialization: None,
                    tags: vec![],
                    tests: vec![],
                    columns: source
                        .columns
                        .clone()
                        .into_iter()
                        .map(|column| quary_proto::ProjectFileColumn {
                            name: column,
                            description: None,
                            tests: vec![],
                        })
                        .collect(),
                    indexes: vec![]
                }
            }))
            .collect(),
        snapshots: vec![],
    }
}

/// Builds boilerplate SQL select statements for each source.
pub fn build_models_from_sources(
    folder_path: &str,
    sources: &[DatabaseSource],
) -> HashMap<String, String> {
    sources
        .iter()
        .map(|source| {
            let file_name = format!("{}/stg_{}.sql", folder_path, source.name);
            let select_statement = create_staging_model_sql_for_source(source);
            (file_name, select_statement)
        })
        .collect()
}

pub(crate) fn create_staging_model_sql_for_source(source: &DatabaseSource) -> String {
    let columns: Vec<String> = source
        .columns
        .iter()
        .map(|column| format!("    {} AS {}", column, column))
        .collect();
    format!(
        "SELECT\n{}\nFROM q.raw_{}",
        columns.join(",\n"),
        source.name
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use quary_proto::{project_file, ProjectFileColumn, ProjectFileSource};

    fn create_test_source(name: &str, path: &str, columns: Vec<&str>) -> DatabaseSource {
        DatabaseSource {
            name: name.to_string(),
            path: path.to_string(),
            columns: columns.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_create_project_file_source_without_prefix() {
        let source = create_test_source("test_table", "database.test_table", vec!["col1", "col2"]);
        let project_file_source = create_project_file_source(&source, None);
        assert_eq!(project_file_source.name, source.name);
        assert_eq!(project_file_source.path, source.path);
    }

    #[test]
    fn test_create_project_file_source_with_prefix() {
        let source = create_test_source("test_table", "database.test_table", vec!["col1", "col2"]);
        let project_file_source = create_project_file_source(&source, Some("raw_"));
        assert_eq!(project_file_source.name, "raw_test_table");
    }

    #[test]
    fn test_sources_to_project_file() {
        let sources = vec![
            create_test_source("test_table", "database.test_table", vec!["col1", "col2"]),
            create_test_source("demo_table", "database.demo_table", vec!["col1", "col2"]),
        ];
        let project_file = sources_to_project_file(&sources);
        assert_eq!(project_file.sources.len(), 2);
    }

    #[test]
    fn test_build_staging_schema_file_from_sources_with_empty_file() {
        let sources = vec![
            create_test_source("test_table", "database.test_table", vec!["col1", "col2"]),
            create_test_source("demo_table", "database.demo_table", vec!["col1", "col2"]),
        ];
        let project_file = build_staging_schema_file_from_sources(Default::default(), &sources);

        assert_eq!(
            project_file,
            ProjectFile {
                sources: vec![
                    ProjectFileSource {
                        name: "raw_test_table".to_string(),
                        description: None,
                        path: "database.test_table".to_string(),
                        tags: vec![],
                        tests: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ]
                    },
                    ProjectFileSource {
                        name: "raw_demo_table".to_string(),
                        description: None,
                        path: "database.demo_table".to_string(),
                        tests: vec![],
                        tags: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ]
                    }
                ],
                models: vec![
                    project_file::Model {
                        name: "stg_test_table".to_string(),
                        description: Some(
                            "Replace this with your description for test_table".to_string()
                        ),
                        materialization: None,
                        tags: vec![],
                        tests: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ],
                        indexes: vec![]
                    },
                    project_file::Model {
                        name: "stg_demo_table".to_string(),
                        description: Some(
                            "Replace this with your description for demo_table".to_string()
                        ),
                        materialization: None,
                        tags: vec![],
                        tests: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ],
                        indexes: vec![]
                    }
                ],
                snapshots: vec![],
            }
        )
    }

    #[test]
    fn test_build_staging_schema_file_from_sources_with_preexisting_project_file() {
        let sources = vec![
            create_test_source("test_table", "database.test_table", vec!["col1", "col2"]),
            create_test_source("demo_table", "database.demo_table", vec!["col1", "col2"]),
        ];
        let project_file = build_staging_schema_file_from_sources(
            ProjectFile {
                sources: vec![ProjectFileSource {
                    name: "test_123".to_string(),
                    description: None,
                    path: "".to_string(),
                    tags: vec![],
                    tests: vec![],
                    columns: vec![],
                }],
                models: vec![project_file::Model {
                    name: "test_456".to_string(),
                    description: None,
                    tags: vec![],
                    tests: vec![],
                    materialization: None,
                    columns: vec![],
                    indexes: vec![],
                }],
                snapshots: vec![],
            },
            &sources,
        );

        assert_eq!(
            project_file,
            ProjectFile {
                sources: vec![
                    ProjectFileSource {
                        name: "test_123".to_string(),
                        description: None,
                        path: "".to_string(),
                        tags: vec![],
                        tests: vec![],
                        columns: vec![]
                    },
                    ProjectFileSource {
                        name: "raw_test_table".to_string(),
                        description: None,
                        path: "database.test_table".to_string(),
                        tags: vec![],
                        tests: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ]
                    },
                    ProjectFileSource {
                        name: "raw_demo_table".to_string(),
                        description: None,
                        path: "database.demo_table".to_string(),
                        tags: vec![],
                        tests: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ]
                    }
                ],
                models: vec![
                    project_file::Model {
                        name: "test_456".to_string(),
                        description: None,
                        materialization: None,
                        tags: vec![],
                        tests: vec![],
                        columns: vec![],
                        indexes: vec![],
                    },
                    project_file::Model {
                        name: "stg_test_table".to_string(),
                        description: Some(
                            "Replace this with your description for test_table".to_string()
                        ),
                        materialization: None,
                        tags: vec![],
                        tests: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ],
                        indexes: vec![]
                    },
                    project_file::Model {
                        name: "stg_demo_table".to_string(),
                        description: Some(
                            "Replace this with your description for demo_table".to_string()
                        ),
                        materialization: None,
                        tags: vec![],
                        tests: vec![],
                        columns: vec![
                            ProjectFileColumn {
                                name: "col1".to_string(),
                                description: None,
                                tests: vec![]
                            },
                            ProjectFileColumn {
                                name: "col2".to_string(),
                                description: None,
                                tests: vec![]
                            }
                        ],
                        indexes: vec![]
                    }
                ],
                snapshots: vec![],
            }
        )
    }

    #[test]
    fn test_create_staging_model_sql_for_source() {
        let source = DatabaseSource {
            name: "test_table".to_string(),
            path: "database.test_table".to_string(),
            columns: vec!["col1", "col2"].into_iter().map(String::from).collect(),
        };

        let expected_sql: &str =
            "SELECT\n    col1 AS col1,\n    col2 AS col2\nFROM q.raw_test_table";
        let sql = create_staging_model_sql_for_source(&source);
        assert_eq!(sql, expected_sql);
    }
    #[test]
    fn test_create_staging_model_sql_for_sources() {
        let sources = vec![
            create_test_source("table1", "database.table1", vec!["col1", "col2"]),
            create_test_source("table2", "database.table2", vec!["col3", "col4"]),
        ];
        let folder_path = "models/staging";
        let model_files = build_models_from_sources(folder_path, &sources);

        assert_eq!(model_files.len(), 2);
        assert!(model_files.contains_key("models/staging/stg_table1.sql"));
        assert!(model_files.contains_key("models/staging/stg_table2.sql"));

        let expected_sql_table1: &str =
            "SELECT\n    col1 AS col1,\n    col2 AS col2\nFROM q.raw_table1";
        let expected_sql_table2 = "SELECT\n    col3 AS col3,\n    col4 AS col4\nFROM q.raw_table2";
        assert_eq!(
            model_files.get("models/staging/stg_table1.sql").unwrap(),
            &expected_sql_table1.to_string()
        );
        assert_eq!(
            model_files.get("models/staging/stg_table2.sql").unwrap(),
            &expected_sql_table2.to_string()
        );
    }
}
