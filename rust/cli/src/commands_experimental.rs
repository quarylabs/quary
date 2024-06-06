#![allow(unwrap_used)]
use crate::commands::ExperimentalCommands;
use crate::commands::ExternalCommands;
use crate::{get_config_file, parse_project};
use quary_core::project::parse_project_files;
use quary_databases::databases_connection::database_from_config;
use quary_proto::ProjectFile;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

pub(crate) async fn experimental_commands(
    args: &ExternalCommands,
    project_file: &str,
) -> Result<(), String> {
    match args.command {
        ExperimentalCommands::SeperateProjectFiles => {
            let config = get_config_file(project_file)?;
            let database = database_from_config(&config).await?;
            let query_generator = database.query_generator();
            let (project, file_system) = parse_project(&query_generator).await?;

            let project_files = parse_project_files(&file_system, "", &query_generator).await?;

            // delete all the files
            project_files
                .iter()
                .map(|(path, _)| std::fs::remove_file(path))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;

            // sources go into a _sources.yaml file
            let grouped_by_folder = project_files
                .iter()
                .fold(
                    HashMap::<String, Vec<ProjectFile>>::new(),
                    |mut acc: HashMap<String, Vec<ProjectFile>>, (path, file)| {
                        let (folder, _) = path.rsplit_once("/").unwrap();
                        let entry = acc.entry(folder.to_string()).or_default();
                        entry.push(file.clone());
                        acc
                    },
                )
                .into_iter()
                .filter_map(|(folder, files)| {
                    let folder = Path::new(&folder);
                    let project_root = Path::new("");
                    let file = "_sources.yaml";
                    let file_path = project_root.join(folder).join(file);

                    let sources = files
                        .iter()
                        .flat_map(|file| {
                            file.sources
                                .iter()
                                .map(|source| (source.name.clone(), source.clone()))
                        })
                        .collect::<BTreeMap<_, _>>();
                    let project_file = ProjectFile {
                        sources: sources.iter().map(|(_, source)| source.clone()).collect(),
                        ..Default::default()
                    };
                    if project_file.sources.is_empty() {
                        return None;
                    }
                    Some((file_path, project_file))
                })
                .collect::<Vec<_>>();

            grouped_by_folder.iter().for_each(|(path, project_file)| {
                let file = std::fs::File::create(path).unwrap();
                serde_yaml::to_writer(file, &project_file).unwrap();
            });

            // models go into individually named model files
            let model_files = project
                .models
                .iter()
                .filter_map(|model| {
                    let found_project_file_entry = project_files
                        .iter()
                        .find(|(_, file)| file.models.iter().any(|m| m.name == *model.0));
                    if let Some((_, project_file)) = found_project_file_entry {
                        let project_file_model = project_file
                            .models
                            .iter()
                            .find(|m| m.name == *model.0)
                            .unwrap();
                        let file_name = model.1.file_path.clone();
                        let file_path = Path::new("").join(file_name);
                        let file_path = file_path.with_extension("yaml");
                        let project_file = ProjectFile {
                            models: vec![project_file_model.clone()],
                            ..Default::default()
                        };
                        Some((file_path, project_file))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(PathBuf, ProjectFile)>>();
            model_files.iter().for_each(|(path, project_file)| {
                let file = std::fs::File::create(path).unwrap();
                serde_yaml::to_writer(file, &project_file).unwrap();
            });

            // snapshots go into individually named snapshots
            let snapshot_files = project
                .snapshots
                .iter()
                .filter(|snapshot| {
                    let found_project_file_entry = project_files
                        .iter()
                        .find(|(_, file)| file.snapshots.iter().any(|s| s.name == *snapshot.0));
                    found_project_file_entry.is_some()
                })
                .map(|snapshot| {
                    let found_project_file_entry = project_files
                        .iter()
                        .find(|(_, file)| file.snapshots.iter().any(|s| s.name == *snapshot.0))
                        .unwrap();
                    let project_file_snapshot = found_project_file_entry
                        .1
                        .snapshots
                        .iter()
                        .find(|s| s.name == *snapshot.0)
                        .unwrap();
                    let file_name = snapshot.1.file_path.clone();
                    let file_path = Path::new("").join(file_name);
                    let file_path = file_path.with_extension("snapshot.yaml");
                    let project_file = ProjectFile {
                        snapshots: vec![project_file_snapshot.clone()],
                        ..Default::default()
                    };
                    Ok((file_path, project_file))
                })
                .collect::<Result<Vec<(PathBuf, ProjectFile)>, String>>()?;

            snapshot_files.iter().for_each(|(path, project_file)| {
                let file = std::fs::File::create(path).unwrap();
                serde_yaml::to_writer(file, &project_file).unwrap();
            });

            println!("Seperated project files into individual files.");
            Ok(())
        }
    }
}
