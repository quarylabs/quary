use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_cli_in_memory() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Define the sequence of command arguments
    let commands = vec![
        vec!["init"],
        vec!["compile"],
        vec!["build"],
        vec!["build", "-c"],
        vec!["test", "-d", "-s"],
        vec!["test", "-s"],
    ];

    // Execute each command in the sequence
    for args in commands {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(args)
            .assert()
            .success();
    }
}

#[test]
fn test_rust_run_sqlite() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Initialise project
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["init"])
        .assert()
        .success();

    // Add file
    let sqlite_path_yaml_file = "
sqlite:
  path: db.sqlite";
    let path_location = "sqlite-path.quary.yaml";
    std::fs::write(project_dir.join(path_location), sqlite_path_yaml_file).unwrap();

    // Run a series of commands
    let project_arg = "-p=sqlite-path.quary.yaml"; // Common argument
    let commands = vec![
        vec!["compile", project_arg],
        vec!["build", project_arg],
        vec!["build", project_arg],
        vec!["test", project_arg],
        vec!["build", "-c", project_arg],
        vec!["build", "-c", project_arg],
        vec!["test", project_arg],
        vec!["test", "-d", project_arg],
        vec!["test", project_arg, "-m=skip"],
    ];

    for args in commands {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(args)
            .assert()
            .success();
    }
}

#[test]
fn test_rust_run_duckdb_in_memory() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Initialise project
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["init", "-t=duckdb"])
        .assert()
        .success();

    // Run a series of commands
    let commands = vec![vec!["compile"], vec!["build"], vec!["test", "-s"]];

    for args in commands {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(args)
            .assert()
            .success();
    }
}

#[test]
fn test_rust_run_duckdb_in_memory_with_schema() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Initialise project
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["init", "-t=duckdb"])
        .assert()
        .success();

    // Override the quary.yaml path
    let sqlite_path_yaml_file = "
duckdbInMemory:
  schema: transform";
    let path_location = "quary.yaml";
    std::fs::write(project_dir.join(path_location), sqlite_path_yaml_file).unwrap();

    // Run a series of commands
    let commands = vec![vec!["compile"], vec!["build"], vec!["test", "-s"]];

    for args in commands {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(args)
            .assert()
            .success();
    }
}

#[test]
fn test_rust_run_duckdb_with_path() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Initialise project
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["init", "-t=duckdb"])
        .assert()
        .success();

    // Override the quary.yaml path
    let sqlite_path_yaml_file = "
duckdb:
  path: db.sqlite";
    let path_location = "quary.yaml";
    std::fs::write(project_dir.join(path_location), sqlite_path_yaml_file).unwrap();

    // Run a series of commands
    let commands = vec![
        vec!["compile"],
        vec!["test", "-s"],
        vec!["build"],
        vec!["test"],
    ];

    for args in commands {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(args)
            .assert()
            .success();
    }
}

#[test]
fn test_rust_run_duckdb_with_schema_with_schema() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Initialise project
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["init", "-t=duckdb"])
        .assert()
        .success();

    // Override the quary.yaml path
    let sqlite_path_yaml_file = "
duckdb:
  path: db.sqlite
  schema: transform
";

    let path_location = "quary.yaml";
    std::fs::write(project_dir.join(path_location), sqlite_path_yaml_file).unwrap();

    // Run a series of commands
    let commands = vec![
        vec!["compile"],
        vec!["test", "-s"],
        vec!["build"],
        vec!["build", "-c"],
        vec!["test"],
    ];

    for args in commands {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(args)
            .assert()
            .success();
    }
}
