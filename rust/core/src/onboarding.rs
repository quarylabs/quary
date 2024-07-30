use crate::config::serialize_config_to_yaml;
use crate::file_system::{convert_async_read_to_blocking_read, FileSystem};
use crate::init::Asset;
use crate::project_file::{deserialize_project_file_from_yaml, serialize_project_file_to_yaml};
use crate::sources::{build_models_from_sources, build_staging_schema_file_from_sources};
use quary_proto::{ConnectionConfig, DatabaseSource, ProjectFile};
use rust_embed::RustEmbed;
use std::path::Path;
use std::string::FromUtf8Error;

#[derive(RustEmbed)]
#[folder = "./src/template_files"]
struct TemplateFiles;

pub async fn generate_onboarding_files(
    connection_config: ConnectionConfig,
) -> Result<impl Iterator<Item = (String, String)>, String> {
    let tests_placeholder_folder = ("tests/.gitkeep".to_string(), "".to_string());
    let models_placeholder_folder = ("models/.gitkeep".to_string(), "".to_string());
    let readme_asset =
        TemplateFiles::get("README.md").ok_or("README file not found".to_string())?;

    let readme_contents =
        String::from_utf8(readme_asset.data.to_vec()).map_err(|e: FromUtf8Error| e.to_string())?;
    let readme_file = ("README.md".to_string(), readme_contents);

    let query_config_file_yaml = serialize_config_to_yaml(&connection_config)
        .map_err(|e| format!("Error serializing quary.yaml file: {}", e))?;
    let project_file = ("quary.yaml".to_string(), query_config_file_yaml);

    let gitignore_contents = String::from_utf8(
        Asset::get(".gitignore")
            .ok_or("gitignore file not found".to_string())?
            .data
            .to_vec(),
    )
    .map_err(|e: FromUtf8Error| e.to_string())?;
    let gitignore_file = (".gitignore".to_string(), gitignore_contents);

    let sqlfluff_contents = String::from_utf8(
        Asset::get(".sqlfluff")
            .ok_or("gitignore file not found".to_string())?
            .data
            .to_vec(),
    )
    .map_err(|e: FromUtf8Error| e.to_string())?;
    let sqlfluff_file = (".sqlfluff".to_string(), sqlfluff_contents);

    let vscode_file_definitions = Asset
        .list_all_files_recursively(".vscode")
        .await?
        .into_iter()
        .map(|path| {
            let contents = String::from_utf8(
                Asset::get(&path)
                    .ok_or(format!("{} file not found", path))?
                    .data
                    .to_vec(),
            )
            .map_err(|e: FromUtf8Error| e.to_string())?;
            Ok((path, contents))
        })
        .collect::<Result<Vec<(String, String)>, String>>()?;

    let github_file_definitions = Asset
        .list_all_files_recursively(".github")
        .await?
        .into_iter()
        .map(|path| {
            let contents = String::from_utf8(
                Asset::get(&path)
                    .ok_or(format!("{} file not found", path))?
                    .data
                    .to_vec(),
            )
            .map_err(|e: FromUtf8Error| e.to_string())?;
            Ok((path, contents))
        })
        .collect::<Result<Vec<(String, String)>, String>>()?;

    Ok(vscode_file_definitions
        .into_iter()
        .chain(github_file_definitions)
        .chain([
            project_file,
            readme_file,
            tests_placeholder_folder,
            models_placeholder_folder,
            gitignore_file,
            sqlfluff_file,
        ]))
}

/// generate_source_files generates the source files for the given sources. It returns an iterator of tuples of the form
/// (file_path, file_contents). The file_path is relative to the root of the project.
///
/// The files generated are:
/// - {folder_path}/schema.yaml: The staging schema file which contains the models and sources
/// - {folder_path}/stg_<model_name>.sql: The model files
///
/// If the schema.yaml file already exists, the data will be appended to it.
pub async fn generate_source_files(
    project_root: &str,
    file_system: &impl FileSystem,
    folder_path: &str,
    sources: &[DatabaseSource],
) -> Result<impl Iterator<Item = (String, String)>, String> {
    let path = Path::new(project_root)
        .join(folder_path)
        .join("schema.yaml");
    let schema_file_path = path.to_str().ok_or("Invalid path")?.to_string();
    let schema_file = match file_system.read_file(&schema_file_path).await {
        Ok(file) => {
            let file = convert_async_read_to_blocking_read(file).await;
            let schema_file = deserialize_project_file_from_yaml(file)
                .map_err(|e| format!("Error deserializing .schema file: {}", e))?;
            Ok(schema_file)
        }
        Err(io_error) => {
            if io_error.kind() == std::io::ErrorKind::NotFound {
                Ok(ProjectFile {
                    models: vec![],
                    sources: vec![],
                    snapshots: vec![],
                })
            } else {
                Err(format!("Error reading .schema file: {}", io_error))
            }
        }
    }?;

    let stg_schema_content = build_staging_schema_file_from_sources(schema_file, sources);
    let stg_schema_yaml = serialize_project_file_to_yaml(stg_schema_content)
        .map_err(|e| format!("Error serializing .schema file: {}", e))?;
    let staging_file = (schema_file_path, stg_schema_yaml);

    let source_model_files = build_models_from_sources(folder_path, sources);

    Ok(source_model_files.into_iter().chain([staging_file]))
}

