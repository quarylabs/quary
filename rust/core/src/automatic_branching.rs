use crate::databases::DatabaseQueryGenerator;
use crate::file_system::FileSystem;
use crate::graph::{project_to_graph, ProjectGraph};
use data_encoding::HEXLOWER;
use futures::AsyncReadExt;
use petgraph::visit::IntoNodeIdentifiers;
use quary_proto::{Project, Source};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};

/// automatic branching is a strategy for investigating data in a database where some of the models and seeds
/// have already been applied. The idea is that if you have already built views, especially
/// materialized views, you are likely to want to further experiment on top of them and by relying
/// on already created views that are identical to those created in a previous run, the
/// experimentation will be faster.
///
/// Example:
/// Suppose you have a main version of the project which we will call 'main' which has been applied
/// to the database. The views have been created as materialized views to speed up any querying.
/// This version of the project contains an original table (T) two views (V1 & V2) that depend on
/// each other as following:
/// A -> V1 -> V2
///
/// Suppose now you want to alter V2 to V2' in a way that still maintains the dependencies:
/// A -> V1 -> V2'
/// You are essentially branching off of the 'main' version and experimenting on V2. While
/// experimenting, you probably want to rely on the materialized V1 to compare the results but also
/// for the performance boost. Automatic branching is a querying that relies on already applied versions
/// if and only if upstream views (V1 in this example) are identical in both branches.
///
///
/// The process works by taking the hash of the accumulated upstream dag and comparing it.
///
/// Need to think about the different permutations of this and see if can make this a clean problem.
/// Finding the upper most that matches, while also being able to move along down.
///
/// The way this works is to put inside of the database views with hashes that are the same as the
/// hash of the upstream views. This way, when you are querying, you can check if the hash of the
/// upstream views is the same as the hash of the views in the database. If it is, then you can use it.
///
/// The name of the upstream view is qqq_<model_name>_<hash _of_upstream_views> where the hash
/// is the first 7 digits of the sha256 hash of the upstream views.

/// derive_model_hash returns a hash of itself with everything up to that model such that if the
/// hashes of the model are identical, then you can now that it is the same model.
pub fn derive_model_hash(
    project: &Project,
    project_graph: &ProjectGraph,
    model: &str,
) -> Result<String, String> {
    let upstream_graph = project_graph.graph.return_upstream_graph(model)?;

    let model_or_seeds_in_upstream = upstream_graph
        .graph
        .node_identifiers()
        .map(|index| {
            upstream_graph
                .get_node_name(&index)
                .ok_or(format!("Model {:?} not found in upstream graph", index))
        })
        .collect::<Result<HashSet<_>, String>>()?;

    let model_hash_map = model_or_seeds_in_upstream
        .iter()
        .map(|model| {
            match (
                project.sources.get(model),
                project.seeds.get(model),
                project.models.get(model),
            ) {
                (Some(source), None, None) => Ok((source.name.clone(), hash_source(source))),
                (None, Some(seed), None) => Ok((seed.name.clone(), seed.file_sha256_hash.clone())),
                (None, None, Some(model)) => {
                    Ok((model.name.clone(), model.file_sha256_hash.clone()))
                }

                _ => Err(format!("Model or seed {:?} not found in project", model)),
            }
        })
        .collect::<Result<HashMap<_, _>, String>>()?;

    let hash_set = model_hash_map.values().cloned().collect::<HashSet<_>>();

    let mut vec: Vec<String> = hash_set.into_iter().collect();
    vec.sort();

    let hash_of_hashes = {
        let mut hasher = Sha256::new();
        for hash in vec {
            hasher.update(hash.as_bytes());
        }
        hasher.finalize()
    };
    Ok(HEXLOWER.encode(hash_of_hashes.as_ref()))
}

fn hash_source(source: &Source) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source.name.as_bytes());
    hasher.update(source.path.as_bytes());
    let finalised = hasher.finalize();
    HEXLOWER.encode(finalised.as_ref())
}

