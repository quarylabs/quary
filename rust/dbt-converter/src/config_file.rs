use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ConfigFile {
    pub(crate) name: String,
    #[serde(rename = "config-version")]
    pub(crate) config_version: u32,
    pub(crate) version: String,
    pub(crate) profile: String,
    #[serde(rename = "model-paths")]
    pub(crate) model_paths: Vec<String>,
    #[serde(rename = "seed-paths")]
    pub(crate) seeds_paths: Vec<String>,
    #[serde(rename = "test-paths")]
    pub(crate) tests_paths: Vec<String>,
    // ... similarly for other fields
}

impl ConfigFile {
    pub(crate) fn from_yaml(reader: impl io::Read) -> serde_yaml::Result<Self> {
        serde_yaml::from_reader(reader)
    }
}
