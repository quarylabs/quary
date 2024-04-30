use crate::file_system::{convert_async_read_to_blocking_read, FileSystem};
use quary_proto::ConnectionConfig;
use std::io;
use std::path::Path;

pub fn deserialize_config_from_yaml(read: impl io::Read) -> Result<ConnectionConfig, String> {
    serde_yaml::from_reader(read).map_err(|e| format!("reading yaml: {}", e))
}

pub fn serialize_config_to_yaml(config: &ConnectionConfig) -> Result<String, String> {
    serde_yaml::to_string(config).map_err(|e| format!("writing yaml: {}", e))
}

/// get_config_from_filesystem reads the config file from the filesystem and returns it if it is
/// present. It looks for the config file in the current working director for the file:
/// `quary.yaml`. Otherwise it returns an error.
///
/// The path of the config file can be overridden by specifying a path.
pub async fn get_config_from_filesystem(
    file_system: &impl FileSystem,
    project_root: &str,
) -> Result<ConnectionConfig, String> {
    let path = Path::new(project_root).join(DEFAULT_CONFIG_PATH);

    let file = file_system
        .read_file(path.to_str().ok_or("Invalid path")?)
        .await
        .map_err(|e| format!("opening file: {}", e))?;

    let file = convert_async_read_to_blocking_read(file).await;
    deserialize_config_from_yaml(file)
}

pub const DEFAULT_CONFIG_PATH: &str = "quary.yaml";

#[cfg(test)]
mod tests {
    use super::*;
    use prost::bytes::Bytes;
    use quary_proto::{connection_config, Var};
    use std::{collections::HashMap, io::Cursor};

    #[tokio::test]
    async fn test_get_config_from_filesystem_valid_config() {
        let fs = quary_proto::FileSystem {
            files: HashMap::from([(
                "quary.yaml".to_string(),
                quary_proto::File {
                    name: "quary.yaml".to_string(),
                    contents: Bytes::from(
                        r#"
        bigQuery:
          projectId: "test-project"
          datasetId: "test_dataset"
        vars:
          - name: test
            value: test
        "#
                        .as_bytes(),
                    ),
                },
            )]),
        };

        let config = get_config_from_filesystem(&fs, "").await.unwrap();

        let expected_config = ConnectionConfig {
            config: Some(connection_config::Config::BigQuery(
                connection_config::ConnectionConfigBigQuery {
                    project_id: "test-project".to_string(),
                    dataset_id: "test_dataset".to_string(),
                },
            )),
            vars: vec![Var {
                name: "test".to_string(),
                value: "test".to_string(),
            }],
        };

        assert_eq!(config, expected_config);
    }

    #[tokio::test]
    async fn test_get_config_from_filesystem_with_prefix() {
        let fs = quary_proto::FileSystem {
            files: HashMap::from([(
                "./root_folder/quary.yaml".to_string(),
                quary_proto::File {
                    name: "quary.yaml".to_string(),
                    contents: Bytes::from(
                        r#"
                bigQuery:
                    projectId: "test-project"
                    datasetId: "test_dataset"
            "#
                        .as_bytes(),
                    ),
                },
            )]),
        };

        let config = get_config_from_filesystem(&fs, "./root_folder/")
            .await
            .unwrap();

        let expected_config = ConnectionConfig {
            config: Some(connection_config::Config::BigQuery(
                connection_config::ConnectionConfigBigQuery {
                    project_id: "test-project".to_string(),
                    dataset_id: "test_dataset".to_string(),
                },
            )),
            vars: Vec::new(),
        };

        assert_eq!(config, expected_config);
    }

    #[tokio::test]
    async fn test_get_config_from_filesystem_invalid_path() {
        let fs: quary_proto::FileSystem = quary_proto::FileSystem {
            files: HashMap::from([(
                "./root_folder/quary.yaml".to_string(),
                quary_proto::File {
                    name: "quary.yaml".to_string(),
                    contents: Bytes::from(
                        r#"
                bigQuery:
                    projectId: "test-project"
                    datasetId: "test_dataset"
            "#
                        .as_bytes(),
                    ),
                },
            )]),
        };
        let result = get_config_from_filesystem(&fs, ".").await;

        assert!(result.is_err(), "Expected an error for invalid file path");
    }

    #[tokio::test]
    async fn test_get_config_from_filesystem_missing_file() {
        let fs: quary_proto::FileSystem = Default::default();
        let result = get_config_from_filesystem(&fs, ".").await;
        assert!(
            result.is_err(),
            "Expected an error for missing configuration file"
        );
    }

    #[test]
    fn test_deserialize_invalid_yaml_config() {
        let invalid_yaml_str = "invalid yaml";
        let result = deserialize_config_from_yaml(Cursor::new(invalid_yaml_str));
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_bigquery_connection_config() {
        let yaml_str = r#"
            bigQuery:
                projectId: "test-project"
                datasetId: "test_dataset"
        "#;

        let deserialized_config = deserialize_config_from_yaml(Cursor::new(yaml_str)).unwrap();

        let expected_big_query_config = connection_config::ConnectionConfigBigQuery {
            project_id: "test-project".to_string(),
            dataset_id: "test_dataset".to_string(),
        };

        let expected_config = ConnectionConfig {
            config: Some(connection_config::Config::BigQuery(
                expected_big_query_config,
            )),
            vars: Vec::new(),
        };

        assert_eq!(deserialized_config, expected_config);
    }

    #[test]
    fn test_serialize_bigquery_connection_config() {
        let big_query_config = connection_config::ConnectionConfigBigQuery {
            project_id: "test_project".to_string(),
            dataset_id: "test_dataset".to_string(),
        };
        let config = ConnectionConfig {
            config: Some(connection_config::Config::BigQuery(big_query_config)),
            vars: Vec::new(),
        };
        // Serialize the ConnectionConfig instance to a YAML string
        let yaml_str = serialize_config_to_yaml(&config).unwrap();

        // Deserialize the YAML string back into a ConnectionConfig instance
        let deserialized_config = deserialize_config_from_yaml(Cursor::new(yaml_str)).unwrap();

        // Assert that the original and deserialized objects are equal
        assert_eq!(config, deserialized_config,);
    }
}