/// derive_sha256_file_contents returns the sha256 hash of the file contents.
pub async fn derive_sha256_file_contents(
    fs: &impl FileSystem,
    file_path: &str,
) -> Result<String, String> {
    let mut read = fs
        .read_file(file_path)
        .await
        .map_err(|e| format!("Error reading file contents for hash: {}", e))?;

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = read
                .read(&mut buffer)
                .await
                .map_err(|e| format!("Error reading file contents for hash: {}", e))?;
            if count == 0 {
                break;
            }
            hasher.update(
                buffer
                    .get(..count)
                    .ok_or("Error reading file contents for hash: buffer overflow".to_string())?,
            );
        }
        hasher.finalize()
    };

    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// derive_hash_views returns table names for every model
/// and seed inside of the project that is passed in. The returned type is a map from the model
/// to a tuple of the model hash and sql statements required to create the view.
pub fn derive_hash_views<'a>(
    database: &impl DatabaseQueryGenerator,
    project: &'a Project,
    project_graph: &'_ ProjectGraph,
) -> Result<BTreeMap<ModelName<'a>, (String, Vec<String>)>, String> {
    let all_models = project
        .models
        .keys()
        .chain(project.seeds.keys())
        .collect::<HashSet<_>>();
    all_models
        .into_iter()
        .map(|model| {
            let hash = derive_model_hash(project, project_graph, model)?;
            let name = create_automatic_branch_view_name(model, &hash)?;
            let sql = database.automatic_cache_sql_create_statement(model, &name);
            Ok((model.as_str(), (name, sql)))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()
}

/// create_automatic_branch_view_name creates a name for the view that is created for the
/// automatic branching. The name is qqq_<model_name>_<hash _of_upstream_views> where the hash
/// is the first 7 digits of the sha256 hash of the upstream views.
fn create_automatic_branch_view_name(model: &str, hash: &str) -> Result<String, String> {
    let shortened_hash = hash.get(0..7).ok_or(format!(
        "Hash {} is not long enough to create automatic branch view name",
        hash
    ))?;
    Ok(format!("qqq_{}_{}", model, shortened_hash))
}

/// is_cache_view_name returns true if the view name is a cache view name. This is used to
/// determine if the view name is a cache view name or not.
fn is_cache_view_name(view_name: &str) -> bool {
    view_name.starts_with("qqq_")
}

pub fn cache_view_name_to_table_name_and_hash(
    name: ModelWithHash,
) -> Result<(ModelNameString, ShortenedHash), String> {
    let split = name.split('_').collect::<Vec<_>>();
    match split.as_slice() {
        ["qqq", model_name @ .., hash] => {
            if model_name.is_empty() {
                return Err(format!(
                    "Cache view name {} is not in the correct format",
                    name
                ));
            }
            Ok((
                model_name
                    .iter()
                    .map(|model_name| model_name.to_string())
                    .collect::<Vec<_>>()
                    .join("_"),
                hash,
            ))
        }
        _ => Err(format!(
            "Cache view name {} is not in the correct format",
            name
        )),
    }
}

/// is_cache_full_path returns true if the full path is a cache view name.
pub fn is_cache_full_path(
    database: &impl DatabaseQueryGenerator,
    full_path: &str,
) -> Result<bool, String> {
    let view_name = database.return_name_from_full_path(full_path)?;
    Ok(is_cache_view_name(view_name))
}

/// drop_statement_for_cache_view returns a drop statement for the cache view. This is used to
/// drop the cache view before creating a new one.
///
/// ```
/// use crate::quary_core::automatic_branching::drop_statement_for_cache_view;
///
/// let path = "project_id.dataset_id.qqq_shifts_summary_fbas143";
/// let drop_statement = drop_statement_for_cache_view(path).unwrap();
///
/// assert_eq!(drop_statement, "DROP VIEW IF EXISTS project_id.dataset_id.qqq_shifts_summary_fbas143");
/// ```
pub fn drop_statement_for_cache_view(path: &str) -> Result<String, String> {
    Ok(format!("DROP VIEW IF EXISTS {}", path))
}

/// ModelWithHash is a type alias for the model name with the hash of the upstream views. The format
/// is qqq_<model_name>_<hash _of_upstream_views> where the hash is the first 7 digits of the sha256.
/// TODO This could be much improved by using a struct instead of a string and having methods on it.
pub type ModelWithHash<'a> = &'a str;

/// ModelName is just the model name in the project such as `shifts_summary`.
pub type ModelName<'a> = &'a str;
pub type ModelNameString = String;

pub type ShortenedHash<'a> = &'a str;

/// given_map_and_hash_map_return_sub_graph_all_cached takes a graph and a map of the model to the
/// shortened hashes and returns a set that includes the model names of the models that are cached.
///
/// The returned map is the model name to matched hashed model name
pub fn given_map_and_hash_map_return_sub_graph_all_cached_for_a_particular_model(
    project: Project,
    model: &str,
    map: &HashMap<ModelName, ModelWithHash>,
) -> Result<HashMap<String, String>, String> {
    let graph = project_to_graph(project.clone())?;
    let upstream_graph = graph.graph.return_upstream_graph(model)?;
    let models_of_interest: HashSet<String> = upstream_graph
        .graph
        .node_identifiers()
        .map(|index| {
            upstream_graph
                .get_node_name(&index)
                .ok_or(format!("Model {:?} not found in upstream graph", index))
        })
        .collect::<Result<_, String>>()?;
    let models_of_interest_with_upstream_hash: HashMap<String, String> = models_of_interest
        .iter()
        .map(|model| {
            let upstream_hash = derive_model_hash(&project, &graph, model.as_str())?;
            Ok((model.clone(), upstream_hash))
        })
        .collect::<Result<_, String>>()?;
    let map = models_of_interest_with_upstream_hash
        .into_iter()
        .map(|(model, upstream_hash)| {
            let shortened_hash = upstream_hash.get(0..7).ok_or(format!(
                "Hash {} is not long enough to create automatic branch view name",
                upstream_hash
            ))?;
            let model_name_with_hash = map.get(model.as_str());
            if let Some(model_name_with_hash) = model_name_with_hash {
                let model_hash = model_name_with_hash
                    .get(model_name_with_hash.len() - 7..)
                    .ok_or(
                        format!(
                            "Hash {} is not long enough to create automatic branch view name",
                            model_name_with_hash
                        )
                        .as_str(),
                    )?;
                if shortened_hash == model_hash {
                    Ok(Some((
                        model.clone(),
                        create_automatic_branch_view_name(model.as_str(), model_hash)?,
                    )))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<Option<(String, String)>>, String>>()?
        .into_iter()
        .flatten()
        .collect::<HashMap<String, String>>();
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_bigquery::DatabaseQueryGeneratorBigQuery;
    use crate::database_sqlite::DatabaseQueryGeneratorSqlite;
    use crate::file_system::convert_async_read_to_blocking_read;
    use crate::init::{init_to_file_system, Asset};
    use crate::project::parse_project;
    use crate::project_file::{deserialize_project_file_from_yaml, serialize_project_file_to_yaml};
    use quary_proto::{File, Seed};
    use quary_proto::{FileSystem as ProtoFileSystem, ProjectFileColumn};

    #[tokio::test]
    /// test_derive_sha256_contents_of_init_compare_to_web_values tests that the sha256 hash of the
    /// matches the values extracted from running pnpm run dev:extension and when runnning compared
    /// to the init assets locally.
    async fn test_derive_sha256_contents_of_init_compare_to_web_values() {
        let assets = init_to_file_system();

        let database = DatabaseQueryGeneratorSqlite {};

        let project = parse_project(&assets, &database, "").await.unwrap();

        let model_to_check = "shifts_summary";
        let model = project.models.get(model_to_check).unwrap();

        let value_want = "182517b781abdd14cfca64a1e8368971e2853ca3eae51a89647f6322bc52787b";
        assert_eq!(model.file_sha256_hash, value_want);

        let file_content_hash = derive_sha256_file_contents(&assets, &model.file_path)
            .await
            .unwrap();
        assert_eq!(file_content_hash, value_want);
    }

    const MODEL_OF_INTEREST: &str = "shifts_summary";
    /// get_hash_derive_model_hash is a helper function for testing the derive_model_hash function.
    async fn get_hash_derive_model_hash(fs: &impl FileSystem) -> String {
        let database = DatabaseQueryGeneratorSqlite {};
        let project = parse_project(fs, &database, "").await.unwrap();
        let graph = project_to_graph(project.clone()).unwrap();
        derive_model_hash(&project, &graph, MODEL_OF_INTEREST).unwrap()
    }

    /// test_derive_model_hash tests that the model hash is derived correctly. It compares before
    /// and after certain changes where you would expect the hash to change and some where you would
    /// not.
    /// TODO Finish these tests
    /// Scenarios it tests by looking at 'shifts_summary':
    /// 8. Compare 2 seeds with change -> Change
    /// TODO NEED TO TEST SOURCES AS WELL
    #[tokio::test]
    async fn test_derive_model_hash_different_fs() {
        let fs = init_to_file_system();
        let hash_0 = get_hash_derive_model_hash(&fs).await;
        let fs = Asset;
        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_same_fs() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;
        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_add_model() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;
        let mut fs = init_to_file_system();
        fs.files.insert(
            "models/shifts_summary_2.sql".to_string(),
            File {
                name: "models/shifts_summary_2.sql".to_string(),
                contents: prost::bytes::Bytes::from(
                    r#"
                        SELECT
                            *
                        FROM
                            q.shifts_summary
                    "#
                    .to_string(),
                ),
            },
        );
        fs.files.insert(
            "models/schema_2.yaml".to_string(),
            File {
                name: "models/schema_2.yaml".to_string(),
                contents: prost::bytes::Bytes::from(
                    r#"
                        models: 
                            - name: shifts_summary_2
                              description: test description for model
                    "#
                    .to_string(),
                ),
            },
        );

        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_change_project_file_for_intermediary_model() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;

        let mut fs = init_to_file_system();
        let reader = fs.read_file("models/staging/schema.yaml").await.unwrap();
        let reader = convert_async_read_to_blocking_read(reader).await;

        let mut staging_schema = deserialize_project_file_from_yaml(reader).unwrap();
        staging_schema.models.iter_mut().for_each(|model| {
            if model.name == "stg_employees" {
                model.columns.push(ProjectFileColumn {
                    name: "doesnt exist".to_string(),
                    description: None,
                    tests: vec![],
                })
            }
        });

        fs.files.insert(
            "models/staging/schema.yaml".to_string(),
            File {
                name: "models/staging/schema.yaml".to_string(),
                contents: prost::bytes::Bytes::from(
                    serialize_project_file_to_yaml(staging_schema).unwrap(),
                ),
            },
        );

        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_add_sql_test_on_end_model() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;

        let mut fs = init_to_file_system();
        fs.files.insert(
            "tests/shifts_summary_test.sql".to_string(),
            File {
                name: "tests/shifts_summary_test.sql".to_string(),
                contents: prost::bytes::Bytes::from(
                    r#"
                        SELECT
                            *
                        FROM
                            q.shifts_summary
                    "#
                    .to_string(),
                ),
            },
        );

        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_add_sql_test_on_in_between_model() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;

        let mut fs = init_to_file_system();
        fs.files.insert(
            "tests/shifts_summary_test.sql".to_string(),
            File {
                name: "tests/shifts_summary_test.sql".to_string(),
                contents: prost::bytes::Bytes::from(
                    r#"
                        SELECT
                            *
                        FROM
                            q.stg_employees
                    "#
                    .to_string(),
                ),
            },
        );

        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_change_underlying_model() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;

        let mut fs = init_to_file_system();
        let mut raw_file = fs
            .read_file("models/staging/stg_employees.sql")
            .await
            .unwrap();
        let mut contents = String::new();
        raw_file.read_to_string(&mut contents).await.unwrap();

        let contents = contents.replace(' ', "  ");
        fs.files.insert(
            "models/staging/stg_employees.sql".to_string(),
            File {
                name: "models/staging/stg_employees.sql".to_string(),
                contents: prost::bytes::Bytes::from(contents),
            },
        );

        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_ne!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_change_model_itself() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;

        let path_of_interest = format!("models/{}.sql", MODEL_OF_INTEREST);

        let mut fs = init_to_file_system();
        let mut raw_file = fs.read_file(&path_of_interest).await.unwrap();
        let mut contents = String::new();
        raw_file.read_to_string(&mut contents).await.unwrap();

        let contents = contents.replace(' ', "  ");
        fs.files.insert(
            path_of_interest.clone(),
            File {
                name: path_of_interest.clone(),
                contents: prost::bytes::Bytes::from(contents),
            },
        );

        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_ne!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_change_underlying_seed() {
        let fs = Asset;
        let hash_0 = get_hash_derive_model_hash(&fs).await;

        let file_of_interest = "seeds/raw_employees.csv";
        let mut fs = init_to_file_system();
        let mut raw_file = fs.read_file(file_of_interest).await.unwrap();
        let mut contents = String::new();
        raw_file.read_to_string(&mut contents).await.unwrap();

        let mut lines: Vec<&str> = contents.split('\n').collect();
        lines.push(lines.last().unwrap());
        let new_csv_string: String = lines.join("\n");

        fs.files.insert(
            file_of_interest.to_string(),
            File {
                name: file_of_interest.to_string(),
                contents: prost::bytes::Bytes::from(new_csv_string),
            },
        );

        let hash_1 = get_hash_derive_model_hash(&fs).await;

        assert_ne!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_for_seed_no_change() {
        let seed_of_interest = "raw_employees";

        let fs = Asset;
        let project = parse_project(&fs, &DatabaseQueryGeneratorSqlite {}, "")
            .await
            .unwrap();
        let graph = project_to_graph(project.clone()).unwrap();

        let hash_0 = derive_model_hash(&project, &graph, seed_of_interest).unwrap();
        let hash_1 = derive_model_hash(&project, &graph, seed_of_interest).unwrap();

        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_for_seed_changed_seed() {
        let seed_of_interest = "raw_employees";

        let fs = Asset;
        let project = parse_project(&fs, &DatabaseQueryGeneratorSqlite {}, "")
            .await
            .unwrap();
        let graph = project_to_graph(project.clone()).unwrap();

        let hash_0 = derive_model_hash(&project, &graph, seed_of_interest).unwrap();

        let file_of_interest = "seeds/raw_employees.csv";
        let mut fs = init_to_file_system();
        let mut raw_file = fs.read_file(file_of_interest).await.unwrap();
        let mut contents = String::new();
        raw_file.read_to_string(&mut contents).await.unwrap();

        let mut lines: Vec<&str> = contents.split('\n').collect();
        lines.push(lines.last().unwrap());
        let new_csv_string: String = lines.join("\n");

        fs.files.insert(
            file_of_interest.to_string(),
            File {
                name: file_of_interest.to_string(),
                contents: prost::bytes::Bytes::from(new_csv_string),
            },
        );
        let project = parse_project(&fs, &DatabaseQueryGeneratorSqlite {}, "")
            .await
            .unwrap();
        let graph = project_to_graph(project.clone()).unwrap();
        let hash_1 = derive_model_hash(&project, &graph, seed_of_interest).unwrap();

        assert_ne!(hash_0, hash_1);
    }

    fn file_system_for_project_with_source() -> (String, ProtoFileSystem) {
        let mut fs = init_to_file_system();
        fs.files.insert(
            "models/sources.yaml".to_string(),
            File {
                name: "models/sources.yaml".to_string(),
                contents: prost::bytes::Bytes::from(
                    r#"
sources:
    - name: employees_sources
      path: employees.csv
                    "#
                    .to_string(),
                ),
            },
        );
        fs.files.insert(
            "models/model_with_source.sql".to_string(),
            File {
                name: "models/model_with_source.sql".to_string(),
                contents: prost::bytes::Bytes::from(
                    r#"
                        SELECT
                            *
                        FROM
                            q.employees_sources
                    "#
                    .to_string(),
                ),
            },
        );
        (
            "model_with_source".to_string(),
            ProtoFileSystem { files: fs.files },
        )
    }

    #[tokio::test]
    async fn test_derive_model_hash_for_model_with_source_dont_change() {
        let (model, fs) = file_system_for_project_with_source();
        let project = parse_project(&fs, &DatabaseQueryGeneratorSqlite {}, "")
            .await
            .unwrap();
        let graph = project_to_graph(project.clone()).unwrap();
        let hash_0 = derive_model_hash(&project, &graph, &model).unwrap();
        let hash_1 = derive_model_hash(&project, &graph, &model).unwrap();
        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_for_model_with_source_that_changes() {
        let (model, fs) = file_system_for_project_with_source();
        let project = parse_project(&fs, &DatabaseQueryGeneratorSqlite {}, "")
            .await
            .unwrap();
        let graph = project_to_graph(project.clone()).unwrap();
        let hash_0 = derive_model_hash(&project, &graph, &model).unwrap();

        let mut fs = fs;
        fs.files.insert(
            "models/sources.yaml".to_string(),
            File {
                name: "models/sources.yaml".to_string(),
                contents: prost::bytes::Bytes::from(
                    r#"
sources:
  - name: employees_sources
    path: employees_2.csv
                        "#
                    .to_string(),
                ),
            },
        );

        let project = parse_project(&fs, &DatabaseQueryGeneratorSqlite {}, "")
            .await
            .unwrap();
        let graph = project_to_graph(project.clone()).unwrap();
        let hash_1 = derive_model_hash(&project, &graph, &model).unwrap();

        assert_ne!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_model_hash_for_source() {
        let (_, fs) = file_system_for_project_with_source();
        let project = parse_project(&fs, &DatabaseQueryGeneratorSqlite {}, "")
            .await
            .unwrap();
        let graph = project_to_graph(project.clone()).unwrap();

        let model = "employees_sources";
        let hash_0 = derive_model_hash(&project, &graph, model).unwrap();
        let hash_1 = derive_model_hash(&project, &graph, model).unwrap();
        assert_eq!(hash_0, hash_1);
    }

    #[tokio::test]
    async fn test_derive_hashes() {
        let fs = Asset;
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_id".to_string(), "dataset_id".to_string());
        let project = parse_project(&fs, &database, "").await.unwrap();
        let graph = project_to_graph(project.clone()).unwrap();

        let hashes = derive_hash_views(&database, &project, &graph).unwrap();

        assert_eq!(hashes.len(), project.seeds.len() + project.models.len());
        assert!(hashes.contains_key("shifts_summary"));
        let unpacked = hashes.get("shifts_summary").unwrap();
        assert_eq!(unpacked.0, "qqq_shifts_summary_e694e97");
        assert_eq!(
            unpacked.1,
            vec!["CREATE OR REPLACE VIEW project_id.dataset_id.qqq_shifts_summary_e694e97 AS SELECT * FROM project_id.dataset_id.shifts_summary".to_string()]
        );
    }

    #[tokio::test]
    async fn test_is_cache_full_path_sqlite() {
        let fs = Asset;
        let database = DatabaseQueryGeneratorSqlite {};
        let project = parse_project(&fs, &database, "").await.unwrap();
        let graph = project_to_graph(project.clone()).unwrap();

        let derived_hash = derive_hash_views(&database, &project, &graph).unwrap();
        derived_hash.iter().for_each(|(_, (view_name, _))| {
            let full_path = database.return_full_path_requirement(view_name);

            let is_cache = is_cache_full_path(&database, &full_path).unwrap();
            assert!(is_cache);
        });
    }

    #[tokio::test]
    async fn test_is_cache_full_path_bigquery() {
        let fs = Asset;
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_id".to_string(), "dataset_id".to_string());
        let project = parse_project(&fs, &database, "").await.unwrap();
        let graph = project_to_graph(project.clone()).unwrap();

        let derived_hash = derive_hash_views(&database, &project, &graph).unwrap();
        derived_hash.iter().for_each(|(_, (view_name, _))| {
            let full_path = database.return_full_path_requirement(view_name);

            let is_cache = is_cache_full_path(&database, &full_path).unwrap();
            assert!(is_cache);
        })
    }

    #[test]
    fn test_create_automatic_branch_view_name() {
        let model = "shifts_summary";
        let hash = "5643694d72f848a4f938557cb806a24760df5ea5db6cccfffa1bce3c61e9fec6";
        let view_name = create_automatic_branch_view_name(model, hash).unwrap();

        assert_eq!(view_name, "qqq_shifts_summary_5643694");
    }

    /// tests multiple bits of functionality together: seeds, sources, models, and automatic branching
    /// with a tree that looks like following:
    ///
    /// ```mermaid
    /// flowchart TD
    ///     seed_1 --> model_1
    ///     source_1 --> model_1
    ///     seed_2 --> model_2
    ///     source_2 --> model_2
    ///     seed_3 --> model_3
    ///     source_3 --> model_3
    ///     model_1 --> model_4
    ///     model_2 --> model_4
    ///     model_3 --> model_4    
    /// ```
    ///
    /// and in the cache,
    /// - seed_1, source_1, model_1 match
    /// - seed_2, source_2, model_2 do not match
    /// - seed_3, source_3, model_3 are not present
    #[test]
    fn given_map_and_hash_map_return_sub_graph_all_cached_for_a_particular_model_encompassing_example(
    ) {
        let project = Project {
            seeds: HashMap::from([
                (
                    "seed_1".to_string(),
                    Seed {
                        name: "seed_1".to_string(),
                        file_path: "seeds/seed_1.csv".to_string(),
                        file_sha256_hash:
                            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
                                .to_string(),
                    },
                ),
                (
                    "seed_2".to_string(),
                    Seed {
                        name: "seed_2".to_string(),
                        file_path: "seeds/seed_2.csv".to_string(),
                        file_sha256_hash:
                            "d301fcd0b7c84c879456eb041af246fbc7edbfea54f6470a859d8bd4073a47b8"
                                .to_string(),
                    },
                ),
                (
                    "seed_3".to_string(),
                    Seed {
                        name: "seed_3".to_string(),
                        file_path: "seeds/seed_3.csv".to_string(),
                        file_sha256_hash:
                            "61f0b5d712957b23dced53604c9a76047f0348be2e1a4af0417b8bb3bcd56605"
                                .to_string(),
                    },
                ),
            ]),
            models: HashMap::from([
                (
                    "model_1".to_string(),
                    quary_proto::Model {
                        name: "model_1".to_string(),
                        file_path: "models/model_1.sql".to_string(),
                        file_sha256_hash:
                            "f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b"
                                .to_string(),
                        materialization: None,
                        references: vec!["seed_1".to_string(), "source_1".to_string()],
                        description: None,
                        tags: vec![],
                        columns: vec![],
                    },
                ),
                (
                    "model_2".to_string(),
                    quary_proto::Model {
                        name: "model_2".to_string(),
                        file_path: "models/model_2.sql".to_string(),
                        file_sha256_hash:
                            "00f34167b566208b3df2aebd093495f718a9d950e2dd7c7658977bb2734abff4"
                                .to_string(),
                        materialization: None,
                        references: vec!["seed_2".to_string(), "source_2".to_string()],
                        description: None,
                        tags: vec![],
                        columns: vec![],
                    },
                ),
                (
                    "model_3".to_string(),
                    quary_proto::Model {
                        name: "model_3".to_string(),
                        file_path: "models/model_3.sql".to_string(),
                        file_sha256_hash:
                            "f76a243c1d2b8bd70a95eb968c1b1e8d08931166a3cf63d24f9844ec07e029f6"
                                .to_string(),
                        materialization: None,
                        references: vec!["seed_3".to_string(), "source_3".to_string()],
                        description: None,
                        tags: vec![],
                        columns: vec![],
                    },
                ),
                (
                    "model_4".to_string(),
                    quary_proto::Model {
                        name: "model_4".to_string(),
                        file_path: "models/model_4.sql".to_string(),
                        file_sha256_hash:
                            "c3ac694382627234ebe1edad0f9cd75333ffbe8cf959a9c0d427625ab8e1172a"
                                .to_string(),
                        materialization: None,
                        references: vec![
                            "model_1".to_string(),
                            "model_2".to_string(),
                            "model_3".to_string(),
                        ],
                        description: None,
                        tags: vec![],
                        columns: vec![],
                    },
                ),
            ]),
            snapshots: HashMap::new(),
            tests: Default::default(),
            sources: HashMap::from([
                (
                    "source_1".to_string(),
                    Source {
                        name: "source_1".to_string(),
                        path: "project_id.dataset_id.source_1_table".to_string(),
                        file_path: "models/staging/schema.yaml".to_string(),
                        description: None,
                        tags: vec![],
                        columns: vec![],
                    },
                ),
                (
                    "source_2".to_string(),
                    Source {
                        name: "source_2".to_string(),
                        path: "project_id.dataset_id.source_2_table".to_string(),
                        file_path: "models/staging/schema.yaml".to_string(),
                        description: None,
                        tags: vec![],
                        columns: vec![],
                    },
                ),
                (
                    "source_3".to_string(),
                    Source {
                        name: "source_3".to_string(),
                        path: "project_id.dataset_id.source_3_table".to_string(),
                        file_path: "models/staging/schema.yaml".to_string(),
                        description: None,
                        tags: vec![],
                        columns: vec![],
                    },
                ),
            ]),
            project_files: Default::default(),
            connection_config: Default::default(),
        };

        let map = HashMap::from([
            ("model_1", "b7b7de6"),
            ("seed_1", "dfe7a23"),
            ("source_1", "e810a39"),
            ("model_2", "49475c4"),
            ("seed_2", "49475c4"),
            ("source_2", "49475c4"),
        ]);

        let result = given_map_and_hash_map_return_sub_graph_all_cached_for_a_particular_model(
            project, "model_4", &map,
        )
        .unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(
            result,
            HashMap::from([
                ("model_1".to_string(), "qqq_model_1_b7b7de6".to_string()),
                ("seed_1".to_string(), "qqq_seed_1_dfe7a23".to_string()),
                ("source_1".to_string(), "qqq_source_1_e810a39".to_string()),
            ])
        )
    }

    #[test]
    fn test_cache_view_name_to_table_name_and_hash() {
        let cache_view_name = "qqq_shifts_summary_7d2221b";
        let (table_name, hash) = cache_view_name_to_table_name_and_hash(cache_view_name).unwrap();
        assert_eq!(table_name, "shifts_summary");
        assert_eq!(hash, "7d2221b");

        let cache_view_name = "qqq_shifts49475c4";
        let result = cache_view_name_to_table_name_and_hash(cache_view_name);
        assert!(result.is_err());

        let cache_view_name = "q2q_shifts_summary_49475c4";
        let result = cache_view_name_to_table_name_and_hash(cache_view_name);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_derive_hash_views_on_seed() {
        let fs = quary_proto::FileSystem {
            files: HashMap::from([
                (
                    "seeds/seed_checkout_disputes.csv".to_string(),
                    File {
                        name: "seeds/seed_checkout_disputes.csv".to_string(),
                        contents: prost::bytes::Bytes::from("id,order_id,payment_method,amount"),
                    },
                ),
                (
                    "quary.yaml".to_string(),
                    File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            r#"
                            sqliteInMemory: {}
                "#
                            .as_bytes(),
                        ),
                    },
                ),
            ]),
        };
        let database = DatabaseQueryGeneratorSqlite {};
        let project = parse_project(&fs, &database, "").await.unwrap();
        let graph = project_to_graph(project.clone()).unwrap();

        assert!(derive_hash_views(&database, &project, &graph).is_ok());
    }
}
