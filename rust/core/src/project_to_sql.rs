use crate::databases::{DatabaseConnection, DatabaseQueryGenerator};
use crate::file_system::{convert_async_read_to_blocking_read, FileSystem};
use crate::file_system_override::OverrideFileSystem;
use crate::graph::{project_to_graph, Edge};
use crate::models::{parse_model_schemas_to_views, read_normalise_model};
use crate::project::parse_project;
use crate::schema_name::DEFAULT_SCHEMA_PREFIX;
use crate::seeds::parse_table_schema_seeds;
use crate::sql::return_reference_search;
use quary_proto::{ConnectionConfig, Model, Project, Seed, Snapshot, Source};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

/// project_and_fs_to_query_sql_for_model_sql wraps project_and_fs_to_query_sql but injects model
/// string into the fs and project. This is useful for when you want to get the sql for a model
/// that is not in the project. The model is injected into the project and file system and then
/// the sql is generated.
///
/// For example: SELECT * FROM q.shifts can be injected and the full qualified sql can be returned.
pub async fn project_and_fs_to_query_sql_for_model_sql(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    project_root: &str,
    model_sql: &str,
    overrides: Option<HashMap<String, String>>,
    temporary_id: &str,
) -> Result<(String, (BTreeSet<String>, Vec<Edge>)), String> {
    let project_root_path = Path::new(project_root);
    let full_path = project_root_path
        .join("models")
        .join(format!("{}.sql", temporary_id));
    let random_model_location = full_path
        .to_str()
        .ok_or("failed to convert random model location to string")?;

    let mut file_system = OverrideFileSystem::new(Box::new(file_system));
    file_system.add_override(random_model_location, model_sql);

    let project = parse_project(&file_system, database, project_root).await?;
    project_and_fs_to_query_sql(database, &project, &file_system, temporary_id, overrides).await
}

/// project_and_fs_to_query_sql returns the sql statement for a model in a project. The model can be
/// a source, seed, model or snapshot. The dependencies are resolved and the sql statement is
/// returned.
///
/// - `overrides` is a map of model name to the string that the reference should be replaced with if
/// a model is used that is found in overrides, then the reference will be replaced with a
/// `SELECT * FROM {found_value}` and any upstream references are dropped.
///
/// For example, if the dependencies are A -> B -> C and overrides is {B: "D"} then the returned
/// A -> D.
///
/// Returns a tuple of
/// - the sql statement
/// - the (nodes, edges) of models and other assets that were used to create the sql statement.
pub async fn project_and_fs_to_query_sql(
    database: &impl DatabaseQueryGenerator,
    project: &Project,
    file_system: &impl FileSystem,
    model_name: &str,
    overrides: Option<HashMap<String, String>>,
) -> Result<(String, (BTreeSet<String>, Vec<Edge>)), String> {
    // TODO Some really ugly code here. Clean it up.
    match (
        project.sources.get(model_name),
        project.seeds.get(model_name),
        project.snapshots.get(model_name),
        project.models.get(model_name),
    ) {
        (Some(_), None, None, None) => Ok(AssetType::Source),
        (None, Some(_), None, None) => Ok(AssetType::Seed),
        (None, None, Some(_), None) => Ok(AssetType::Snapshot),
        (None, None, None, Some(_)) => Ok(AssetType::Model),
        _ => Err(format!(
            "requested {:?} is neither a seed, nor source, nor model, nor snapshot",
            model_name
        )),
    }?;

    let overrides = overrides.unwrap_or_default();
    let graph = project_to_graph(project.clone())?;
    let mut overriden_graph = graph.graph;
    for model in overrides.keys() {
        if overriden_graph.graph.node_weights().any(|n| n == model) {
            let (override_graph, _) =
                overriden_graph.return_shrunk_downstream_graph(model.as_str(), model_name)?;
            overriden_graph = override_graph;
        }
    }

    let (upstream, models) = overriden_graph.return_parent_nods_to_apply_in_order(model_name)?;

    let to_process: Vec<NodeWithName> = models
        .iter()
        .map(|name| {
            match (
                overrides.get(name),
                project.sources.get(name),
                project.seeds.get(name),
                project.snapshots.get(name),
                project.models.get(name),
            ) {
                // model overriden by cache
                (Some(overriden), None, None, None, Some(_)) => Ok(NodeWithName {
                    name: name.to_string(),
                    asset: AssetData::Override((name.clone(), overriden.clone())),
                }),
                // snapshot overriden by cache
                (Some(overriden), None, None, Some(_), None) => Ok(NodeWithName {
                    name: name.to_string(),
                    asset: AssetData::Override((name.clone(), overriden.clone())),
                }),
                (None, Some(source), None, None, None) => Ok(NodeWithName {
                    name: name.to_string(),
                    asset: AssetData::Source(source.clone()),
                }),
                (None, None, Some(seed), None, None) => Ok(NodeWithName {
                    name: name.to_string(),
                    asset: AssetData::Seed(seed.clone()),
                }),
                (None, None, None, Some(snapshot), None) => Ok(NodeWithName {
                    name: name.to_string(),
                    asset: AssetData::Snapshot(snapshot.clone()),
                }),
                (None, None, None, None, Some(model)) => Ok(NodeWithName {
                    name: name.to_string(),
                    asset: AssetData::Model(model.clone()),
                }),
                _ => Err(format!(
                    "model {:?} is neither a seed nor a model nor a source nor a override nor a snapshot",
                    name
                )),
            }
        })
        .collect::<Result<Vec<_>, String>>()?;

    let sql = convert_to_select_statement(database, file_system, &to_process, project).await?;

    let edges = upstream.return_graph_edges()?;
    let nodes = upstream.graph.node_weights().cloned().collect();

    Ok((sql, (nodes, edges)))
}

