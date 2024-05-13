use assert_cmd::Command;
use chrono::NaiveDateTime;
use chrono::Utc;
use quary_core::databases::DatabaseConnection;
use quary_databases::databases_duckdb;
use quary_databases::databases_redshift;
use std::fs;
use tempfile::tempdir;
use testcontainers::runners::AsyncRunner;
use testcontainers::RunnableImage;
use testcontainers_modules::postgres::Postgres as TestcontainersPostgres;
#[test]
fn test_cli_in_memory() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Define the sequence of command arguments
    let commands = vec![
        vec!["init", "-t=sqlite"],
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
        .args(vec!["init", "-t=sqlite"])
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

/// This test simulates a workflow where a model references a snapshot in duckdb.
/// 1. The initial snapshot is taken which builds the orders_snapshot table  in the database.
/// 2. The project is built which references the orders_snapshot table.
/// 3. The initial state of the snapshot is asserted.
/// 4. The data is updated and a new snapshot is taken.
/// 5. The updated state of the snapshot is asserted. (from the stg_orders_snapshot table)
#[tokio::test]
async fn test_duckdb_snapshots() {
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

    // Create data directory and orders.csv file
    let data_dir = project_dir.join("data");
    fs::create_dir_all(&data_dir).unwrap();
    let orders_file = data_dir.join("orders.csv");
    let orders_content = "id,user_id,order_date,status\n1,1,2023-06-01,in_progress";
    fs::write(&orders_file, orders_content).unwrap();

    // Create snapshots directory and orders_snapshot.snapshot.sql file
    let snapshots_dir = project_dir.join("models").join("staging").join("snapshots");
    fs::create_dir_all(&snapshots_dir).unwrap();
    let orders_snapshot_file = snapshots_dir.join("orders_snapshot.snapshot.sql");
    let orders_snapshot_content = "SELECT id, user_id, order_date, status FROM q.raw_orders";
    fs::write(&orders_snapshot_file, orders_snapshot_content).unwrap();

    let staging_models_dir = project_dir.join("models").join("staging");
    fs::create_dir_all(&staging_models_dir).unwrap();
    let stg_orders_snapshot_file = staging_models_dir.join("stg_orders_snapshot.sql");
    let stg_orders_snapshot_content = "SELECT * FROM q.orders_snapshot";
    fs::write(&stg_orders_snapshot_file, stg_orders_snapshot_content).unwrap();

    // Create schema.yaml file
    let schema_file = snapshots_dir.join("schema.yaml");
    let schema_content = r#"
    sources:
      - name: raw_orders
        path: "'data/orders.csv'"
    models:
      - name: stg_orders_snapshot
    snapshots:
      - name: orders_snapshot
        unique_key: id
        strategy:
          timestamp:
            updated_at: order_date
    "#;
    fs::write(&schema_file, schema_content).unwrap();

    // Create quary.yaml file
    let quary_yaml_content = "duckdb:\n  path: \"./database.db\"\n";
    let quary_yaml_path = project_dir.join("quary.yaml");
    fs::write(&quary_yaml_path, quary_yaml_content).unwrap();

    // Take the initial snapshot and build the project which references the snapshot
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["snapshot"])
        .assert()
        .success();
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["build"])
        .assert()
        .success();

    {
        // Assert initial snapshot
        let database = databases_duckdb::DuckDB::new_with_file(
            None,
            project_dir.join("database.db").to_str().unwrap(),
        )
        .ok()
        .unwrap();

        let result = database
        .query("SELECT id, user_id, order_date, status, quary_valid_from, quary_valid_to, quary_scd_id FROM orders_snapshot")
        .await
        .unwrap();

        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "1"); // id
        assert_eq!(result.rows[0][3], "in_progress"); // status
        assert_eq!(result.rows[0][5], "NULL"); // quary_valid_to

        // Check that quary_valid_from has the same date as the current date
        let current_date = Utc::now().date_naive();
        let quary_valid_from_str = &result.rows[0][4];
        let quary_valid_from =
            chrono::NaiveDate::parse_from_str(quary_valid_from_str, "%Y-%m-%d %H:%M:%S%.6f %Z")
                .unwrap();
        assert_eq!(current_date, quary_valid_from);

        // Update orders.csv data
        let updated_orders_content = "id,user_id,order_date,status\n1,1,2099-06-01,completed";
        fs::write(&orders_file, updated_orders_content).unwrap();
    }

    // Take updated snapshot
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["snapshot"])
        .assert()
        .success();

    {
        let database = databases_duckdb::DuckDB::new_with_file(
            None,
            project_dir.join("database.db").to_str().unwrap(),
        )
        .ok()
        .unwrap();

        // Assert updated snapshot
        let updated_result = database
            .query("SELECT id, user_id, order_date, status, quary_valid_from, quary_valid_to, quary_scd_id FROM stg_orders_snapshot ORDER BY quary_valid_from")
            .await
            .unwrap();

        assert_eq!(updated_result.rows.len(), 2);

        // Check the initial row
        assert_eq!(updated_result.rows[0][0], "1"); // id
        assert_eq!(updated_result.rows[0][3], "in_progress"); // status
        assert_ne!(updated_result.rows[0][5], "NULL"); // quary_valid_to should not be NULL

        // Check the updated row
        assert_eq!(updated_result.rows[1][0], "1"); // id
        assert_eq!(updated_result.rows[1][3], "completed"); // status
        assert_eq!(updated_result.rows[1][5], "NULL"); // quary_valid_to should be NULL

        // Check that quary_valid_from of the updated row has the same date as the current date
        // Check that quary_valid_from has the same date as the current date
        let current_date = Utc::now().date_naive();
        let quary_valid_from_str = &updated_result.rows[1][4];
        let quary_valid_from =
            chrono::NaiveDate::parse_from_str(quary_valid_from_str, "%Y-%m-%d %H:%M:%S%.6f %Z")
                .unwrap();
        assert_eq!(current_date, quary_valid_from);
    }
}

