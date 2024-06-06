use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_init() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Create a file that looks like git
    let git_dir = project_dir.join(".git");
    std::fs::create_dir(&git_dir).unwrap();
    let git_dir_head = git_dir.join("HEAD");
    std::fs::write(git_dir_head, "").unwrap();

    // Create a file that looks like a gitignore
    let gitignore = project_dir.join(".gitignore");
    std::fs::write(gitignore, "").unwrap();

    // Define the sequence of command arguments
    let commands = vec![vec!["init"]];

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
fn test_init_duckdb() {
    let name = "quary";

    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let project_dir = temp_dir.path();

    // Create a file that looks like git
    let git_dir = project_dir.join(".git");
    std::fs::create_dir(&git_dir).unwrap();
    let git_dir_head = git_dir.join("HEAD");
    std::fs::write(git_dir_head, "").unwrap();

    // Create a file that looks like a gitignore
    let gitignore = project_dir.join(".gitignore");
    std::fs::write(gitignore, "").unwrap();

    // Create a file in `.idea` for jetbrains based tooling
    let jetbrains_folder = project_dir.join(".idea");
    std::fs::create_dir(&jetbrains_folder).unwrap();

    let jetbrains_file = jetbrains_folder.join("temp.text");
    std::fs::write(jetbrains_file, "").unwrap();

    // Define the sequence of command arguments
    let commands = vec![vec!["init", "--type=duckdb"]];

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
