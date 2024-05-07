
use assert_cmd::Command;
use duckdb::{params, Connection};
use std::fs;
use tempfile::tempdir;

/// This test simulates a workflow where a model references a snapshot in duckdb.
/// 1. The initial snapshot is taken which builds the orders_snapshot table  in the database.
/// 2. The project is built which references the orders_snapshot table.
/// 3. The initial state of the snapshot is asserted.
/// 4. The data is updated and a new snapshot is taken.
/// 5. The updated state of the snapshot is asserted. (from the stg_orders_snapshot table)
#[tokio::test]
async fn test_generate_sources_duckdb() {
    // Setup
    let name = "quary";
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["init", "--type=duckdb"])
        .assert()
        .success();

    // Create quary.yaml file
    let quary_yaml_content = "duckdb:\n  path: \"./database.db\"\n";
    let quary_yaml_path = project_dir.join("quary.yaml");
    fs::write(&quary_yaml_path, quary_yaml_content).unwrap();

    Command::cargo_bin(name)
    .unwrap()
    .current_dir(project_dir)
    .args(vec!["build"])
    .assert()
    .success();

    let conn = Connection::open("database.db").unwrap();

    // Create a new table named "users"
    conn.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name VARCHAR(50),
            age INTEGER
        )",
        params![],
    ).unwrap();

let binding = Command::cargo_bin(name)
.unwrap()
.current_dir(project_dir)
.args(vec!["generate-sources"])
.assert()
.success();
let output = binding
.get_output();

let stdout = &output.stdout;
println!("Output: {}", String::from_utf8_lossy(stdout));



}