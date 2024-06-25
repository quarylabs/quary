use crate::file_system::convert_async_read_to_blocking_read;
use crate::{
    file_system::FileSystem,
    project::{get_path_bufs, EXTENSION_DASHBOARD_YAML, PATH_FOR_MODELS},
};
use quary_proto::dashboard_chart::Chart;
use quary_proto::dashboard_item::Item;
use quary_proto::{Dashboard, DashboardFile};
use std::{collections::BTreeMap, io::Read};

pub fn dashboard_file_from_yaml(yaml: impl Read) -> Result<DashboardFile, String> {
    serde_yaml::from_reader(yaml).map_err(|e| format!("reading yaml: {}", e))
}

pub fn dashboard_file_to_yaml(chart_file: &DashboardFile) -> Result<String, String> {
    serde_yaml::to_string(&chart_file).map_err(|e| format!("writing yaml: {}", e))
}

// parse all dashboard files in the project and return them in a map that includes the file itself
// and the list of any object reference the file depends on
pub(crate) async fn parse_dashboard_files(
    filesystem: &impl FileSystem,
    project_root: &str,
) -> Result<BTreeMap<String, (Dashboard, Vec<String>)>, String> {
    let paths = get_path_bufs(
        filesystem,
        project_root,
        PATH_FOR_MODELS,
        EXTENSION_DASHBOARD_YAML,
        &[],
    )
    .await?;

    let mut dashboard_files = BTreeMap::new();
    for path in paths {
        let str_path = path.to_string_lossy();
        let file = filesystem
            .read_file(&str_path)
            .await
            .map_err(|e| format!("reading file {:?}: {}", path, e))?;
        let file_contents = convert_async_read_to_blocking_read(file).await;
        let dashboard_file = dashboard_file_from_yaml(file_contents)
            .map_err(|e| format!("parsing file {:?}: {}", path, e))?;

        let file_name = path
            .file_name()
            .ok_or_else(|| format!("no file name in path: {:?}", path))?;
        // find name by removing the extension
        let name = file_name
            .to_string_lossy()
            .trim_end_matches(EXTENSION_DASHBOARD_YAML)
            .to_string();

        dashboard_files.insert(name, (str_path.to_string(), dashboard_file));
    }

    dashboard_files
        .into_iter()
        .map(|(name, (path, dashboard_file))| {
            let dependencies = dashboard_file
                .items
                .iter()
                .map(|item| match &item.item {
                    None => Err("item has no item".to_string()),
                    Some(item) => match item {
                        Item::Chart(chart) => {
                            if let Some(chart) = &chart.chart {
                                match chart {
                                    Chart::Reference(reference) => Ok(reference.reference.clone()),
                                }
                            } else {
                                Err("chart has no chart".to_string())
                            }
                        }
                    },
                })
                .collect::<Result<Vec<String>, String>>()?;
            let dashboard = Dashboard {
                name: dashboard_file.name,
                title: dashboard_file.title,
                description: dashboard_file.description,
                tags: dashboard_file.tags,
                items: dashboard_file.items,
                file_path: path,
            };
            Ok((name, (dashboard, dependencies)))
        })
        .collect::<Result<BTreeMap<String, (Dashboard, Vec<String>)>, String>>()
}