/// project_and_fs_to_sql_for_views returns the sql for creating tables for seeds and views for models
/// for the project. The entries are returned in the right order in which they need to be applied.
/// Each returned entry is a tuple of the form (model/seed name, sql statements that make up the
/// model/seed).
///
/// returns: Result<Vec<(String, Vec<String, Global>), Global>, String>
pub async fn project_and_fs_to_sql_for_views(
    project: &Project,
    file_system: &impl FileSystem,
    database: &impl DatabaseQueryGenerator,
    only_models: bool,
    do_not_include_seeds_data: bool,
) -> Result<Vec<(String, Vec<String>)>, String> {
    let graph = project_to_graph(project.clone())?;
    let sorted = graph.graph.get_node_sorted()?;

    let models: Vec<_> = sorted
        .iter()
        .filter_map(|node| {
            match (
                project.seeds.get(node),
                project.sources.get(node),
                project.tests.get(node),
                project.models.get(node),
            ) {
                (Some(_), _, _, _) => None,
                (_, Some(_), _, _) => None,
                (_, _, Some(_), _) => None,
                (_, _, _, Some(model)) => Some(model),
                _ => None,
            }
        })
        .map(|model| async move {
            let file = file_system.read_file(&model.file_path).await.map_err(|e| {
                format!(
                    "failed to read file {:?} with error {:?}",
                    model.file_path, e
                )
            })?;
            let sql_view = parse_model_schemas_to_views(
                database,
                file,
                &model.name,
                &model.materialization,
                DEFAULT_SCHEMA_PREFIX,
                |s| {
                    let replaced = replace_reference_string_found_with_database(
                        &project.sources,
                        &database,
                    )(s);
                    let replaced = replaced.trim();
                    format!(" {}", replaced)
                },
                project,
            )
            .await?;
            Ok::<(String, Vec<String>), String>((model.name.clone(), sql_view))
        })
        .collect();
    let models: Vec<(String, Vec<String>)> = futures::future::join_all(models)
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;

    let models_map: HashMap<String, Vec<String>> = models
        .iter()
        .map(|(name, vec)| (name.clone(), vec.to_vec()))
        .collect::<HashMap<String, Vec<String>>>();
    let mut models = vec![];
    for model in sorted.iter() {
        if let Some(sqls) = models_map.get(model) {
            models.push((model.clone(), sqls.clone()));
        }
    }
    // let mut order: Vec<(String, Vec<String>>) = Vec::new();
    // }
    // models.sort_by_key(|a| *order.get(&a.0).unwrap());

    // TODO Make more efficient by skipping the seeds if this is on.
    if only_models {
        return Ok(models);
    }
    let mut seeds: Vec<&Seed> = project.seeds.values().collect();
    seeds.sort_by_key(|a| a.name.clone());
    let seeds_out = seeds
        .iter()
        .map(|seed| async move {
            let reader = file_system.read_file(&seed.file_path).await.map_err(|e| {
                format!(
                    "failed to read file {:?} with error {:?}",
                    seed.file_path, e
                )
            })?;
            let values =
                parse_table_schema_seeds(database, &seed.name, reader, do_not_include_seeds_data)
                    .await
                    .map_err(|e| {
                        format!("failed to parse seed {:?} with error {:?}", seed.name, e)
                    })?;
            Ok::<(String, Vec<String>), String>((seed.name.clone(), values))
        })
        .collect::<Vec<_>>();
    let mut seeds_out: Vec<_> = futures::future::join_all(seeds_out)
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;

    seeds_out.append(&mut models);
    Ok(seeds_out)
}

/// Generates SQL statements for snapshots based on the project and file system.
pub async fn project_and_fs_to_sql_for_snapshots(
    project: &Project,
    file_system: &impl FileSystem,
    database: &impl DatabaseQueryGenerator,
    database_connection: &dyn DatabaseConnection,
) -> Result<Vec<(String, Vec<String>)>, String> {
    let snapshots_out = project.snapshots.values().map(|snapshot| async move {
        let connection_config = project
            .connection_config
            .clone()
            .ok_or("missing connection config")?;

        let sql_statements = generate_snapshot_sql(
            &connection_config,
            project,
            database,
            file_system,
            snapshot,
            database_connection,
        )
        .await?;
        Ok::<(String, Vec<String>), String>((snapshot.name.clone(), sql_statements))
    });

    let snapshots_out: Vec<_> = futures::future::join_all(snapshots_out)
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;
    Ok(snapshots_out)
}