/// Checks if the directory is empty, excluding
/// - hidden files
/// - .sqlite files
/// - .git folder
/// - .idea folder
/// - .env files
pub async fn is_empty_bar_hidden_and_sqlite(
    file_system: &impl FileSystem,
    root_path: &str,
) -> Result<bool, String> {
    let file_paths = file_system
        .list_all_files_recursively(root_path)
        .await
        .map_err(|e| format!("Error listing files: {}", e))?;

    // Check if any file does not match the criteria
    let is_empty = file_paths.iter().all(|file_path| {
        let path = Path::new(file_path);
        let file_name = path.file_name().and_then(|n| n.to_str());

        let is_hidden = file_name.map(|name| name.starts_with('.')).unwrap_or(false);
        let is_sqlite = path.extension().map(|ext| ext == "sqlite").unwrap_or(false);
        let is_git = file_path.contains(".git/");
        let is_jetbrains = file_path.contains(".idea/");
        let is_env = file_path.ends_with(".env");

        is_hidden || is_sqlite || is_git || is_jetbrains || is_env
    });

    Ok(is_empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::deserialize_config_from_yaml;
    use prost::bytes::Bytes;
    use quary_proto::connection_config::ConnectionConfigSqLiteInMemory;
    use quary_proto::{connection_config, File};
    use std::collections::HashMap;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_generate_onboarding_files() {
        let sqlite_config = ConnectionConfig {
            pre_run_scripts: vec![],
            config: Some(connection_config::Config::SqliteInMemory(
                ConnectionConfigSqLiteInMemory {},
            )),
            vars: Vec::new(),
        };

        let files = generate_onboarding_files(sqlite_config.clone())
            .await
            .unwrap()
            .collect::<HashMap<_, _>>();

        [
            ".vscode/extensions.json",
            ".vscode/settings.json",
            ".github/workflows/pr.yml",
            "quary.yaml",
            ".gitignore",
            ".sqlfluff",
            "README.md",
            "tests/.gitkeep",
            "models/.gitkeep",
        ]
        .iter()
        .for_each(|file_name| {
            assert!(files.contains_key(*file_name));
        });

        let quary_yaml = files.get("quary.yaml").unwrap();
        let deserialized_quary_config: ConnectionConfig =
            deserialize_config_from_yaml(Cursor::new(quary_yaml)).unwrap();
        assert_eq!(sqlite_config, deserialized_quary_config);
    }

    #[tokio::test]
    async fn test_generate_source_files() {
        let file_system: quary_proto::FileSystem = Default::default();

        let sources = vec![DatabaseSource {
            name: "test_table".to_string(),
            path: "project1.dataset1.test_table".to_string(),
            columns: vec!["col1".to_string(), "col2".to_string()],
        }];

        let folder_path = "models/staging";
        let files = generate_source_files("", &file_system, folder_path, &sources)
            .await
            .unwrap()
            .collect::<HashMap<_, _>>();

        let schema_yaml = files.get("models/staging/schema.yaml").unwrap();
        let deserialized_schema: ProjectFile =
            deserialize_project_file_from_yaml(Cursor::new(schema_yaml)).unwrap();
        assert_eq!(deserialized_schema.models.len(), 1);
        assert_eq!(deserialized_schema.sources.len(), 1);
        assert_eq!(deserialized_schema.models[0].name, "stg_test_table");
        assert_eq!(deserialized_schema.sources[0].name, "raw_test_table");
    }

    #[tokio::test]
    async fn is_empty_bar_hidden_and_sqlite_normal_case() {
        let file_system = quary_proto::FileSystem {
            files: HashMap::from([(
                "/quary.yaml".to_string(),
                File {
                    name: "quary.yaml".to_string(),
                    contents: Bytes::from("".as_bytes()),
                },
            )]),
        };

        let result = is_empty_bar_hidden_and_sqlite(&file_system, "")
            .await
            .unwrap();

        assert!(!result);
    }

    #[tokio::test]
    async fn is_empty_bar_hidden_and_sqlite_with_excluded_files() {
        let file_system = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "/.gitignore".to_string(),
                    File {
                        name: ".gitignore".to_string(),
                        contents: Bytes::from("".as_bytes()),
                    },
                ),
                (
                    "/database.sqlite".to_string(),
                    File {
                        name: "database.sqlite".to_string(),
                        contents: Bytes::from("".as_bytes()),
                    },
                ),
                (
                    ".git/HEAD".to_string(),
                    File {
                        name: ".git/HEAD".to_string(),
                        contents: Bytes::from("".as_bytes()),
                    },
                ),
                (
                    ".idea/text.txt".to_string(),
                    File {
                        name: ".idea/text.txt".to_string(),
                        contents: Bytes::from("".as_bytes()),
                    },
                ),
                (
                    "/.env".to_string(),
                    File {
                        name: ".env".to_string(),
                        contents: Bytes::from("".as_bytes()),
                    },
                ),
            ]),
        };

        let result = is_empty_bar_hidden_and_sqlite(&file_system, "")
            .await
            .unwrap();

        assert!(result);
    }
}
