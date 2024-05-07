use crate::file_system::{convert_async_read_to_blocking_read, FileSystem};
use crate::project::EXTENSION_CHART_YAML;
use crate::schema_name::DEFAULT_SCHEMA_PREFIX;
use crate::sql::{remove_sql_comments, return_reference_search};
use quary_proto::chart_file::Source;
use quary_proto::{Chart, ChartFile};
use std::collections::BTreeSet;
use std::io::Read;

pub fn chart_file_from_yaml(yaml: impl Read) -> Result<ChartFile, String> {
    serde_yaml::from_reader(yaml).map_err(|e| format!("reading yaml: {}", e))
}

// chart_file_to_yaml returns the written string as a yaml where the keys in the google.protobuf.Struct
// under config are sorted alphabetically
pub fn chart_file_to_yaml(chart_file: &ChartFile) -> Result<String, String> {
    serde_yaml::to_string(&chart_file).map_err(|e| format!("writing yaml: {}", e))
}

fn parse_chart_file_to_chart(path: &str, file: ChartFile) -> Result<(String, Chart), String> {
    let name: &str = path
        .split('/')
        .last()
        .ok_or("Invalid path to determined name")?
        .split('.')
        .next()
        .ok_or("Invalid path")?;

    let description = file.description;
    let tags = file.tags;

    let source = file.source.ok_or("no source provided".to_string())?;

    let (source, dependencies) = match source {
        Source::RawSql(sql) => {
            let dependencies = vec![];
            (quary_proto::chart::Source::RawSql(sql), dependencies)
        }
        Source::PreTemplatedSql(sql) => {
            let reference_search = return_reference_search(DEFAULT_SCHEMA_PREFIX).map_err(|e| {
                format!("Could not parse reference search from schema name: {:?}", e)
            })?;
            let contents = remove_sql_comments(&sql);
            let references: Vec<String> = reference_search
                .captures_iter(&contents)
                .map(|cap| {
                    cap.iter()
                        .map(|m| {
                            Ok(m.ok_or(format!(
                                "Could not parse reference search from schema name: {:?}",
                                m
                            ))?
                            .as_str()
                            .to_string())
                        })
                        .skip(1)
                        .step_by(2)
                        .collect::<Result<Vec<_>, String>>()
                })
                .collect::<Result<Vec<Vec<_>>, String>>()?
                .into_iter()
                .flatten()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let dependencies = references;
            (
                quary_proto::chart::Source::PreTemplatedSql(sql),
                dependencies,
            )
        }
        Source::Reference(reference) => {
            let name = reference.name;
            let dependencies = vec![name.clone()];
            (
                quary_proto::chart::Source::Reference(quary_proto::chart::AssetReference { name }),
                dependencies,
            )
        }
    };

    Ok((
        name.to_string(),
        Chart {
            name: name.to_string(),
            description,
            path: path.to_string(),
            tags,
            source: Some(source),
            references: dependencies,
            config: file.config,
        },
    ))
}