/// This test simulates a workflow where a model references a snapshot in redshift.
/// 1. The initial snapshot is taken which builds the orders_snapshot table  in the database.
/// 2. The project is built which references the orders_snapshot table.
/// 3. The initial state of the snapshot is asserted.
/// 4. The data is updated and a new snapshot is taken.
/// 5. The updated state of the snapshot is asserted. (from the stg_orders_snapshot table)
#[tokio::test]
#[ignore]
async fn test_redshift_snapshots() {
    // Prepare the database
    let database =
        databases_redshift::Redshift::new("", None, "", "", "", "", None, None, None, None, None)
            .await
            .ok()
            .unwrap();
    database
        .exec("DROP TABLE analytics.orders CASCADE")
        .await
        .unwrap();
    database
        .exec("DROP TABLE transform.orders_snapshot CASCADE")
        .await
        .unwrap();

    database
        .exec(
            "
        CREATE TABLE analytics.orders (
            order_id character varying(255) ENCODE lzo,
            customer_id character varying(255) ENCODE lzo,
            order_date timestamp without time zone ENCODE az64,
            total_amount numeric(10, 2) ENCODE az64,
            status character varying(255) ENCODE lzo
        ) DISTSTYLE AUTO;
    ",
        )
        .await
        .unwrap();

    database.exec(
            "
            INSERT INTO analytics.orders (order_id, customer_id, order_date, total_amount, status) VALUES ('1', '1', '2022-01-01 00:00:00', 100, 'in_progress')
            "
        )
        .await
        .unwrap();

    // Setup
    let name = "quary";
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // create a .env file
    let env_file_path = project_dir.join(".env");
    let env_content =
        "REDSHIFT_HOST=\nREDSHIFT_PORT=\nREDSHIFT_USER=\nREDSHIFT_PASSWORD=\nREDSHIFT_DATABASE=";
    fs::write(&env_file_path, env_content).unwrap();

    // Create snapshots directory and orders_snapshot.snapshot.sql file
    let snapshots_dir = project_dir.join("models").join("staging").join("snapshots");
    fs::create_dir_all(&snapshots_dir).unwrap();
    let orders_snapshot_file = snapshots_dir.join("orders_snapshot.snapshot.sql");
    let orders_snapshot_content =
        "SELECT order_id, customer_id, order_date, total_amount, status FROM q.raw_orders";
    fs::write(&orders_snapshot_file, orders_snapshot_content).unwrap();

    // Create a model which references the snapshot
    let staging_models_dir = project_dir.join("models").join("staging");
    fs::create_dir_all(&staging_models_dir).unwrap();
    let stg_orders_snapshot_file = staging_models_dir.join("stg_orders_snapshot.sql");
    let stg_orders_snapshot_content = "SELECT * FROM q.orders_snapshot";
    fs::write(&stg_orders_snapshot_file, stg_orders_snapshot_content).unwrap();

    // Create quary.yaml file
    let quary_yaml_content = "redshift:\n  schema: transform";
    let quary_yaml_path = project_dir.join("quary.yaml");
    fs::write(&quary_yaml_path, quary_yaml_content).unwrap();

    // Create schema.yaml file
    let schema_file = snapshots_dir.join("schema.yaml");
    let schema_content = r#"
    sources:
    - name: raw_orders
      path: analytics.orders
    snapshots:
    - name: orders_snapshot
      unique_key: order_id
      strategy:
       timestamp:
        updated_at: order_date
    "#;
    fs::write(&schema_file, schema_content).unwrap();

    // Take the initial snapshot and build the project which references the snapshot
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["snapshot"])
        .assert()
        .success();
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["build"])
        .assert()
        .success();

    {
        let result = database
        .query("SELECT order_id, customer_id, order_date, total_amount, status, quary_valid_from, quary_valid_to, quary_scd_id FROM transform.orders_snapshot")
        .await
        .unwrap();

        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "1"); // id
        assert_eq!(result.rows[0][4], "in_progress"); // status
        assert_eq!(result.rows[0][6], "NULL"); // quary_valid_to

        // Check that quary_valid_from has the same date as the current date
        let current_date: NaiveDateTime = Utc::now().date_naive().into();
        let quary_valid_from_str = &result.rows[0][5];
        let quary_valid_from_date = quary_valid_from_str.split_whitespace().next().unwrap();

        let quary_valid_from =
            chrono::NaiveDateTime::parse_from_str(quary_valid_from_date, "%Y-%m-%dT%H:%M:%S%.f%:z")
                .unwrap();
        assert_eq!(current_date.date(), quary_valid_from.date());

        database
            .exec(
                "
            UPDATE analytics.orders
            SET order_date = '2099-06-01 00:00:00', status = 'completed'
            WHERE order_id = '1'
            ",
            )
            .await
            .unwrap();
    }

    // Take updated snapshot
    Command::cargo_bin(name)
        .unwrap()
        .current_dir(project_dir)
        .args(vec!["snapshot"])
        .assert()
        .success();

    {
        // Assert updated snapshot
        let updated_result = database
            .query("SELECT order_id, customer_id, order_date, total_amount, status, quary_valid_from, quary_valid_to, quary_scd_id FROM transform.stg_orders_snapshot ORDER BY quary_valid_from")
            .await
            .unwrap();

        assert_eq!(updated_result.rows.len(), 2);

        // Check the initial row
        assert_eq!(updated_result.rows[0][0], "1"); // id
        assert_eq!(updated_result.rows[0][4], "in_progress"); // status
        assert_ne!(updated_result.rows[0][6], "NULL"); // quary_valid_to should not be NULL

        // Check the updated row
        assert_eq!(updated_result.rows[1][0], "1"); // id
        assert_eq!(updated_result.rows[1][4], "completed"); // status
        assert_eq!(updated_result.rows[1][6], "NULL"); // quary_valid_to should be NULL

        // Check that quary_valid_from of the updated row has the same date as the current date
        let current_date: NaiveDateTime = Utc::now().date_naive().into();
        let updated_quary_valid_from_str = &updated_result.rows[1][5];
        let updated_quary_valid_from_date = updated_quary_valid_from_str
            .split_whitespace()
            .next()
            .unwrap();
        let updated_quary_valid_from = chrono::NaiveDateTime::parse_from_str(
            updated_quary_valid_from_date,
            "%Y-%m-%dT%H:%M:%S%.f%:z",
        )
        .unwrap();
        assert_eq!(current_date.date(), updated_quary_valid_from.date());
    }
}