/// Generates SQL statements for a specific snapshot.
async fn generate_snapshot_sql(
    connection_config: &ConnectionConfig,
    project: &Project,
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    snapshot: &Snapshot,
    database_connection: &dyn DatabaseConnection,
) -> Result<Vec<String>, String> {
    let snapshot_strategy = snapshot
        .strategy
        .clone()
        .ok_or("missing snapshot strategy")?
        .strategy_type
        .ok_or("missing strategy type")?;
    let snapshot_path = database.return_full_path_requirement(&snapshot.name);
    let file = file_system
        .read_file(&snapshot.file_path)
        .await
        .map_err(|e| {
            format!(
                "failed to read file {:?} with error {:?}",
                &snapshot.file_path, e
            )
        })?;
    let raw_query = read_normalise_model(file).await?;
    let vars_replaced_select_statement =
        replace_variable_templates_with_variable_defined_in_config(&raw_query, connection_config)?;

    let sources = &project.sources;
    let name_replacing_strategy = move |s: &regex::Captures| {
        let replaced = replace_reference_string_found_with_database(sources, &database)(s);
        let replaced = replaced.trim();
        format!(" {}", replaced)
    };

    let reference_search =
        return_reference_search(DEFAULT_SCHEMA_PREFIX).map_err(|e| e.to_string())?;
    let templated_select =
        reference_search.replace_all(&vars_replaced_select_statement, name_replacing_strategy);

    let table_exists = match database_connection.table_exists(&snapshot_path).await {
        Ok(Some(exists)) => Some(exists),
        Ok(None) => None,
        Err(err) => {
            return Err(format!(
                "An error occurred checking for the existence of the snapshot table: {}",
                err
            ));
        }
    };
    database.generate_snapshot_sql(
        &snapshot_path,
        &templated_select,
        &snapshot.unique_key,
        &snapshot_strategy,
        table_exists,
    )
}

/// convertToSelectStatements takes in an array of model/seed and returns a string that can be used in a select statement.
/// It also replaces any q.references with the actual name that is in the select. It uses no views.
///
/// array of models is in the shape of [][2]string where the first element is the name of the model and the second element is the sql
async fn convert_to_select_statement(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    values: &[NodeWithName],
    project: &Project,
) -> Result<String, String> {
    /// Info contains the name of the model and the sql for that model
    type Info = (String, String);

    let nodes = values
        .iter()
        .map(|node| async move {
            match &node.asset {
                AssetData::Override((name, target)) => {
                    let sql = render_override_select_statement(target);
                    Ok((name.clone(), sql))
                }
                AssetData::Source(source) => {
                    let sql = render_source_select_statement(source);
                    Ok((node.name.clone(), sql))
                }
                AssetData::Seed(seed) => {
                    let sql = render_seed_select_statement(database, file_system, seed).await?;
                    Ok((node.name.clone(), sql))
                }
                AssetData::Snapshot(snapshot) => {
                    let sql =
                        render_snapshot_select_statement(database, file_system, snapshot, project)
                            .await?;
                    Ok((node.name.clone(), sql))
                }
                AssetData::Model(model) => {
                    let sql = render_model_select_statement(database, file_system, model, project)
                        .await?;
                    Ok((node.name.clone(), sql))
                }
            }
        })
        .collect::<Vec<_>>();
    let nodes = futures::future::join_all(nodes)
        .await
        .into_iter()
        .collect::<Result<Vec<Info>, String>>()?;

    match &nodes[..] {
        [] => Err("no nodes to process".to_string()),
        [(_, sql)] => Ok(sql.clone()),
        [node1, node2] => Ok(format!(
            "WITH {} AS ({}) SELECT * FROM ({}) AS alias",
            node1.0, node1.1, node2.1
        )),
        [withs @ .., last] => {
            let withs = withs
                .iter()
                .map(|(name, sql)| format!("{} AS ({})", name, sql))
                .collect::<Vec<_>>()
                .join(",\n");
            Ok(format!(
                "WITH\n{}\nSELECT * FROM ({}) AS alias",
                withs, last.1
            ))
        }
    }
}

async fn render_seed_select_statement(
    database: &impl DatabaseQueryGenerator,
    fs: &impl FileSystem,
    seed: &Seed,
) -> Result<String, String> {
    let reader = fs.read_file(seed.file_path.as_str()).await.map_err(|e| {
        format!(
            "failed to read file {:?} with error {:?}",
            seed.file_path, e
        )
    })?;
    let reader = convert_async_read_to_blocking_read(reader).await;

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);

    let mut records: Vec<Vec<String>> = Vec::new();
    for result in csv_reader.records() {
        let record = result
            .map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<String>>())
            .map_err(|e| format!("error reading csv record: {:?}", e))?;
        records.push(record);
    }
    let headers = records
        .first()
        .ok_or("no headers found in csv".to_string())?;
    let records = records
        .get(1..)
        .ok_or("no records found in csv".to_string())?;

    Ok(render_seed_select_statement_string(
        database,
        headers.clone(),
        records.to_vec(),
    ))
}

fn render_seed_select_statement_string(
    database: &impl DatabaseQueryGenerator,
    headers: Vec<String>,
    values: Vec<Vec<String>>,
) -> String {
    let header_section = headers
        .iter()
        .enumerate()
        .map(|(i, header)| format!("column{} AS {}", i + 1, header))
        .collect::<Vec<String>>()
        // TODO Could use intersperse here
        .join(",");

    let values_section = values
        .iter()
        .map(|row| {
            format!(
                "({})",
                row.iter()
                    .map(|value| format!("'{}'", database.escape_seed_value(value)))
                    .collect::<Vec<String>>()
                    .join(",")
            )
        })
        .collect::<Vec<String>>()
        .join(",");

    format!("SELECT {} FROM (VALUES {})", header_section, values_section)
}

fn render_source_select_statement(source: &Source) -> String {
    format!("SELECT * FROM {}", source.path)
}

fn render_override_select_statement(override_target: &str) -> String {
    format!("SELECT * FROM {}", override_target)
}

