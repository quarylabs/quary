use crate::automatic_branching::{create_automatic_branch_view_name, derive_model_hash};
use crate::databases::CacheStatus;
use crate::{
    automatic_branching::{
        derive_hash_views, is_cache_full_path, model_name_with_hash_to_model_name,
    },
    databases::DatabaseQueryGenerator,
    file_system::FileSystem,
    graph::project_to_graph,
    project::project_and_fs_to_sql_for_views,
};
use quary_proto::Project;
use std::collections::{BTreeMap, HashSet};

async fn build_only_non_cached_things(
    project: &Project,
    file_system: &impl FileSystem,
    database: &impl DatabaseQueryGenerator,
    view_paths_in_target: HashSet<String>,
) -> Result<Vec<(String, Vec<String>)>, String> {
    let filtered_cache_views_paths = view_paths_in_target
        .into_iter()
        .map(|path| {
            let is_cache = is_cache_full_path(database, &path)?;
            Ok((path, is_cache))
        })
        .collect::<Result<BTreeMap<String, bool>, String>>()?
        .into_iter()
        .filter(|(_, is_cache)| !is_cache)
        .map(|(path, _)| {
            let name = database.return_full_path_requirement(&path);
            let name_without_hash = model_name_with_hash_to_model_name(&name);
            (
                name_without_hash.to_string(),
                (name.to_string(), path.to_string()),
            )
        })
        .collect::<BTreeMap<String, (String, String)>>();

    let views_to_create =
        project_and_fs_to_sql_for_views(project, file_system, database, false, false).await?;

    let graph = project_to_graph(project.clone())?;
    let hashed_views = derive_hash_views(database, project, &graph)?;

    let views_to_create = views_to_create
        .into_iter()
        .map(|(name, view)| {
            let new_name_with_hash = hashed_views
                .get(name.as_str())
                .ok_or(format!("Could not find view with name: {}", name))?;

            let (new_name_with_hash, _) = new_name_with_hash;

            let existing_name_with_hash = filtered_cache_views_paths.get(&name);
            if let Some((name_with_hash, _)) = existing_name_with_hash {
                if name_with_hash == new_name_with_hash {
                    Ok::<_, String>((name, (CacheStatus::CachedAndMatching, new_name_with_hash)))
                } else {
                    Ok((name, (CacheStatus::NotMatching, new_name_with_hash)))
                }
            } else {
                Ok((name, (CacheStatus::NotMatching, new_name_with_hash)))
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    /// TODO Need to add here finding of old model and dropping it
    let each_model = views_to_create
        .into_iter()
        .map(|(name, (status, new_name_with_hash))| {
            let cache_view_path = database.return_full_path_requirement(&new_name_with_hash);

            let hash = derive_model_hash(&project, &graph, &name)?;
            let branch_view_name = create_automatic_branch_view_name(&name, &hash)?;
            let create_cache_view =
                database.automatic_cache_sql_create_statement(&name, &branch_view_name);
            // TODO Need to add function to drop old cache view
            // let drop_old_cache_view = drop_statement_for_cache_view(old_view.as_str());

            // TODO Make custom types for strings so that can't mess up rather than type aliases
            let drop_old_model = database
                .models_drop_query(&name, &None, &status)?
                .map(|x| vec![x])
                .unwrap_or_default();
            // TODO Fix the select statement
            let create_model = database
                .models_create_query(&name, &branch_view_name, &None, &status)?
                .map(|x| vec![x])
                .unwrap_or_default();

            let outs = vec![vec![], vec![drop_old_model], vec![create_model]]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .into_iter()
                .flatten()
                .collect();
            Ok((name, outs))
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(each_model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_postgres::DatabaseQueryGeneratorPostgres;
    use crate::project::parse_project;
    use std::collections::HashSet;

    fn to_file_system(files: Vec<(&str, &str)>) -> quary_proto::FileSystem {
        quary_proto::FileSystem {
            files: files
                .into_iter()
                .map(|(name, content)| {
                    (
                        name.to_string(),
                        quary_proto::File {
                            name: name.to_string(),
                            contents: prost::bytes::Bytes::from(content.to_string()),
                        },
                    )
                })
                .collect(),
        }
    }

    fn to_output(output: Vec<(&str, Vec<&str>)>) -> Vec<(String, Vec<String>)> {
        output
            .into_iter()
            .map(|(name, contents)| {
                (
                    name.to_string(),
                    contents.into_iter().map(|x| x.to_string()).collect(),
                )
            })
            .collect()
    }

    #[tokio::test]
    async fn test_build_only_empty() {
        let file_system = to_file_system(vec![("quary.yaml", "sqlite: {}")]);

        let database = DatabaseQueryGeneratorPostgres::new("analytics".to_string(), None);
        let project = parse_project(&file_system, &database, "").await.unwrap();

        let view_paths_in_target = HashSet::new();

        let result =
            build_only_non_cached_things(&project, &file_system, &database, view_paths_in_target)
                .await
                .unwrap();

        assert!(result.is_empty())
    }

    #[tokio::test]
    async fn test_build_single_model_view_no_cache() {
        let file_system = to_file_system(vec![
            ("quary.yaml", "sqlite: {}"),
            ("models/test.sql", "SELECT * FROM q.source_table"),
            (
                "models/schema.yaml",
                "sources: [{name: \"source_table\", path: data.schema.table}]",
            ),
        ]);

        let database = DatabaseQueryGeneratorPostgres::new("analytics".to_string(), None);
        let project = parse_project(&file_system, &database, "").await.unwrap();

        let view_paths_in_target = HashSet::new();

        let result =
            build_only_non_cached_things(&project, &file_system, &database, view_paths_in_target)
                .await
                .unwrap();

        assert_eq!(
            result,
            to_output(vec![(
                "test",
                vec![
                    "DROP VIEW IF EXISTS analytics.test CASCADE;",
                    "CREATE VIEW analytics.test AS SELECT * FROM data.schema.table;",
                    "CREATE VIEW qqq_test_19da7cf AS SELECT * FROM analytics.test;",
                ]
            )])
        )
    }

    /// TODO Actually implement
    #[tokio::test]
    async fn test_build_single_model_view_not_matching_cache() {
        let file_system = to_file_system(vec![
            ("quary.yaml", "sqlite: {}"),
            ("models/test.sql", "SELECT * FROM q.source_table"),
            (
                "models/schema.yaml",
                "sources: [{name: \"source_table\", path: data.schema.table}]",
            ),
        ]);

        let database = DatabaseQueryGeneratorPostgres::new("analytics".to_string(), None);
        let project = parse_project(&file_system, &database, "").await.unwrap();

        let view_paths_in_target = HashSet::from([
            "analytics.test".to_string(),
            "analytics.qqq_test_notright".to_string(),
        ]);

        let result =
            build_only_non_cached_things(&project, &file_system, &database, view_paths_in_target)
                .await
                .unwrap();

        assert_eq!(
            result,
            to_output(vec![(
                "test",
                vec![
                    "DROP VIEW IF EXISTS analytics.qqq_test_notright CASCADE;",
                    "DROP VIEW IF EXISTS analytics.test CASCADE;",
                    "CREATE OR REPLACE analytics.test AS SELECT * FROM data.schema.table;",
                    "CREATE VIEW qqq_test AS SELECT * FROM analytics.test;",
                ]
            )])
        )
    }

    /// TODO Actually implement
    #[tokio::test]
    async fn test_build_single_model_view_matching_cache() {
        let file_system = to_file_system(vec![
            ("quary.yaml", "sqlite: {}"),
            ("models/test.sql", "SELECT * FROM q.source_table"),
            (
                "models/schema.yaml",
                "sources: [{name: \"source_table\", path: data.schema.table}]",
            ),
        ]);

        let database = DatabaseQueryGeneratorPostgres::new("analytics".to_string(), None);
        let project = parse_project(&file_system, &database, "").await.unwrap();

        let view_paths_in_target = HashSet::new();

        let result =
            build_only_non_cached_things(&project, &file_system, &database, view_paths_in_target)
                .await
                .unwrap();

        assert_eq!(
            result,
            to_output(vec![(
                "test",
                vec![
                    "CREATE OR REPLACE analytics.test AS SELECT * FROM data.schema.table;",
                    "CREATE VIEW qqq_test AS SELECT * FROM analytics.test;",
                ]
            )])
        )
    }
}
