use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_compile_model_does_not_exist() {
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

    // Put a broken file in the project directory
    let broken_file = "SELECT * FROM q.broken;";
    let broken_file_location = project_dir.join("models").join("model_that_is_broken.sql");
    std::fs::write(broken_file_location, broken_file).unwrap();

    // Compile should fail
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["compile"])
        .assert()
        .failure();
}

#[test]
fn test_compile_test_refers_to_nonexistant_model() {
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

    // Put a broken file in the project directory
    let broken_file = "SELECT * FROM q.broken;";
    let broken_file_location = project_dir.join("tests").join("model_that_is_broken.sql");
    std::fs::write(broken_file_location, broken_file).unwrap();

    // Compile should fail
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["compile"])
        .assert()
        .failure();
}

#[test]
fn test_model_is_self_referencing() {
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

    // Put a self-referencing file in the project directory
    let broken_file = "SELECT * FROM q.broken;";
    let broken_file_location = project_dir.join("models").join("broken.sql");
    std::fs::write(broken_file_location, broken_file).unwrap();

    // Compile should fail
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["compile"])
        .assert()
        .failure();
}

#[test]
fn test_yaml_no_model() {
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

    // Put a non-existent model in the yaml file
    let broken_file = "models: \
  - name: doesnt_exist";
    let broken_file_location = project_dir.join("models").join("test.yaml");
    std::fs::write(broken_file_location, broken_file).unwrap();

    // Compile should fail
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["compile"])
        .assert()
        .failure();
}