async fn render_model_select_statement(
    database: &impl DatabaseQueryGenerator,
    fs: &impl FileSystem,
    model: &Model,
    project: &Project,
) -> Result<String, String> {
    let reader = fs.read_file(model.file_path.as_str()).await.map_err(|e| {
        format!(
            "failed to read file {:?} with error {:?}",
            model.file_path, e
        )
    })?;
    let sql = read_normalise_model(reader).await?;

    let reference_search = return_reference_search(DEFAULT_SCHEMA_PREFIX).map_err(|e| {
        format!(
            "error creating reference search for model {:?}: {:?}",
            model.name, e
        )
    })?;
    let replaced = reference_search.replace_all(
        sql.as_str(),
        replace_reference_string_found(&HashMap::new(), database),
    );
    let connection_config = project
        .connection_config
        .as_ref()
        .ok_or_else(|| "Connection config is required".to_string())?;

    let replaced =
        replace_variable_templates_with_variable_defined_in_config(&replaced, connection_config)?;
    Ok(replaced)
}

async fn render_snapshot_select_statement(
    database: &dyn DatabaseQueryGenerator,
    fs: &impl FileSystem,
    snapshot: &Snapshot,
    project: &Project,
) -> Result<String, String> {
    let reader = fs
        .read_file(snapshot.file_path.as_str())
        .await
        .map_err(|e| {
            format!(
                "failed to read file {:?} with error {:?}",
                snapshot.file_path, e
            )
        })?;
    let sql = read_normalise_model(reader).await?;

    let reference_search = return_reference_search(DEFAULT_SCHEMA_PREFIX).map_err(|e| {
        format!(
            "error creating reference search for snapshot {:?}: {:?}",
            snapshot.name, e
        )
    })?;
    let replaced = reference_search.replace_all(
        sql.as_str(),
        replace_reference_string_found(&HashMap::new(), database),
    );
    let connection_config = project
        .connection_config
        .as_ref()
        .ok_or_else(|| "Connection config is required".to_string())?;

    let replaced =
        replace_variable_templates_with_variable_defined_in_config(&replaced, connection_config)?;

    let snapshot_strategy = snapshot
        .strategy
        .clone()
        .ok_or("missing snapshot strategy")?;
    let snapshot_strategy_type = snapshot_strategy
        .strategy_type
        .ok_or("missing snapshot strategy type")?;

    database.generate_snapshot_query(
        &replaced,
        &snapshot.unique_key,
        &snapshot_strategy_type,
        &database.get_current_timestamp(),
    )
}

pub fn replace_variable_templates_with_variable_defined_in_config(
    sql: &str,
    connection_config: &ConnectionConfig,
) -> Result<String, String> {
    let re = regex::Regex::new(r"\{\{\s*var\('([^']+)'\)\s*\}\}").map_err(|e| e.to_string())?;

    let mut errors = Vec::new();

    let result = re
        .replace_all(sql, |caps: &regex::Captures| {
            let var_name = match caps.get(1) {
                Some(m) => m.as_str(),
                None => {
                    errors.push("Missing variable target".to_string());
                    return ""; // placeholder if variable target is missing
                }
            };
            match connection_config
                .vars
                .iter()
                .find(|var| var.name == var_name)
            {
                Some(var) => &var.value,
                None => {
                    errors.push(format!("Variable '{}' not found", var_name));
                    "" // placeholder if variable value is missing
                }
            }
        })
        .to_string();

    // Check if there were any errors during replacement
    if let Some(error) = errors.first() {
        return Err(error.clone());
    }

    Ok(result)
}

fn replace_reference_string_found_with_database<'a>(
    sources: &'a HashMap<String, Source>,
    database: &'a &impl DatabaseQueryGenerator,
) -> Box<dyn Fn(&regex::Captures) -> String + 'a> {
    #[allow(clippy::indexing_slicing)]
    Box::new(move |caps: &regex::Captures| {
        let model = &caps[1];
        let model = sources
            .get(model)
            .map(|s| s.path.clone())
            .unwrap_or(database.return_full_path_requirement(model));
        let wrapped = database.database_name_wrapper(model.as_str());
        format!(" {}", wrapped)
    })
}

pub fn replace_reference_string_found<'a>(
    overrides: &'a HashMap<String, String>,
    database: &'a (impl DatabaseQueryGenerator + ?Sized),
) -> Box<dyn Fn(&regex::Captures) -> String + 'a> {
    #[allow(clippy::indexing_slicing)]
    Box::new(move |caps: &regex::Captures| {
        let model = &caps[1];
        if let Some(path) = overrides.get(model) {
            if path.starts_with('(') & path.ends_with(')') {
                format!(" {}", path)
            } else {
                format!(" {}", database.database_name_wrapper(path))
            }
        } else {
            format!(" {}", database.database_name_wrapper(model))
        }
    })
}

#[derive(Debug, Clone)]
enum AssetType {
    Model,
    Seed,
    Source,
    Snapshot,
}

#[derive(Debug, Clone)]
enum AssetData {
    Model(Model),
    Seed(Seed),
    Snapshot(Snapshot),
    Source(Source),
    Override((String, String)),
}

#[derive(Debug, Clone)]
struct NodeWithName {
    name: String,
    asset: AssetData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_bigquery::DatabaseQueryGeneratorBigQuery;
    use crate::database_sqlite::DatabaseQueryGeneratorSqlite;
    use crate::project::parse_project;
    use quary_proto::{ConnectionConfig, Seed};
    use std::collections::HashMap;