pub(crate) async fn parse_charts(
    file_system: &impl FileSystem,
    project_root: &str,
) -> Result<Vec<(String, Chart)>, String> {
    let paths = crate::project::get_path_bufs(
        file_system,
        project_root,
        crate::project::PATH_FOR_MODELS,
        EXTENSION_CHART_YAML,
        None,
    )
    .await?;
    let mut charts = Vec::new();
    for path in paths {
        let file = file_system
            .read_file(path.to_str().ok_or("Invalid path")?)
            .await
            .map_err(|e| format!("opening file: {}", e))?;

        let file = convert_async_read_to_blocking_read(file).await;
        let chart_file = chart_file_from_yaml(file)?;

        let (name, chart) =
            parse_chart_file_to_chart(path.to_str().ok_or("Invalid path")?, chart_file)?;
        charts.push((name, chart));
    }
    Ok(charts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quary_proto::chart_file::AssetReference;
    use std::collections::HashMap;
    use std::io;

    #[test]
    fn test_serialize_deserialize_chart_file() {
        let chart_file = ChartFile {
            description: Some("test description for chart".to_string()),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            source: Some(Source::PreTemplatedSql("SELECT * FROM table".to_string())),
            config: Some(pbjson_types::Struct {
                fields: HashMap::from([
                    (
                        "a".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue("a".to_string())),
                        },
                    ),
                    (
                        "b".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue("b".to_string())),
                        },
                    ),
                    (
                        "c".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue("c".to_string())),
                        },
                    ),
                    (
                        "d".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue("d".to_string())),
                        },
                    ),
                    (
                        "e".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue("e".to_string())),
                        },
                    ),
                    (
                        "f".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue("f".to_string())),
                        },
                    ),
                    (
                        "g".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue("g".to_string())),
                        },
                    ),
                ]),
            }),
        };

        let yaml = chart_file_to_yaml(&chart_file).unwrap();

        assert_eq!(
            r#"description: test description for chart
tags:
- tag1
- tag2
config:
  a: a
  b: b
  c: c
  d: d
  e: e
  f: f
  g: g
preTemplatedSql: SELECT * FROM table
"#,
            &yaml
        );

        let deserialized = chart_file_from_yaml(io::Cursor::new(yaml.as_bytes())).unwrap();

        assert_eq!(chart_file, deserialized);
    }

    #[test]
    fn parse_chart_file_raw_sql() {
        let chart_file = ChartFile {
            description: Some("test description for chart".to_string()),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            source: Some(Source::RawSql("SELECT * FROM table".to_string())),
            config: Some(pbjson_types::Struct {
                fields: HashMap::new(),
            }),
        };

        let (name, chart) =
            parse_chart_file_to_chart("models/test_path.chart.yaml", chart_file).unwrap();

        assert_eq!("test_path", name.as_str());
        assert_eq!(
            Chart {
                name: "test_path".to_string(),
                description: Some("test description for chart".to_string()),
                path: "models/test_path.chart.yaml".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                config: Some(pbjson_types::Struct {
                    fields: HashMap::new(),
                }),
                references: vec![],
                source: Some(quary_proto::chart::Source::RawSql(
                    "SELECT * FROM table".to_string()
                )),
            },
            chart
        );
    }

    #[test]
    fn parse_chart_file_templated_sql() {
        let chart_file = ChartFile {
            description: Some("test description for chart".to_string()),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            source: Some(Source::PreTemplatedSql(
                "SELECT * FROM q.model_a".to_string(),
            )),
            config: Some(pbjson_types::Struct {
                fields: HashMap::new(),
            }),
        };

        let (name, chart) =
            parse_chart_file_to_chart("models/test_path.chart.yaml", chart_file).unwrap();

        assert_eq!("test_path", name.as_str());
        assert_eq!(
            Chart {
                name: "test_path".to_string(),
                description: Some("test description for chart".to_string()),
                path: "models/test_path.chart.yaml".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                config: Some(pbjson_types::Struct {
                    fields: HashMap::new(),
                }),
                references: vec!["model_a".to_string()],
                source: Some(quary_proto::chart::Source::PreTemplatedSql(
                    "SELECT * FROM q.model_a".to_string()
                )),
            },
            chart
        );
    }

    #[test]
    fn parse_chart_file_raw_reference() {
        let chart_file = ChartFile {
            description: Some("test description for chart".to_string()),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            source: Some(Source::Reference(AssetReference {
                name: "model_a".to_string(),
            })),
            config: Some(pbjson_types::Struct {
                fields: HashMap::new(),
            }),
        };

        let (name, chart) =
            parse_chart_file_to_chart("models/test_chart.chart.yaml", chart_file).unwrap();

        assert_eq!("test_chart", name.as_str());
        assert_eq!(
            Chart {
                name: "test_chart".to_string(),
                description: Some("test description for chart".to_string()),
                path: "models/test_chart.chart.yaml".to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                config: Some(pbjson_types::Struct {
                    fields: HashMap::new(),
                }),
                references: vec!["model_a".to_string()],
                source: Some(quary_proto::chart::Source::Reference(
                    quary_proto::chart::AssetReference {
                        name: "model_a".to_string(),
                    }
                )),
            },
            chart
        );
    }
}
