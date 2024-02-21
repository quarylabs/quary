use crate::config_file::ConfigFile;
use crate::stats::ConvertDbtStats;
use std::io::Cursor;
use std::path::Path;

pub fn parse_dbt_project(
    readable_directory: &Path,
    directory_to_write: &Path,
) -> Result<ConvertDbtStats, String> {
    let mut stats = ConvertDbtStats::default();
    // parse project file
    // TODO Actually parse project_file
    let file_content = std::fs::read_to_string(readable_directory.join("dbt_project.yml"))
        .map_err(|e| format!("Error reading dbt_project.yml: {}", e))?;
    let file_content = Cursor::new(file_content);
    let config_file = ConfigFile::from_yaml(file_content)
        .map_err(|e| format!("Error parsing dbt_project.yml: {}", e))?;

    // write temp project file
    const OUTPUT: &str = "# Please set up the Quary config file.";
    std::fs::write(directory_to_write.join("quary.yaml"), OUTPUT)
        .map_err(|e| format!("Error writing quary.yaml: {}", e))?;

    // create seeds folder
    std::fs::create_dir_all(directory_to_write.join("seeds"))
        .map_err(|e| format!("Error creating seeds directory: {}", e))?;
    // look at seeds
    match &config_file.seeds_paths[..] {
        [] => Ok(()),
        // single entry
        [seed_path] => {
            // read all files in the path recursively, look for CSV files and copy them into the output directory
            let seed_path = readable_directory.join(seed_path);
            for entry in walkdir::WalkDir::new(&seed_path) {
                let entry = entry.map_err(|e| format!("Error reading seed directory: {}", e))?;
                if entry.file_type().is_file() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".csv") {
                        // write file to output director in /seeds/ and same path
                        let output_path = directory_to_write
                            .join("seeds")
                            .join(entry.path().strip_prefix(&seed_path).unwrap());
                        std::fs::copy(entry.path(), &output_path).map_err(|error| {
                            format!(
                                "Error copying seed file {} to {}: {}",
                                entry.path().display(),
                                output_path.display(),
                                error
                            )
                        })?;
                    }
                }
            }
            Ok(())
        }
        // multiple entries
        multiple_entries => Err(format!(
            "Multiple seeds paths not supported yet: {:?}",
            multiple_entries
        )),
    }?;

    // TODO ADD A TEST TO THE FOLDER

    // create tests folder
    std::fs::create_dir_all(directory_to_write.join("tests"))
        .map_err(|e| format!("Error creating seeds directory: {}", e))?;
    // look at tests
    match &config_file.tests_paths[..] {
        [] => Ok(()),
        // single entry
        [test] => {
            let test_path = readable_directory.join(test);
            // if test folder exists
            if test_path.exists() {
                // read all files in the path recursively, look for SQL files and copy them into the output directory
                for entry in walkdir::WalkDir::new(&test_path) {
                    let entry =
                        entry.map_err(|e| format!("Error reading test directory: {}", e))?;
                    if entry.file_type().is_file() {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name.ends_with(".sql") {
                            // write file to output director in /tests/ and same path
                            let output_path = directory_to_write
                                .join("tests")
                                .join(entry.path().strip_prefix(&test_path).unwrap());
                            std::fs::copy(entry.path(), output_path)
                                .map_err(|e| format!("Error copying test file: {}", e))?;
                            stats.file_tests_created += 1;
                        }
                    }
                }
            }
            Ok(())
        }
        _ => Err("Multiple test paths not supported yet".to_string()),
    }?;

    // look at models moving sql files
    match &config_file.model_paths[..] {
        [] => Ok(()),
        // single entry
        [models] => {
            // read all files in the path recursively, look for SQL files and copy them into the output directory
            let models_path = readable_directory.join(models);
            for entry in walkdir::WalkDir::new(models_path.clone()) {
                let entry = entry.map_err(|e| format!("Error reading models directory: {}", e))?;
                if entry.file_type().is_file() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".sql") {
                        // write file to output director in /models/ and same path
                        let output_path = directory_to_write
                            .join("models")
                            .join(entry.path().strip_prefix(&models_path).unwrap());
                        // read entry, pass it through translate_sql_file and paste
                        let reader =
                            Cursor::new(std::fs::read_to_string(entry.path()).map_err(|e| {
                                let error_message = format!("Error reading model file: {}", e);
                                stats.add_error(error_message.clone());
                                error_message
                            })?);
                        let translated = crate::sql_file::translate_sql_file(reader)?;
                        // create file and folder structure if missing
                        std::fs::create_dir_all(output_path.parent().unwrap())
                            .map_err(|e| format!("Error creating model directory: {}", e))?;
                        // write to file
                        std::fs::write(&output_path, translated).map_err(|e| {
                            format!("Error writing model file {}: {}", file_name, e)
                        })?;
                        stats.models_created += 1;
                    }
                }
            }
            Ok(())
        }
        _ => Err("Multiple models paths not supported yet".to_string()),
    }?;

    // look at project files
    match &config_file.model_paths[..] {
        [] => Ok(()),
        // single entry
        [models] => {
            // read all files in the path recursively, look for SQL files and copy them into the output directory
            let models_path = readable_directory.join(models);
            for entry in walkdir::WalkDir::new(models_path.clone()) {
                let entry = entry.map_err(|e| format!("Error reading models directory: {}", e))?;
                if entry.file_type().is_file() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".yml") {
                        // write file to output director in /models/ and same path
                        let output_path = directory_to_write
                            .join("models")
                            .join(entry.path().strip_prefix(&models_path.clone()).unwrap());
                        // read entry, pass it through translate_sql_file and paste
                        let reader = Cursor::new(
                            std::fs::read_to_string(entry.path())
                                .map_err(|e| format!("Error reading model file: {}", e))?,
                        );
                        let project_file = crate::project_file::ProjectFile::from_yaml(reader)
                            .map_err(|e| {
                                format!(
                                    "Error parsing model file {}: {}",
                                    entry.path().display(),
                                    e
                                )
                            })?;
                        let translated = project_file.to_quary(&mut stats)?;

                        // write yaml version
                        // change the output path from .yml to .yaml
                        let output_path = output_path.with_extension("yaml");
                        let file = std::fs::File::create(output_path)
                            .map_err(|e| format!("Error creating model file: {}", e))?;
                        let mut writer = std::io::BufWriter::new(file);
                        serde_yaml::to_writer(&mut writer, &translated)
                            .map_err(|e| format!("Error writing project file: {}", e))?;
                    }
                }
            }
            Ok(())
        }
        _ => Err("Multiple models paths not supported yet".to_string()),
    }?;

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // TODO Implement sources

    /// test_parse_dbt_project reads the input file, writes the output to a temporary directory and compares the output
    /// to the expected output which is in the output directory
    #[test]
    fn test_parse_dbt_project() {
        let write_dir = TempDir::new().unwrap();

        let directory_to_write = write_dir.path().to_path_buf();
        // current directory is rust/dbt-converter
        let crate_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let input_directory = crate_directory.join("src/input");
        let expected_output = crate_directory.join("src/output");

        parse_dbt_project(&input_directory, &directory_to_write).unwrap();

        // compare the files in the output directory to the expected output
        let mut expected_files = walkdir::WalkDir::new(&expected_output)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                e.path()
                    .strip_prefix(&expected_output)
                    .unwrap()
                    .to_path_buf()
            })
            .collect::<Vec<_>>();
        expected_files.sort();

        let mut actual_files = walkdir::WalkDir::new(&directory_to_write)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                e.path()
                    .strip_prefix(&directory_to_write)
                    .unwrap()
                    .to_path_buf()
            })
            .collect::<Vec<_>>();
        actual_files.sort();

        assert_eq!(expected_files.len(), actual_files.len());
        assert_eq!(
            expected_files.clone().iter().collect::<BTreeSet<_>>(),
            actual_files.clone().iter().collect::<BTreeSet<_>>(),
        );

        // compare the content of the files
        for file in expected_files {
            match file.extension().unwrap().to_str().unwrap() {
                "sql" => {
                    let expected_content =
                        std::fs::read_to_string(expected_output.join(&file)).unwrap();
                    let actual_content =
                        std::fs::read_to_string(directory_to_write.join(&file)).unwrap();
                    assert_eq!(expected_content, actual_content);
                }
                "yaml" => {
                    let expected = std::fs::read_to_string(expected_output.join(&file)).unwrap();
                    let actual_content =
                        std::fs::read_to_string(directory_to_write.join(&file)).unwrap();

                    // Parse the YAML contents into Rust data structures
                    let expected: serde_yaml::Value = serde_yaml::from_str(&expected).unwrap();
                    let actual_content: serde_yaml::Value =
                        serde_yaml::from_str(&actual_content).unwrap();

                    assert_eq!(expected, actual_content);
                }
                "csv" => {
                    let expected_content =
                        std::fs::read_to_string(expected_output.join(&file)).unwrap();
                    let actual_content =
                        std::fs::read_to_string(directory_to_write.join(&file)).unwrap();
                    assert_eq!(expected_content, actual_content);
                }
                _ => panic!("Unexpected file extension: {}", file.display()),
            }
        }
    }
}