    #[test]
    fn test_render_seed_select_statement_string() {
        let headers = vec!["id".to_string(), "name".to_string()];
        let values = vec![
            vec!["1".to_string(), "Bob's Burger".to_string()],
            vec!["2".to_string(), "Sally".to_string()],
        ];
        let expected =
            "SELECT column1 AS id,column2 AS name FROM (VALUES (\'1\',\'Bob\'\'s Burger\'),(\'2\',\'Sally\'))";
        let database = DatabaseQueryGeneratorSqlite::default();
        let actual = render_seed_select_statement_string(&database, headers, values);
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_render_seed_select_statement() {
        let fs = quary_proto::FileSystem {
            files: vec![(
                "test.csv".to_string(),
                quary_proto::File {
                    name: "test.csv".to_string(),
                    contents: prost::bytes::Bytes::from("id,name\n1,Bob\n2,Sally".to_string()),
                },
            )]
            .into_iter()
            .collect::<HashMap<String, quary_proto::File>>(),
        };

        let seed = Seed {
            name: "test".to_string(),
            file_path: "test.csv".to_string(),
            file_sha256_hash: "test".to_string(),
        };

        let expected =
            "SELECT column1 AS id,column2 AS name FROM (VALUES (\'1\',\'Bob\'),(\'2\',\'Sally\'))";
        let database = DatabaseQueryGeneratorSqlite::default();
        let actual = render_seed_select_statement(&database, &fs, &seed)
            .await
            .unwrap();
        assert_eq!(expected, actual);
    }
    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_sqlite_simple_model_source() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) = project_and_fs_to_query_sql(&database, &project, &fs, "shifts", None)
            .await
            .unwrap();