/// This test simulates a workflow where a model is built twice in a Postgres database.
/// In Postgres DROP TABLE IF EXISTS requires a CASCADE if there are view dependencies
/// Error we are trying to avoid: cannot drop view transform.model_1 because other objects depend on it
/// replace with materialized_view field in the schame to: view, materialized_view & table to test difference scenarios
#[tokio::test]
async fn test_postgres_build_model_twice() {
    // Setup
    let postgres = RunnableImage::from(TestcontainersPostgres::default())
        .start()
        .await;

    let name = "quary";
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // create a .env file
    let env_file_path = project_dir.join(".env");
    let env_content = format!(
        "PGHOST=localhost\nPGPORT={}\nPGUSER=postgres\nPGPASSWORD=postgres\nPGDATABASE=postgres",
        postgres.get_host_port_ipv4(5432).await
    );
    fs::write(&env_file_path, env_content).unwrap();

    // Create a model which reference a model
    let models_dir: std::path::PathBuf = project_dir.join("models");
    fs::create_dir_all(&models_dir).unwrap();
    let model_1_file = models_dir.join("model_1.sql");
    let model_1_content = "SELECT 100 as random_number";
    let model_2_file = models_dir.join("model_2.sql");
    let model_2_content = "SELECT * FROM q.model_1";
    fs::write(&model_1_file, model_1_content).unwrap();
    fs::write(&model_2_file, model_2_content).unwrap();

    // Create quary.yaml file
    let quary_yaml_content = "postgres:\n  schema: public";
    let quary_yaml_path = project_dir.join("quary.yaml");
    fs::write(&quary_yaml_path, quary_yaml_content).unwrap();

    // Create schema.yaml file
    let schema_file = models_dir.join("schema.yaml");
    let schema_content = r#"
        models:
        - name: model_1
          materialization: view
        - name: model_2
          materialization: view 
        "#;
    fs::write(&schema_file, schema_content).unwrap();
    {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(vec!["build"])
            .assert()
            .success();
    }
    {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(vec!["build"])
            .assert()
            .success();
    }
}

