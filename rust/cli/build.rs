use regex::Regex;
use std::env;

fn main() {
    let package_version = env!("CARGO_PKG_VERSION");

    // read toml file
    let toml = std::fs::read_to_string("Cargo.toml").unwrap();
    let regex = Regex::new(r#"duckdb = \{ version\s*=\s*"(\d+\.\d+\.\d+)""#).unwrap();
    let capture = regex.captures(toml.as_str()).unwrap().get(1).unwrap();
    let version = capture.as_str();

    // assert version is in the format "0.1.0"
    assert_eq!(version.split(".").count(), 3);
    assert!(version.split(".").all(|x| x.parse::<u32>().is_ok()));

    // assert package version is in the format "0.1.0"
    assert_eq!(package_version.split(".").count(), 3);
    assert!(package_version.split(".").all(|x| x.parse::<u32>().is_ok()));

    let version_string = format!("{} - dependencies: duckdb {}", package_version, version);

    println!("cargo:rustc-env=VERSION_STRING={}", version_string);
}