        assert_eq!(sql, "WITH raw_shifts AS (SELECT * FROM raw_shifts_real_table) SELECT * FROM (SELECT employee_id, shift_date, shift FROM `raw_shifts`) AS alias");
    }

    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_sqlite_simple_model_model_source() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/shifts_transformed.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts_transformed.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.shifts"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) =
            project_and_fs_to_query_sql(&database, &project, &fs, "shifts_transformed", None)
                .await
                .unwrap();

        assert_eq!(sql, "WITH\nraw_shifts AS (SELECT * FROM raw_shifts_real_table),\nshifts AS (SELECT employee_id, shift_date, shift FROM `raw_shifts`)\nSELECT * FROM (SELECT * FROM `shifts`) AS alias");
    }

    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_sqlite_simple_model_model_source_with_overides() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/shifts_transformed.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts_transformed.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.shifts"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) = project_and_fs_to_query_sql(
            &database,
            &project,
            &fs,
            "shifts_transformed",
            Some(HashMap::from([
                ("shifts".to_string(), "qqq_shifts_hash".to_string()),
                (
                    "doesntexist".to_string(),
                    "qqq_doesntexist_hash".to_string(),
                ),
            ])),
        )
        .await
        .unwrap();

        assert_eq!(
            sql,
            "WITH shifts AS (SELECT * FROM qqq_shifts_hash) SELECT * FROM (SELECT * FROM `shifts`) AS alias"
        );
    }

    #[tokio::test]
    async fn project_and_fs_to_query_sql_with_overrides_in_middle() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/prs_time_to_merge.sql".to_string(),
                    quary_proto::File {
                        name: "models/prs_time_to_merge.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT merged.id
FROM
    q.stg_pull_requests AS prs
INNER JOIN
    q.prs_merged AS merged
    ON
        prs.id = merged.id
",
                        ),
                    },
                ),
                (
                    "models/prs_merged.sql".to_string(),
                    quary_proto::File {
                        name: "models/prs_merged.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT prs.id
FROM
    q.stg_pull_requests AS prs
WHERE
    prs.merged_at IS NOT NULL
",
                        ),
                    },
                ),
                (
                    "models/stg_pull_requests.sql".to_string(),
                    quary_proto::File {
                        name: "models/stg_pull_requests.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT
    active_lock_reason AS active_lock_reason,
    assignee AS gh_assignee,
    assignees AS assignees,
    author_association AS author_association,
    auto_merge AS auto_merge,
    base AS base,
    body AS body,
    comments_url AS comments_url,
    commits_url AS commits_url,
    diff_url AS diff_url,
    draft AS draft,
    head AS head,
    html_url AS html_url,
    id AS id,
    issue_url AS issue_url,
    labels AS labels,
    locked AS locked,
    merge_commit_sha AS merge_commit_sha,
    milestone AS milestone,
    node_id AS node_id,
    number AS number,
    patch_url AS patch_url,
    repository AS repository,
    requested_reviewers AS requested_reviewers,
    requested_teams AS requested_teams,
    review_comment_url AS review_comment_url,
    review_comments_url AS review_comments_url,
    state AS state,
    statuses_url AS statuses_url,
    title AS title,
    url AS url,
    user AS user_name,
    TIMESTAMP(closed_at) AS closed_at,
    TIMESTAMP(created_at) AS created_at,
    TIMESTAMP(merged_at) AS merged_at,
    TIMESTAMP(updated_at) AS updated_at
FROM
    q.raw_pull_requests
",
                        ),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "
sources:
  - name: raw_pull_requests
    path: source.data.raw_pull_requests_real_table
",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) = project_and_fs_to_query_sql(
            &database,
            &project,
            &fs,
            "prs_time_to_merge",
            Some(HashMap::from([
                (
                    "prs_merged".to_string(),
                    "dataset.transform.qqq_prs_merged_88c7f00".to_string(),
                ),
                (
                    "stg_pull_requests".to_string(),
                    "dataset.transform.qqq_stg_pull_requests_d765fa9".to_string(),
                ),
            ])),
        )
        .await
        .unwrap();

        // Assert is one of the two possibilities
        // TODO Make this deterministic
        let possibility_1 = "WITH\nprs_merged AS (SELECT * FROM dataset.transform.qqq_prs_merged_88c7f00),\nstg_pull_requests AS (SELECT * FROM dataset.transform.qqq_stg_pull_requests_d765fa9)\nSELECT * FROM (SELECT merged.id\nFROM\n    `stg_pull_requests` AS prs\nINNER JOIN\n    `prs_merged` AS merged\n    ON\n        prs.id = merged.id\n) AS alias";
        let possibility_2 = "WITH\nstg_pull_requests AS (SELECT * FROM dataset.transform.qqq_stg_pull_requests_d765fa9),\nprs_merged AS (SELECT * FROM dataset.transform.qqq_prs_merged_88c7f00)\nSELECT * FROM (SELECT merged.id\nFROM\n    `stg_pull_requests` AS prs\nINNER JOIN\n    `prs_merged` AS merged\n    ON\n        prs.id = merged.id\n) AS alias";

        assert!(sql == possibility_1 || sql == possibility_2);
    }

    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_sqlite_simple_model_model_source_with_overide_end() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/shifts_transformed.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts_transformed.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.shifts"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) = project_and_fs_to_query_sql(
            &database,
            &project,
            &fs,
            "shifts_transformed",
            Some(HashMap::from([(
                "shifts_transformed".to_string(),
                "qqq_shifts_transformed_hash".to_string(),
            )])),
        )
        .await
        .unwrap();

        assert_eq!(sql, "SELECT * FROM qqq_shifts_transformed_hash");
    }

    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_big_query_simple_model_source() {
        let database = DatabaseQueryGeneratorBigQuery::new(
            "test-project".to_string(),
            "test-dataset".to_string(),
        );
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: test-project.test-dataset-2.raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) = project_and_fs_to_query_sql(&database, &project, &fs, "shifts", None)
            .await
            .unwrap();

        // TODO: figure out if we should also use backticks for the table name here
        assert_eq!(sql, "WITH raw_shifts AS (SELECT * FROM test-project.test-dataset-2.raw_shifts_real_table) SELECT * FROM (SELECT employee_id, shift_date, shift FROM `raw_shifts`) AS alias");
    }

    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_big_query_simple_model_model_source() {
        let database = DatabaseQueryGeneratorBigQuery::new(
            "test-project".to_string(),
            "test-dataset".to_string(),
        );
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/shifts_transformed.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts_transformed.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.shifts"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: test-project.test-dataset-2.raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) =
            project_and_fs_to_query_sql(&database, &project, &fs, "shifts_transformed", None)
                .await
                .unwrap();

        assert_eq!(sql, "WITH\nraw_shifts AS (SELECT * FROM test-project.test-dataset-2.raw_shifts_real_table),\nshifts AS (SELECT employee_id, shift_date, shift FROM `raw_shifts`)\nSELECT * FROM (SELECT * FROM `shifts`) AS alias");
    }

    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_big_query_simple_model_model_source_with_override() {
        let database = DatabaseQueryGeneratorBigQuery::new(
            "test-project".to_string(),
            "test-dataset".to_string(),
        );
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/shifts_transformed.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts_transformed.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.shifts"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: test-project.test-dataset-2.raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let project = parse_project(&fs, &database, "").await.unwrap();

        let (sql, _) = project_and_fs_to_query_sql(
            &database,
            &project,
            &fs,
            "shifts_transformed",
            Some(HashMap::from([
                (
                    "shifts".to_string(),
                    "test-project.test-dataset-2.qqq_shifts_hash".to_string(),
                ),
                (
                    "doesntexist".to_string(),
                    "qqq_doesntexist_hash".to_string(),
                ),
            ])),
        )
        .await
        .unwrap();

        assert_eq!(sql, "WITH shifts AS (SELECT * FROM test-project.test-dataset-2.qqq_shifts_hash) SELECT * FROM (SELECT * FROM `shifts`) AS alias");
    }

    #[tokio::test]
    async fn test_project_and_fs_to_sql_for_views_big_query() {
        let database =
            DatabaseQueryGeneratorBigQuery::new("quarylabs".to_string(), "transform".to_string());
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/stg_commits.sql".to_string(),
                    quary_proto::File {
                        name: "models/stg_commits.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT author FROM q.raw_commits",
                        ),
                    },
                ),
                (
                    "models/commits_transformed.sql".to_string(),
                    quary_proto::File {
                        name: "models/commits_transformed.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.stg_commits"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_commits, path: quarylabs.airbyte_github.commits}]",
                        ),
                    },
                ),
            ]),
        };
        let expected_output = vec![
            (
                "stg_commits".to_string(),
                vec![
                    "DROP VIEW IF EXISTS `quarylabs.transform.stg_commits`".to_string(),
                    "CREATE VIEW `quarylabs.transform.stg_commits` AS SELECT author FROM `quarylabs.airbyte_github.commits`".to_string()
                ]
            ),
            (
                "commits_transformed".to_string(),
                vec![
                    "DROP VIEW IF EXISTS `quarylabs.transform.commits_transformed`".to_string(),
                    "CREATE VIEW `quarylabs.transform.commits_transformed` AS SELECT * FROM `quarylabs.transform.stg_commits`".to_string()
                ]
            )
        ];

        let project = parse_project(&fs, &database, "").await.unwrap();
        let sql = project_and_fs_to_sql_for_views(&project, &fs, &database, false, false)
            .await
            .unwrap();

        assert_eq!(sql, expected_output)
    }

    #[tokio::test]
    async fn test_project_and_fs_to_sql_for_views_sqlite() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/stg_shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/stg_shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "SELECT employee_id, shift_date, shift FROM q.raw_shifts",
                        ),
                    },
                ),
                (
                    "models/shifts_transformed.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts_transformed.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.stg_shifts"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };

        let expected_output = vec![
            (
                "stg_shifts".to_string(),
                vec![
                    "DROP VIEW IF EXISTS `stg_shifts`".to_string(),
                    "CREATE VIEW `stg_shifts` AS SELECT employee_id, shift_date, shift FROM `raw_shifts_real_table`".to_string()
                ]
            ),
            (
                "shifts_transformed".to_string(),
                vec![
                    "DROP VIEW IF EXISTS `shifts_transformed`".to_string(),
                    "CREATE VIEW `shifts_transformed` AS SELECT * FROM `stg_shifts`".to_string()
                ]
            )
        ];

        let project = parse_project(&fs, &database, "").await.unwrap();
        let sql = project_and_fs_to_sql_for_views(&project, &fs, &database, false, false)
            .await
            .unwrap();

        assert_eq!(sql, expected_output)
    }

    #[test]
    fn test_replace_variable_with_config_variables() {
        let connection_config = ConnectionConfig {
            config: Default::default(),
            vars: vec![
                quary_proto::Var {
                    name: "test".to_string(),
                    value: "value1".to_string(),
                },
                quary_proto::Var {
                    name: "var2".to_string(),
                    value: "value2".to_string(),
                },
            ],
        };

        let sql = "SELECT
        {{ var('test') }} as test_var,
        'morning' AS shift,
        '08:00:00' AS start_time,
        '12:00:00' AS end_time
    UNION ALL
    SELECT
        'afternoon' AS shift,
        '12:00:00' AS start_time,
        '16:00:00' AS end_time
    ";
        let result =
            replace_variable_templates_with_variable_defined_in_config(sql, &connection_config)
                .unwrap();

        assert_eq!(
            result,
            "SELECT\n        value1 as test_var,\n        'morning' AS shift,\n        '08:00:00' AS start_time,\n        '12:00:00' AS end_time\n    UNION ALL\n    SELECT\n        'afternoon' AS shift,\n        '12:00:00' AS start_time,\n        '16:00:00' AS end_time\n    "
        );
    }

    // TODO Reinstate after making get_node_sorted completely deterministic
    #[tokio::test]
    #[ignore]
    async fn test_project_and_fs_to_sql_for_views() {
        let assets = crate::init::Asset {};
        let database = DatabaseQueryGeneratorSqlite::default();
        let project = parse_project(&assets, &database, "").await.unwrap();

        let sql = project_and_fs_to_query_sql(&database, &project, &assets, "stg_shifts", None)
            .await
            .unwrap();
        assert_eq!(
            sql.0,
            "WITH raw_shifts AS (SELECT column1 AS employee_id,column2 AS shop_id,column3 AS date,column4 AS shift FROM (VALUES ('1','2','2023-01-01','morning'),('1','2','2023-01-02','morning'),('1','2','2023-01-03','morning'),('1','2','2023-01-04','morning'),('1','2','2023-01-05','morning'),('1','2','2023-01-06','morning'),('1','2','2023-01-07','morning'),('1','2','2023-01-08','morning'),('1','2','2023-01-09','morning'),('1','2','2023-01-10','morning'),('1','2','2023-01-11','morning'),('1','2','2023-01-12','morning'),('1','2','2023-01-13','morning'),('1','2','2023-01-13','afternoon'))) select\n  employee_id,\n  shop_id,\n  date as shift_date,\n  shift\nfrom\n  raw_shifts\n"
        );

        let sql = project_and_fs_to_query_sql(&database, &project, &assets, "shifts_summary", None)
            .await
            .unwrap();
        assert_eq!(
            sql.0,
            "WITH\nraw_employees AS (SELECT column1 AS id,column2 AS first_name,column3 AS last_name FROM (VALUES ('1','John','Doe'),('2','Jane','Doe'),('3','Ashok','Kumar'),('4','Peter','Pan'),('5','Marie','Curie'))),\nraw_shifts AS (SELECT column1 AS employee_id,column2 AS shop_id,column3 AS date,column4 AS shift FROM (VALUES ('1','2','2023-01-01','morning'),('1','2','2023-01-02','morning'),('1','2','2023-01-03','morning'),('1','2','2023-01-04','morning'),('1','2','2023-01-05','morning'),('1','2','2023-01-06','morning'),('1','2','2023-01-07','morning'),('1','2','2023-01-08','morning'),('1','2','2023-01-09','morning'),('1','2','2023-01-10','morning'),('1','2','2023-01-11','morning'),('1','2','2023-01-12','morning'),('1','2','2023-01-13','morning'),('1','2','2023-01-13','afternoon'))),\nshift_hours AS (SELECT 'morning'  AS shift,\n       '08:00:00' AS start_time,\n       '12:00:00' AS end_time\nUNION ALL\nSELECT 'afternoon' AS shift,\n       '12:00:00'  AS start_time,\n       '16:00:00'  AS end_time),\nshift_first AS (WITH\n  min_shifts AS (\n    SELECT\n      employee_id,\n      MIN(shift_start) AS shift_start\n    FROM\n      shifts\n    GROUP BY\n      employee_id\n  )\nSELECT\n  x.employee_id AS employee_id,\n  x.shift_start AS shift_start,\n  x.shift_end AS shift_end\nFROM\n  shifts x\n  INNER JOIN min_shifts y ON y.employee_id = x.employee_id\n  AND y.shift_start = x.shift_start\nGROUP BY\n  x.employee_id,\n  x.shift_start\n),\nshift_last AS (WITH min_shifts AS (SELECT employee_id,\n                           max(shift_start) AS shift_start\n                    FROM shifts\n                    GROUP BY employee_id)\n\nSELECT x.employee_id AS employee_id,\n       x.shift_start AS shift_start,\n       x.shift_end AS shift_end\nFROM shifts x\nINNER JOIN min_shifts y\nON y.employee_id = x.employee_id AND y.shift_start = x.shift_start\nGROUP BY x.employee_id, x.shift_start),\nstg_employees AS (select\n  id as employee_id,\n  first_name,\n  last_name\nfrom\n  raw_employees\n),\nstg_shifts AS (select\n  employee_id,\n  shop_id,\n  date as shift_date,\n  shift\nfrom\n  raw_shifts\n),\nshifts AS (WITH shifts AS (SELECT employee_id,\n                       shift_date,\n                       shift\n                FROM stg_shifts\n                ),\n     shift_details AS (SELECT shift AS shift_name,\n                              start_time,\n                              end_time\n                       FROM shift_hours\n                       )\n\nSELECT s.employee_id AS employee_id,\n       s.shift AS shift,\n       datetime(s.shift_date, sd.start_time) AS shift_start,\n       datetime(s.shift_date, sd.end_time)   AS shift_end\nFROM shifts s\n         INNER JOIN shift_details sd\n                    ON s.shift = sd.shift_name\n)\nSELECT * FROM (WITH total_hours AS (\n    SELECT employee_id,\n           SUM(strftime('%s', shift_end) - strftime('%s', shift_start)) AS total_hours,\n           COUNT(*) AS total_shifts\n    FROM shifts\n    GROUP BY employee_id\n),\n\npercentage_morning_shifts AS (\n    SELECT employee_id,\n           SUM(CASE WHEN shift = 'morning' THEN 1 ELSE 0 END) AS total_morning_shifts,\n          COUNT(*) AS total_shifts\n    FROM shifts\n    GROUP BY employee_id\n)\n\nSELECT e.employee_id,\n       e.first_name,\n       e.last_name,\n       sf.shift_start AS first_shift,\n       sl.shift_start AS last_shift,\n       pms.total_morning_shifts / pms.total_shifts * 100 AS percentage_morning_shifts,\n       th.total_shifts,\n       th.total_hours\nFROM stg_employees e\nLEFT JOIN shift_first sf\n    ON e.employee_id = sf.employee_id\nLEFT JOIN shift_last sl\n    ON e.employee_id = sl.employee_id\nLEFT JOIN total_hours th\n    ON e.employee_id = th.employee_id\nLEFT JOIN percentage_morning_shifts pms\n    ON e.employee_id = pms.employee_id)"
        )
    }

    // project_and_fs_to_query_sql_for_model_sql is tested by essentially running it with an injected
    // filesystem and project and checking the output against project_and_fs_to_query_sql for a model
    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_for_model_sql() {
        // shared
        let model = "SELECT employee_id, shift_date, shift FROM q.raw_shifts";
        let project_root = "some_random_project_root/nested";
        let database = DatabaseQueryGeneratorSqlite::default();

        // generating sql
        let filesystem = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "some_random_project_root/nested/quary.yaml".to_string(),
                    quary_proto::File {
                        name: "projectquary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "some_random_project_root/nested/models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };
        let (sql, _) = project_and_fs_to_query_sql_for_model_sql(
            &database,
            &filesystem,
            project_root,
            model,
            None,
            "shift_hours.chart.sql", // this is the unique identifier for the injected model
        )
        .await
        .unwrap();

        // generating expected sql
        let filesystem = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "some_random_project_root/nested/quary.yaml".to_string(),
                    quary_proto::File {
                        name: "projectquary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "some_random_project_root/nested/models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
                (
                    "some_random_project_root/nested/models/shifts.sql".to_string(),
                    quary_proto::File {
                        name: "models/shifts.sql".to_string(),
                        contents: prost::bytes::Bytes::from(model.as_bytes()),
                    },
                ),
            ]),
        };
        let project = parse_project(&filesystem, &database, project_root)
            .await
            .unwrap();
        let (expected, _) =
            project_and_fs_to_query_sql(&database, &project, &filesystem, "shifts", None)
                .await
                .unwrap();

        assert_eq!(expected, sql);
        assert_eq!("WITH raw_shifts AS (SELECT * FROM raw_shifts_real_table) SELECT * FROM (SELECT employee_id, shift_date, shift FROM `raw_shifts`) AS alias", sql);
    }

    // This test checks that project_and_fs_to_query_sql_for_model_sql handles the case
    // where a model references a non-existent model, source, or snapshot.
    // It should return a useful error in this case.
    #[tokio::test]
    async fn test_project_and_fs_to_query_sql_for_model_sql_reference_error() {
        // shared
        let model = "SELECT employee_id, shift_date, shift FROM q.non_existent_model";
        let project_root = "some_random_project_root/nested";
        let database = DatabaseQueryGeneratorSqlite::default();

        // generating sql
        let filesystem = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "some_random_project_root/nested/quary.yaml".to_string(),
                    quary_proto::File {
                        name: "projectquary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "some_random_project_root/nested/models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
                        ),
                    },
                ),
            ]),
        };
        let result = project_and_fs_to_query_sql_for_model_sql(
            &database,
            &filesystem,
            project_root,
            model,
            None,
            "shift_hours.chart.sql", // this is the unique identifier for the injected model
        )
        .await;

        assert_eq!(
        result.unwrap_err().to_string(),
        "model \"shift_hours.chart.sql\" has reference to \"non_existent_model\" which is not a model, source or snapshot"
    );
    }
}