/// This test simulates a workflow where a test is added to a model and we run quary test -s (from source).
#[tokio::test]
async fn test_postgres_run_tests_from_sources() {
    // Setup
    let postgres = RunnableImage::from(TestcontainersPostgres::default())
        .start()
        .await;

    let name = "quary";
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // create a .env file
    let env_file_path = project_dir.join(".env");
    let env_content = format!(
        "PGHOST=localhost\nPGPORT={}\nPGUSER=postgres\nPGPASSWORD=postgres\nPGDATABASE=postgres",
        postgres.get_host_port_ipv4(5432).await
    );
    fs::write(&env_file_path, env_content).unwrap();

    // Create a model which reference a model
    let models_dir: std::path::PathBuf = project_dir.join("models");
    fs::create_dir_all(&models_dir).unwrap();
    let model_1_file = models_dir.join("model_1.sql");
    let model_1_content = "SELECT 100 as random_number";
    fs::write(&model_1_file, model_1_content).unwrap();

    // Create quary.yaml file
    let quary_yaml_content = "postgres:\n  schema: public";
    let quary_yaml_path = project_dir.join("quary.yaml");
    fs::write(&quary_yaml_path, quary_yaml_content).unwrap();

    // Create schema.yaml file
    let schema_file = models_dir.join("schema.yaml");
    let schema_content = r#"
        models:
        - name: model_1
          columns:
          - name: random_number
            description: "This is a random number which should never be null"
            tests:
            - type: not_null
            - type: unique
        "#;
    fs::write(&schema_file, schema_content).unwrap();
    {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(vec!["test", "-s"])
            .assert()
            .success();
    }
}

/// This test simulates a workflow where a test is added to a model and we run quary test (from database).
#[tokio::test]
async fn test_postgres_run_tests_from_database_tables() {
    // Setup
    let postgres = RunnableImage::from(TestcontainersPostgres::default())
        .start()
        .await;

    let name = "quary";
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // create a .env file
    let env_file_path = project_dir.join(".env");
    let env_content = format!(
        "PGHOST=localhost\nPGPORT={}\nPGUSER=postgres\nPGPASSWORD=postgres\nPGDATABASE=postgres",
        postgres.get_host_port_ipv4(5432).await
    );
    fs::write(&env_file_path, env_content).unwrap();

    // Create a model which reference a model
    let models_dir: std::path::PathBuf = project_dir.join("models");
    fs::create_dir_all(&models_dir).unwrap();
    let model_1_file = models_dir.join("model_1.sql");
    let model_1_content = "SELECT 100 as random_number";
    fs::write(&model_1_file, model_1_content).unwrap();

    // Create quary.yaml file
    let quary_yaml_content = "postgres:\n  schema: public";
    let quary_yaml_path = project_dir.join("quary.yaml");
    fs::write(&quary_yaml_path, quary_yaml_content).unwrap();

    // Create schema.yaml file
    let schema_file = models_dir.join("schema.yaml");
    let schema_content = r#"
        models:
        - name: model_1
          columns:
          - name: random_number
            description: "This is a random number which should never be null"
            tests:
            - type: not_null
            - type: unique
        "#;
    fs::write(&schema_file, schema_content).unwrap();
    {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(vec!["build"])
            .assert()
            .success();
    }
    {
        Command::cargo_bin(name)
            .unwrap()
            .current_dir(project_dir)
            .args(vec!["test"])
            .assert()
            .success();
    }
}
