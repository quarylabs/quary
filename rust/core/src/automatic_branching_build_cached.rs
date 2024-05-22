use crate::{
    automatic_branching::{derive_hash_views, is_cache_full_path},
    databases::DatabaseQueryGenerator,
    file_system::FileSystem,
    graph::project_to_graph,
    project::project_and_fs_to_sql_for_views,
};
use quary_proto::Project;
use std::collections::{BTreeMap, BTreeSet, HashSet};

fn build_only_non_cached_things(
    project: &Project,
    file_system: &impl FileSystem,
    database: &impl DatabaseQueryGenerator,
    view_paths_in_target: HashSet<String>,
) -> Result<BTreeMap<String, Vec<String>>, String> {
    let filtered_cache_views = view_paths_in_target
        .into_iter()
        .map(|name| {
            let is_cache = is_cache_full_path(database, &name)?;
            Ok((name, is_cache))
        })
        .collect::<Result<BTreeMap<String, bool>, String>>()?
        .into_iter()
        .filter(|(_, is_cache)| !is_cache)
        .collect::<BTreeSet<_>>();

    let views_to_create =
        project_and_fs_to_sql_for_views(project, file_system, database, false, false);

    let graph = project_to_graph(project.clone());
    let hashed_views = derive_hash_views(database, project, graph);

    

    unimplemented!();
}
