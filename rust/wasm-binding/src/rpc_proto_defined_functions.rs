use crate::rpc_proto_scaffolding::{JsFileSystem, Writer};
use quary_core::automatic_branching::{
    cache_view_name_to_table_name_and_hash,
    given_map_and_hash_map_return_sub_graph_all_cached_for_a_particular_model, is_cache_full_path,
};
use quary_core::chart::chart_file_to_yaml;
use quary_core::config::get_config_from_filesystem;
use quary_core::databases::DatabaseQueryGenerator;
use quary_core::description_table::map_to_description_table;
use quary_core::file_system::FileSystem;
use quary_core::graph::project_to_graph;
use quary_core::init::init_to_file_system;
use quary_core::onboarding::{generate_onboarding_files, is_empty_bar_hidden_and_sqlite};
use quary_core::project::{
    build_column_description_map_for_model, parse_project_files, return_defined_description_map,
    return_tests_for_a_particular_model, AssetsToSkip,
};
use quary_core::project_file::serialize_project_file_to_yaml;
use quary_core::project_to_sql::{
    project_and_fs_to_query_sql, project_and_fs_to_query_sql_for_model_sql,
    project_and_fs_to_sql_for_views,
};
use quary_core::rpc_proto_defined_functions::{
    name_to_raw_model_map_internal, render_schema_internal,
};
use quary_core::schema_name::DEFAULT_SCHEMA_PREFIX;
use quary_core::sql_inference_translator::map_test_to_sql_inference;
use quary_core::sql_model_finder::sql_model_finder;
use quary_proto::cache_view_information::CacheView;
use quary_proto::chart::Source;
use quary_proto::chart_file::AssetReference;
use quary_proto::project_file::{Model, Snapshot};
use quary_proto::return_definition_locations_for_sql_response::Definition;
use quary_proto::{
    chart_file, AddColumnTestToModelOrSourceColumnRequest,
    AddColumnTestToModelOrSourceColumnResponse, AddColumnToModelOrSourceRequest,
    AddColumnToModelOrSourceResponse, Chart, ChartFile, CreateModelChartFileRequest,
    CreateModelChartFileResponse, CreateModelSchemaEntryRequest, CreateModelSchemaEntryResponse,
    Edge, GenerateProjectFilesRequest, GenerateProjectFilesResponse, GenerateSourceFilesRequest,
    GenerateSourceFilesResponse, GetModelTableRequest, GetModelTableResponse,
    GetProjectConfigRequest, GetProjectConfigResponse, InitFilesRequest, InitFilesResponse,
    IsPathEmptyRequest, IsPathEmptyResponse, ListAssetsRequest, ListAssetsResponse, Node,
    ParseProjectRequest, ParseProjectResponse, Project, ProjectDag, ProjectFile, ProjectFileColumn,
    ProjectFileSource, RemoveColumnTestFromModelOrSourceColumnRequest,
    RemoveColumnTestFromModelOrSourceColumnResponse, RenderSchemaRequest, RenderSchemaResponse,
    ReturnDataForDocViewRequest, ReturnDataForDocViewResponse,
    ReturnDefinitionLocationsForSqlRequest, ReturnDefinitionLocationsForSqlResponse,
    ReturnFullProjectDagRequest, ReturnFullProjectDagResponse, ReturnFullSqlForAssetRequest,
    ReturnFullSqlForAssetResponse, ReturnSqlForInjectedModelRequest,
    ReturnSqlForInjectedModelResponse, ReturnSqlForSeedsAndModelsRequest,
    ReturnSqlForSeedsAndModelsResponse, StringifyProjectFileRequest, StringifyProjectFileResponse,
    Test, UpdateAssetDescriptionRequest, UpdateAssetDescriptionResponse,
    UpdateModelOrSourceColumnDescriptionRequest, UpdateModelOrSourceColumnDescriptionResponse,
};
use sqlinference::columns::get_columns_internal;
use sqlinference::infer_tests::infer_tests;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

pub(crate) async fn is_path_empty(
    _: Writer,
    file_system: JsFileSystem,
    request: IsPathEmptyRequest,
) -> Result<IsPathEmptyResponse, String> {
    let is_empty = is_empty_bar_hidden_and_sqlite(&file_system, &request.project_root)
        .await
        .map_err(|e| format!("Failed to get isEmpty status: {}", e))?;

    Ok(IsPathEmptyResponse { is_empty })
}

pub(crate) async fn get_project_config(
    _: Writer,
    file_system: JsFileSystem,
    request: GetProjectConfigRequest,
) -> Result<GetProjectConfigResponse, String> {
    let connection_config = get_config_from_filesystem(&file_system, &request.project_root)
        .await
        .map_err(|e| format!("Failed to get config from filesystem: {}", e))?;

    Ok(GetProjectConfigResponse {
        connection_config: Some(connection_config),
    })
}

pub(crate) async fn generate_project_files(
    writer: Writer,
    _: JsFileSystem,
    request: GenerateProjectFilesRequest,
) -> Result<GenerateProjectFilesResponse, String> {
    let connection_config = request
        .connection_config
        .ok_or("Connection config is missing.".to_string())?;
    let files = generate_onboarding_files(connection_config).await?;

    for (name, contents) in files {
        writer(name, contents).await?;
    }

    Ok(GenerateProjectFilesResponse {})
}

pub(crate) async fn generate_source_files(
    _: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: JsFileSystem,
    request: GenerateSourceFilesRequest,
) -> Result<GenerateSourceFilesResponse, String> {
    let project_root = request.project_root;

    let files = quary_core::onboarding::generate_source_files(
        &project_root,
        &file_system,
        &request.folder_path,
        &request.sources,
    )
    .await?;

    for (name, contents) in files {
        writer(name, contents).await?;
    }

    Ok(GenerateSourceFilesResponse {})
}

pub(crate) async fn init_files(
    writer: Writer,
    _: JsFileSystem,
    _: InitFilesRequest,
) -> Result<InitFilesResponse, String> {
    let response = init_to_file_system();

    for (name, file) in response.files.into_iter() {
        let s = String::from_utf8(file.contents.to_vec()).map_err(|_| {
            "Failed to convert bytes to string. This is a bug. Please report it.".to_string()
        })?;
        writer(name, s).await?;
    }

    Ok(InitFilesResponse {})
}

pub(crate) async fn stringify_project_file(
    _: Writer,
    _: JsFileSystem,
    request: StringifyProjectFileRequest,
) -> Result<StringifyProjectFileResponse, String> {
    let project_file = request.project_file.ok_or("No project file provided")?;
    let stringified_project_file = serialize_project_file_to_yaml(project_file)?;

    Ok(StringifyProjectFileResponse {
        stringified_project_file,
    })
}

pub(crate) async fn create_model_schema_entry(
    database: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: JsFileSystem,
    request: CreateModelSchemaEntryRequest,
) -> Result<CreateModelSchemaEntryResponse, String> {
    create_model_schema_entry_internal(
        &database,
        &writer,
        &file_system,
        &request.project_root,
        &request.model_name,
    )
    .await
}

pub(crate) async fn create_model_chart_file(
    _: Writer,
    _: JsFileSystem,
    request: CreateModelChartFileRequest,
) -> Result<CreateModelChartFileResponse, String> {
    let chart_file = ChartFile {
        description: None,
        tags: vec![],
        config: request.config,
        source: Some(chart_file::Source::Reference(AssetReference {
            name: request.model_name,
        })),
    };
    let chart_file = chart_file_to_yaml(&chart_file)?;
    Ok(CreateModelChartFileResponse { chart_file })
}

async fn create_model_schema_entry_internal(
    database: &impl DatabaseQueryGenerator,
    writer: &Writer,
    file_system: &impl FileSystem,
    project_root: &str,
    model_name: &str,
) -> Result<CreateModelSchemaEntryResponse, String> {
    let project = quary_core::project::parse_project(file_system, database, project_root).await?;

    match project.models.get(model_name) {
        None => Err(format!("Model {} not found in project", model_name)),
        Some(model) => {
            let project_files = parse_project_files(file_system, project_root, database).await?;

            // If already exists
            if let Some((key, file)) = project_files
                .iter()
                .find(|(_, file)| file.models.iter().any(|model| model.name == model_name))
            {
                Ok(CreateModelSchemaEntryResponse {
                    path: key.clone(),
                    project_file: Some(file.clone()),
                })
            } else {
                // If not exists
                // Take the filepath of the model and return the folder it is in
                let mut path = PathBuf::from(&project_root);
                path.push(model.file_path.as_str());
                path.pop();

                // Make the schema.yaml file be in there
                let mut schema_path = path.clone();
                schema_path.push("schema.yaml");
                let string_schema_path = schema_path
                    .to_str()
                    .ok_or("Failed to convert path to string")?
                    .to_string();

                // if file exists -> add to the models in it
                let mut project_file = project_files
                    .get(&string_schema_path)
                    .cloned()
                    .unwrap_or_default();
                project_file.models.push(Model {
                    tests: vec![],
                    tags: vec![],
                    name: model_name.to_string(),
                    description: None,
                    materialization: None,
                    columns: vec![],
                });
                let stringified_project_file =
                    serialize_project_file_to_yaml(project_file.clone())?;
                writer(string_schema_path.clone(), stringified_project_file).await?;

                Ok(CreateModelSchemaEntryResponse {
                    path: string_schema_path,
                    project_file: Some(project_file),
                })
            }
        }
    }
}

pub(crate) async fn update_asset_description(
    database: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: JsFileSystem,
    request: UpdateAssetDescriptionRequest,
) -> Result<UpdateAssetDescriptionResponse, String> {
    update_asset_description_internal(database, writer, file_system, request).await
}

async fn update_asset_description_internal(
    database: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: impl FileSystem,
    request: UpdateAssetDescriptionRequest,
) -> Result<UpdateAssetDescriptionResponse, String> {
    let project_root = request.project_root;
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;
    let asset_name = request.asset_name;

    let (file_path, project_file) = match (
        project.sources.get(&asset_name),
        project.models.get(&asset_name),
        project.snapshots.get(&asset_name),
    ) {
        (Some(_), None, None) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) =
                find_source_in_project_files(project_files, &asset_name)
                    .ok_or(format!("Source {} not found in project files", asset_name))?;

            let mut project_file = project_file.clone();
            project_file.sources = project_file
                .sources
                .into_iter()
                .map(|source| {
                    if source.name == asset_name {
                        ProjectFileSource {
                            description: Some(request.description.clone()),
                            ..source
                        }
                    } else {
                        source
                    }
                })
                .collect();
            Ok((file_path.to_string(), project_file))
        }
        (None, Some(_), None) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let schema_entry_exists = project_files
                .values()
                .any(|file| file.models.iter().any(|model| model.name == asset_name));
            match schema_entry_exists {
                false => {
                    let created_model_schema_entry_response = create_model_schema_entry_internal(
                        &database,
                        &writer,
                        &file_system,
                        &project_root,
                        &asset_name,
                    )
                    .await?;
                    let mut created_project_file = created_model_schema_entry_response
                        .project_file
                        .ok_or(format!("Schema file for {} not found", asset_name))?;
                    created_project_file.models = created_project_file
                        .models
                        .into_iter()
                        .map(|model| {
                            if model.name == asset_name {
                                Model {
                                    description: Some(request.description.clone()),
                                    ..model
                                }
                            } else {
                                model
                            }
                        })
                        .collect();

                    Ok((
                        created_model_schema_entry_response.path,
                        created_project_file,
                    ))
                }
                true => {
                    let project_files =
                        parse_project_files(&file_system, &project_root, &database).await?;
                    let (file_path, project_file) =
                        find_model_in_project_files(project_files, &asset_name)
                            .ok_or(format!("Model {} not found in project files", asset_name))?;

                    let mut project_file = project_file.clone();
                    project_file.models = project_file
                        .models
                        .into_iter()
                        .map(|model| {
                            if model.name == asset_name {
                                Model {
                                    description: Some(request.description.clone()),
                                    ..model
                                }
                            } else {
                                model
                            }
                        })
                        .collect();

                    Ok((file_path.to_string(), project_file))
                }
            }
        }
        (None, None, Some(_)) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) =
                find_snapshot_in_project_files(project_files, &asset_name).ok_or(format!(
                    "Snapshot {} not found in project files",
                    asset_name
                ))?;

            let mut project_file = project_file.clone();
            project_file.snapshots = project_file
                .snapshots
                .into_iter()
                .map(|snapshot| {
                    if snapshot.name == asset_name {
                        Snapshot {
                            description: Some(request.description.clone()),
                            ..snapshot
                        }
                    } else {
                        snapshot
                    }
                })
                .collect();
            Ok((file_path.to_string(), project_file))
        }
        (None, None, None) => Err(format!("Asset {} not found", asset_name)),
        _ => Err("Invalid: model/source/snapshot with same name".to_string()),
    }?;

    let project_file = serialize_project_file_to_yaml(project_file)?;
    writer(file_path.to_string(), project_file).await?;
    Ok(UpdateAssetDescriptionResponse {})
}

pub(crate) async fn add_column_to_model_or_source(
    database: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: JsFileSystem,
    request: AddColumnToModelOrSourceRequest,
) -> Result<AddColumnToModelOrSourceResponse, String> {
    add_column_to_model_or_source_internal(&database, &writer, &file_system, request).await?
}

async fn add_column_to_model_or_source_internal(
    database: &impl DatabaseQueryGenerator,
    writer: &Writer,
    file_system: &impl FileSystem,
    request: AddColumnToModelOrSourceRequest,
) -> Result<Result<AddColumnToModelOrSourceResponse, String>, String> {
    let project_root = request.project_root;
    let project = quary_core::project::parse_project(file_system, database, &project_root).await?;
    let model_name = request.model_or_source_name;

    let (file_path, project_file) = match (
        project.sources.get(&model_name),
        project.models.get(&model_name),
    ) {
        (Some(_), None) => {
            let project_files = parse_project_files(file_system, &project_root, database).await?;
            let (file_path, project_file) =
                find_source_in_project_files(project_files, &model_name)
                    .ok_or(format!("Source {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let source = project_file
                .sources
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            if !source
                .columns
                .iter()
                .any(|column| column.name == request.column_name)
            {
                source.columns.push(ProjectFileColumn {
                    name: request.column_name.to_string(),
                    description: None,
                    tests: vec![],
                });
            }
            Ok((file_path.to_string(), project_file))
        }
        (None, Some(_)) => {
            let project_files = parse_project_files(file_system, &project_root, database).await?;
            let (file_path, project_file) = find_model_in_project_files(project_files, &model_name)
                .ok_or(format!("Model {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let model = project_file
                .models
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            if !model
                .columns
                .iter()
                .any(|column| column.name == request.column_name)
            {
                model.columns.push(ProjectFileColumn {
                    name: request.column_name.to_string(),
                    description: None,
                    tests: vec![],
                });
            }
            Ok((file_path.to_string(), project_file))
        }
        (None, None) => {
            create_model_schema_entry_internal(
                database,
                writer,
                file_system,
                &project_root,
                &model_name,
            )
            .await?;

            let project_files = parse_project_files(file_system, &project_root, database).await?;
            let (file_path, project_file) = find_model_in_project_files(project_files, &model_name)
                .ok_or(format!("Model {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let model = project_file
                .models
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            if !model
                .columns
                .iter()
                .any(|column| column.name == request.column_name)
            {
                model.columns.push(ProjectFileColumn {
                    name: request.column_name.to_string(),
                    description: None,
                    tests: vec![],
                });
            }
            Ok((file_path.to_string(), project_file))
        }
        _ => Err("Invalid: model and source with same name".to_string()),
    }?;

    let project_file = serialize_project_file_to_yaml(project_file)?;
    writer(file_path.to_string(), project_file).await?;
    Ok(Ok(AddColumnToModelOrSourceResponse {}))
}

pub(crate) async fn update_model_source_column_description(
    database: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: JsFileSystem,
    request: UpdateModelOrSourceColumnDescriptionRequest,
) -> Result<UpdateModelOrSourceColumnDescriptionResponse, String> {
    let project_root = request.project_root.clone();
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;
    let model_name = request.model_or_source_name.clone();

    create_model_schema_entry_internal(
        &database,
        &writer,
        &file_system,
        &project_root,
        &model_name,
    )
    .await?;

    let (file_path, project_file) = match (
        project.sources.get(&model_name),
        project.models.get(&model_name),
    ) {
        (Some(_), None) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) =
                find_source_in_project_files(project_files, &model_name)
                    .ok_or(format!("Source {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let source = project_file
                .sources
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            if !source
                .columns
                .iter()
                .any(|column| column.name == request.column_name)
            {
                source.columns.push(ProjectFileColumn {
                    name: request.column_name.to_string(),
                    description: request.description.clone(),
                    tests: vec![],
                });
            }
            Ok((file_path.to_string(), project_file))
        }
        (None, Some(_)) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) = find_model_in_project_files(project_files, &model_name)
                .ok_or(format!("Model {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let model = project_file
                .models
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            if !model
                .columns
                .iter()
                .any(|column| column.name == request.column_name)
            {
                model.columns.push(ProjectFileColumn {
                    name: request.column_name.to_string(),
                    description: request.description.clone(),
                    tests: vec![],
                });
            } else {
                model.columns = model
                    .columns
                    .iter_mut()
                    .map(|column| {
                        if column.name == request.column_name {
                            ProjectFileColumn {
                                description: request.description.clone(),
                                ..column.clone()
                            }
                        } else {
                            column.clone()
                        }
                    })
                    .collect();
            }
            Ok((file_path.to_string(), project_file))
        }
        _ => Err("Invalid: model and source with same name".to_string()),
    }?;

    let project_file = serialize_project_file_to_yaml(project_file)?;
    writer(file_path.to_string(), project_file).await?;
    Ok(UpdateModelOrSourceColumnDescriptionResponse {})
}

pub(crate) async fn add_column_test_to_model_or_source_column(
    database: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: JsFileSystem,
    request: AddColumnTestToModelOrSourceColumnRequest,
) -> Result<AddColumnTestToModelOrSourceColumnResponse, String> {
    let _ = add_column_to_model_or_source_internal(
        &database,
        &writer,
        &file_system,
        AddColumnToModelOrSourceRequest {
            project_root: request.project_root.clone(),
            model_or_source_name: request.model_or_source_name.clone(),
            column_name: request.column_name.clone(),
        },
    )
    .await?;

    let project_root = request.project_root;
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;
    let model_name = request.model_or_source_name;
    let column_test = request.column_test.ok_or("No column test provided")?;

    let (path, project_file) = match (
        project.sources.get(&model_name),
        project.models.get(&model_name),
    ) {
        (Some(_), _) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) =
                find_source_in_project_files(project_files, &model_name)
                    .ok_or(format!("Source {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let source = project_file
                .sources
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            let column = source
                .columns
                .iter_mut()
                .find(|column| column.name == request.column_name)
                .ok_or(format!(
                    "Column {} not found in source {}",
                    request.column_name, model_name
                ))?;

            if !column.tests.contains(&column_test) {
                column.tests.push(column_test.clone());
            }
            Ok((file_path.to_string(), project_file))
        }
        (_, Some(_)) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) = project_files
                .iter()
                .find(|(_, project_file)| {
                    project_file
                        .models
                        .iter()
                        .any(|model| model.name == model_name)
                })
                .ok_or(format!("Model {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let model = project_file
                .models
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            let column = model
                .columns
                .iter_mut()
                .find(|column| column.name == request.column_name)
                .ok_or(format!(
                    "Column {} not found in source {}",
                    request.column_name, model_name
                ))?;

            if !column.tests.contains(&column_test) {
                column.tests.push(column_test.clone());
            }
            Ok((file_path.to_string(), project_file))
        }
        _ => Err("Invalid: model and source with same name".to_string()),
    }?;

    let project_file = serialize_project_file_to_yaml(project_file)?;
    writer(path.to_string(), project_file).await?;
    Ok(AddColumnTestToModelOrSourceColumnResponse {})
}

fn find_source_in_project_files(
    project_files: HashMap<String, ProjectFile>,
    name: &str,
) -> Option<(String, ProjectFile)> {
    project_files.into_iter().find(|(_, project_file)| {
        project_file
            .sources
            .iter()
            .any(|source| source.name == name)
    })
}

fn find_model_in_project_files(
    project_files: HashMap<String, ProjectFile>,
    name: &str,
) -> Option<(String, ProjectFile)> {
    project_files
        .into_iter()
        .find(|(_, project_file)| project_file.models.iter().any(|model| model.name == name))
}

fn find_snapshot_in_project_files(
    project_files: HashMap<String, ProjectFile>,
    name: &str,
) -> Option<(String, ProjectFile)> {
    project_files.into_iter().find(|(_, project_file)| {
        project_file
            .snapshots
            .iter()
            .any(|snapshot| snapshot.name == name)
    })
}

pub(crate) async fn remove_column_test_from_model_or_source_column(
    database: impl DatabaseQueryGenerator,
    writer: Writer,
    file_system: JsFileSystem,
    request: RemoveColumnTestFromModelOrSourceColumnRequest,
) -> Result<RemoveColumnTestFromModelOrSourceColumnResponse, String> {
    let project_root = request.project_root;
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;
    let model_name = request.model_or_source_name;
    let column_test = request.column_test.ok_or("No column test provided")?;

    let (path, project_file) = match (
        project.sources.get(&model_name),
        project.models.get(&model_name),
    ) {
        (Some(_), _) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) =
                find_source_in_project_files(project_files, &model_name)
                    .ok_or(format!("Source {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let source = project_file
                .sources
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            let column = source
                .columns
                .iter_mut()
                .find(|column| column.name == request.column_name)
                .ok_or(format!(
                    "Column {} not found in source {}",
                    request.column_name, model_name
                ))?;

            if !column.tests.contains(&column_test) {
                column.tests.push(column_test.clone());
            }
            Ok((file_path.to_string(), project_file))
        }
        (_, Some(_)) => {
            let project_files = parse_project_files(&file_system, &project_root, &database).await?;
            let (file_path, project_file) = find_model_in_project_files(project_files, &model_name)
                .ok_or(format!("Model {} not found in project files", model_name))?;

            let mut project_file = project_file.clone();
            let model = project_file
                .models
                .iter_mut()
                .find(|source| source.name == model_name)
                .ok_or(format!("Source {} not found in project files", model_name))?;
            let column = model
                .columns
                .iter_mut()
                .find(|column| column.name == request.column_name)
                .ok_or(format!(
                    "Column {} not found in source {}",
                    request.column_name, model_name
                ))?;
            if let Some(test_index) = column.tests.iter().position(|test| test == &column_test) {
                column.tests.remove(test_index);
            } else {
                return Err(format!(
                    "Test '{}' not found in column '{}'",
                    column_test.r#type, request.column_name
                ));
            }
            Ok((file_path.to_string(), project_file))
        }
        _ => Err("Invalid: model and source with same name".to_string()),
    }?;

    let project_file = serialize_project_file_to_yaml(project_file)?;
    writer(path.to_string(), project_file).await?;
    Ok(RemoveColumnTestFromModelOrSourceColumnResponse {})
}

pub(crate) async fn list_assets(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ListAssetsRequest,
) -> Result<ListAssetsResponse, String> {
    list_assets_internal(&database, &file_system, request).await
}

pub(crate) async fn list_assets_internal(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    request: ListAssetsRequest,
) -> Result<ListAssetsResponse, String> {
    let assets_to_skip = request.assets_to_skip.ok_or("No assets to skip provided")?;
    let project = quary_core::project::parse_project_with_skip(
        file_system,
        database,
        &request.project_root,
        AssetsToSkip {
            charts: assets_to_skip.charts,
        },
    )
    .await?;

    let assets = project
        .models
        .into_iter()
        .map(|(name, model)| quary_proto::list_assets_response::Asset {
            name,
            description: model.description,
            tags: model.tags,
            asset_type: i32::from(quary_proto::list_assets_response::asset::AssetType::Model),
            file_path: model.file_path,
        })
        .chain(project.seeds.into_iter().map(|(name, seed)| {
            quary_proto::list_assets_response::Asset {
                name,
                description: None,
                tags: vec![],
                asset_type: i32::from(quary_proto::list_assets_response::asset::AssetType::Seed),
                file_path: seed.file_path,
            }
        }))
        .chain(project.sources.into_iter().map(|(name, source)| {
            quary_proto::list_assets_response::Asset {
                name,
                description: source.description,
                tags: source.tags,
                asset_type: i32::from(quary_proto::list_assets_response::asset::AssetType::Source),
                file_path: source.file_path,
            }
        }))
        .chain(project.snapshots.into_iter().map(|(name, snapshot)| {
            quary_proto::list_assets_response::Asset {
                name,
                description: snapshot.description,
                tags: snapshot.tags,
                asset_type: i32::from(
                    quary_proto::list_assets_response::asset::AssetType::Snapshot,
                ),
                file_path: snapshot.file_path,
            }
        }))
        .chain(project.charts.into_iter().map(|(name, chart)| {
            quary_proto::list_assets_response::Asset {
                name,
                description: chart.description,
                tags: chart.tags,
                asset_type: i32::from(quary_proto::list_assets_response::asset::AssetType::Chart),
                file_path: chart.file_path,
            }
        }))
        .collect::<Vec<_>>();

    Ok(ListAssetsResponse { assets })
}

pub(crate) async fn render_schema(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: RenderSchemaRequest,
) -> Result<RenderSchemaResponse, String> {
    let project_root = request.project_root;
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;

    let schema = render_schema_internal(&database, project, &file_system).await?;
    Ok(RenderSchemaResponse { schema })
}

pub(crate) async fn get_model_table(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: GetModelTableRequest,
) -> Result<GetModelTableResponse, String> {
    get_model_table_internal(&database, &file_system, request).await
}

pub(crate) async fn get_model_table_internal(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    request: GetModelTableRequest,
) -> Result<GetModelTableResponse, String> {
    let project_root = request.project_root;

    let project = quary_core::project::parse_project(file_system, database, &project_root).await?;
    let dialect = database.get_dialect().get_dialect();

    let model_map = name_to_raw_model_map_internal(&project, file_system).await?;
    let model_statement = model_map
        .get(&request.model_name)
        .ok_or(format!("Model {} not found", request.model_name))?;
    let columns = get_columns_internal(database.get_dialect(), model_statement)
        .ok()
        .map(|(columns, _)| {
            columns
                .iter()
                .map(|column| column.to_string())
                .collect::<Vec<String>>()
        });
    let inferred_tests: Option<Vec<sqlinference::test::Test>> = infer_tests(
        database.get_dialect(),
        format!("{}.{}", DEFAULT_SCHEMA_PREFIX, request.model_name).as_str(),
        model_statement,
        &project
            .tests
            .values()
            .filter_map(|test| map_test_to_sql_inference(DEFAULT_SCHEMA_PREFIX, test.clone()))
            .collect(),
    )
    .ok()
    .map(|tests| tests.into_keys().collect::<Vec<_>>());
    let inferred_descriptions_map = build_column_description_map_for_model(
        &project,
        DEFAULT_SCHEMA_PREFIX,
        &request.model_name,
        &*dialect,
        file_system,
    )
    .await
    .ok();
    let defined_column_description_map =
        return_defined_description_map(&project, request.model_name.as_str()).map(
            |hash_map| -> HashMap<String, String> {
                hash_map
                    .iter()
                    .map(|(column, description)| (column.to_string(), description.to_string()))
                    .collect()
            },
        );

    let actual_tests: Option<Vec<Test>> = Some(
        return_tests_for_a_particular_model(&project, &request.model_name)
            .cloned()
            .collect(),
    );

    let project_columns = project.models.get(&request.model_name).map(|model| {
        model
            .columns
            .iter()
            .map(|column| column.title.to_string())
            .collect::<Vec<String>>()
    });
    let inferred_columns =
        columns.map(|columns| columns.iter().map(|column| column.to_string()).collect());
    let inferred_column_description_map = inferred_descriptions_map.map(|hash_map| {
        hash_map
            .iter()
            .map(|(column, description)| (column.to_string(), description.to_string()))
            .collect()
    });

    let table = map_to_description_table(
        DEFAULT_SCHEMA_PREFIX,
        project_columns,
        inferred_columns,
        defined_column_description_map,
        inferred_column_description_map,
        actual_tests,
        inferred_tests,
    )?;

    Ok(GetModelTableResponse { table: Some(table) })
}

pub(crate) async fn parse_project(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ParseProjectRequest,
) -> Result<ParseProjectResponse, String> {
    let project =
        quary_core::project::parse_project(&file_system, &database, &request.project_root).await?;

    Ok(ParseProjectResponse {
        project: Some(project),
    })
}

pub(crate) async fn return_sql_for_seeds_and_models(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnSqlForSeedsAndModelsRequest,
) -> Result<ReturnSqlForSeedsAndModelsResponse, String> {
    // TODO This can be moved out of here
    let project =
        quary_core::project::parse_project(&file_system, &database, &request.project_root).await?;
    // TODO Need to make this dynamic
    let sqls =
        project_and_fs_to_sql_for_views(&project, &file_system, &database, false, false).await?;
    let sql = sqls
        .iter()
        .flat_map(|(_, s)| s.clone())
        .collect::<Vec<String>>();
    Ok(ReturnSqlForSeedsAndModelsResponse {
        sql,
        project: Some(project),
    })
}

pub(crate) async fn return_data_for_doc_view(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnDataForDocViewRequest,
) -> Result<ReturnDataForDocViewResponse, String> {
    let data = return_full_sql_for_asset_internal(
        &database,
        &file_system,
        ReturnFullSqlForAssetRequest {
            project_root: request.project_root.clone(),
            asset_name: request.asset_name.clone(),
            cache_view_information: request.cache_view_information,
        },
    )
    .await?;

    let project_files = parse_project_files(&file_system, &request.project_root, &database).await?;
    let is_asset_in_schema_files =
        find_source_in_project_files(project_files.clone(), &request.asset_name).is_some()
            || find_model_in_project_files(project_files.clone(), &request.asset_name).is_some()
            || find_snapshot_in_project_files(project_files, &request.asset_name).is_some();

    Ok(ReturnDataForDocViewResponse {
        full_sql: data.full_sql,
        description: data.description,
        dag: data.dag,
        is_asset_in_schema_files,
    })
}

pub(crate) async fn return_full_sql_for_asset(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnFullSqlForAssetRequest,
) -> Result<ReturnFullSqlForAssetResponse, String> {
    return_full_sql_for_asset_internal(&database, &file_system, request).await
}

async fn return_full_sql_for_asset_internal(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    request: ReturnFullSqlForAssetRequest,
) -> Result<ReturnFullSqlForAssetResponse, String> {
    let project_root = request.project_root;
    let project = quary_core::project::parse_project(file_system, database, &project_root).await?;
    let cache_view = request
        .cache_view_information
        .ok_or("No cache view mode provided")?
        .cache_view
        .ok_or("No cache view mode provided")?;

    match (
        project.sources.get(&request.asset_name),
        project.snapshots.get(&request.asset_name),
        project.models.get(&request.asset_name),
        project.charts.get(&request.asset_name),
    ) {
        (Some(source), None, None, None) => {
            return_full_sql_for_source(project.clone(), file_system, source.name.clone(), database)
                .await
        }
        (None, Some(snapshot), None, None) => {
            return_full_sql_for_snapshot(
                project.clone(),
                file_system,
                snapshot.name.clone(),
                database,
                cache_view,
            )
            .await
        }
        (None, None, Some(model), None) => {
            return_full_sql_for_model(
                project.clone(),
                file_system,
                model.name.clone(),
                database,
                cache_view,
            )
            .await
        }
        (None, None, None, Some(chart)) => {
            return_full_sql_for_chart(file_system, database, project.clone(), chart, cache_view)
                .await
        }
        _ => Err(format!(
            "Model, source, or snapshot '{}' not found in project",
            &request.asset_name
        )),
    }
}

async fn return_full_sql_for_source(
    project: Project,
    file_system: &impl FileSystem,
    source_name: String,
    database: &impl DatabaseQueryGenerator,
) -> Result<ReturnFullSqlForAssetResponse, String> {
    let (sql, nodes, edges) = {
        let (sql, (nodes, edges)) =
            project_and_fs_to_query_sql(database, &project, file_system, &source_name, None)
                .await?;
        (sql, nodes, edges)
    };

    let out_edges = edges
        .into_iter()
        .map(|(from, to)| Edge { from, to })
        .collect::<Vec<_>>();

    // If you want to have a string representation of the path
    Ok(ReturnFullSqlForAssetResponse {
        full_sql: sql,
        description: project
            .sources
            .get(&source_name)
            .and_then(|m| m.description.clone()),
        dag: Some(ProjectDag {
            nodes: nodes
                .into_iter()
                .map(|node| Node {
                    id: node.clone(),
                    is_cached: false, // sources are never cached
                })
                .collect::<Vec<_>>(),
            edges: out_edges,
        }),
    })
}

async fn return_full_sql_for_model(
    project: Project,
    file_system: &impl FileSystem,
    model_name: String,
    database: &impl DatabaseQueryGenerator,
    cache_view: CacheView,
) -> Result<ReturnFullSqlForAssetResponse, String> {
    let (sql, nodes, edges, cached_models): (String, BTreeSet<_>, Vec<_>, HashSet<String>) =
        match cache_view {
            CacheView::CacheViewInformation(cache_views) => {
                let existing_cache_views_name = cache_views
                    .cache_view_paths
                    .iter()
                    .filter(|view| is_cache_full_path(database, view).unwrap_or(false))
                    .map(|cache_view_path| database.return_name_from_full_path(cache_view_path))
                    .collect::<Result<Vec<_>, String>>()?
                    .into_iter()
                    .map(|name_with_hash| {
                        let (name, _) = cache_view_name_to_table_name_and_hash(name_with_hash)?;
                        Ok((name.to_string(), name_with_hash.to_string()))
                    })
                    .collect::<Result<HashMap<String, String>, String>>()?;
                let map = existing_cache_views_name
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                let matched_map =
                    given_map_and_hash_map_return_sub_graph_all_cached_for_a_particular_model(
                        project.clone(),
                        &model_name,
                        &map,
                    )?;
                let matched_map = matched_map
                    .into_iter()
                    .map(|(k, v)| (k, database.return_full_path_requirement(v.as_str())))
                    .collect::<HashMap<_, _>>();
                let (sql, _) = project_and_fs_to_query_sql(
                    database,
                    &project,
                    file_system,
                    &model_name,
                    Some(matched_map.clone()),
                )
                .await?;
                let (_, (nodes, edges)) =
                    project_and_fs_to_query_sql(database, &project, file_system, &model_name, None)
                        .await?;
                Ok::<
                    (
                        String,
                        BTreeSet<String>,
                        Vec<(String, String)>,
                        HashSet<String>,
                    ),
                    String,
                >((sql, nodes, edges, matched_map.keys().cloned().collect()))
            }
            CacheView::DoNotUse(_) => {
                let (sql, (nodes, edges)) =
                    project_and_fs_to_query_sql(database, &project, file_system, &model_name, None)
                        .await?;
                Ok((sql, nodes, edges, HashSet::<String>::new()))
            }
        }?;

    let out_edges = edges
        .into_iter()
        .map(|(from, to)| Edge { from, to })
        .collect::<Vec<_>>();

    // If you want to have a string representation of the path
    Ok(ReturnFullSqlForAssetResponse {
        full_sql: sql,
        description: project
            .models
            .get(&model_name)
            .and_then(|m| m.description.clone()),
        dag: Some(ProjectDag {
            nodes: nodes
                .into_iter()
                .map(|node| Node {
                    id: node.clone(),
                    is_cached: cached_models.contains(&node),
                })
                .collect::<Vec<_>>(),
            edges: out_edges,
        }),
    })
}

async fn return_full_sql_for_snapshot(
    project: Project,
    file_system: &impl FileSystem,
    snapshot_name: String,
    database: &impl DatabaseQueryGenerator,
    cache_view: CacheView,
) -> Result<ReturnFullSqlForAssetResponse, String> {
    let (sql, nodes, edges, cached_models): (String, BTreeSet<_>, Vec<_>, HashSet<String>) =
        match cache_view {
            CacheView::CacheViewInformation(cache_views) => {
                let existing_cache_views_name = cache_views
                    .cache_view_paths
                    .iter()
                    .filter(|view| is_cache_full_path(database, view).unwrap_or(false))
                    .map(|cache_view_path| database.return_name_from_full_path(cache_view_path))
                    .collect::<Result<Vec<_>, String>>()?
                    .into_iter()
                    .map(|name_with_hash| {
                        let (name, _) = cache_view_name_to_table_name_and_hash(name_with_hash)?;
                        Ok((name.to_string(), name_with_hash.to_string()))
                    })
                    .collect::<Result<HashMap<String, String>, String>>()?;

                let map = existing_cache_views_name
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                let matched_map =
                    given_map_and_hash_map_return_sub_graph_all_cached_for_a_particular_model(
                        project.clone(),
                        &snapshot_name,
                        &map,
                    )?;

                let matched_map = matched_map
                    .into_iter()
                    .map(|(k, v)| (k, database.return_full_path_requirement(v.as_str())))
                    .collect::<HashMap<_, _>>();
                let (sql, _) = project_and_fs_to_query_sql(
                    database,
                    &project,
                    file_system,
                    &snapshot_name,
                    Some(matched_map.clone()),
                )
                .await?;
                let (_, (nodes, edges)) = project_and_fs_to_query_sql(
                    database,
                    &project,
                    file_system,
                    &snapshot_name,
                    None,
                )
                .await?;
                Ok::<
                    (
                        String,
                        BTreeSet<String>,
                        Vec<(String, String)>,
                        HashSet<String>,
                    ),
                    String,
                >((sql, nodes, edges, matched_map.keys().cloned().collect()))
            }
            CacheView::DoNotUse(_) => {
                let (sql, (nodes, edges)) = project_and_fs_to_query_sql(
                    database,
                    &project,
                    file_system,
                    &snapshot_name,
                    None,
                )
                .await?;
                Ok((sql, nodes, edges, HashSet::<String>::new()))
            }
        }?;

    let out_edges = edges
        .into_iter()
        .map(|(from, to)| Edge { from, to })
        .collect::<Vec<_>>();

    Ok(ReturnFullSqlForAssetResponse {
        full_sql: sql,
        description: project
            .snapshots
            .get(&snapshot_name)
            .and_then(|m| m.description.clone()),
        dag: Some(ProjectDag {
            nodes: nodes
                .into_iter()
                .map(|node| Node {
                    id: node.clone(),
                    is_cached: cached_models.contains(&node),
                })
                .collect::<Vec<_>>(),
            edges: out_edges,
        }),
    })
}

async fn return_full_sql_for_chart(
    file_system: &impl FileSystem,
    database: &impl DatabaseQueryGenerator,
    project: Project,
    chart: &Chart,
    cache_view: CacheView,
) -> Result<ReturnFullSqlForAssetResponse, String> {
    let source = chart.source.as_ref().ok_or("No source provided")?;
    let description = chart.description.clone();
    match source {
        Source::RawSql(raw_sql) => Ok(ReturnFullSqlForAssetResponse {
            full_sql: raw_sql.clone(),
            description,
            dag: None,
        }),
        Source::PreTemplatedSql(_) => {
            unimplemented!()
        }
        Source::Reference(reference) => {
            let returned_for_model = return_full_sql_for_model(
                project,
                file_system,
                reference.name.clone(),
                database,
                cache_view,
            )
            .await?;
            Ok(ReturnFullSqlForAssetResponse {
                full_sql: returned_for_model.full_sql,
                dag: returned_for_model.dag,
                description,
            })
        }
    }
}

pub(crate) async fn return_full_project_dag(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnFullProjectDagRequest,
) -> Result<ReturnFullProjectDagResponse, String> {
    let project_root = request.project_root;

    let mut project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;

    project.tests = Default::default();

    let full_dag = project_to_graph(project)?;

    let edges = full_dag
        .edges
        .iter()
        .map(|(from, to)| Edge {
            from: from.to_string(),
            to: to.to_string(),
        })
        .collect::<Vec<Edge>>();

    let nodes: BTreeSet<String> = edges
        .iter()
        .flat_map(|edge| vec![edge.from.clone(), edge.to.clone()])
        .collect::<BTreeSet<String>>();

    // If you want to have a string representation of the path
    Ok(ReturnFullProjectDagResponse {
        dag: Some(ProjectDag {
            nodes: nodes
                .into_iter()
                .map(|node| Node {
                    id: node.clone(),
                    is_cached: false,
                })
                .collect::<Vec<_>>(),
            edges,
        }),
    })
}

pub(crate) async fn return_definition_locations_for_sql(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnDefinitionLocationsForSqlRequest,
) -> Result<ReturnDefinitionLocationsForSqlResponse, String> {
    let project_root = request.project_root;
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;

    let models = sql_model_finder(&request.sql);

    let definitions = models
        .into_iter()
        .flat_map(|(model, range)| -> Vec<Definition> {
            range
                .iter()
                .filter_map(
                    |range| match (project.models.get(&model), project.seeds.get(&model)) {
                        (Some(model), None) => {
                            let file_path = model.file_path.to_string();
                            let file_path = file_path
                                .strip_prefix(&project_root)
                                .unwrap_or(&file_path)
                                .to_string();
                            Some(Definition {
                                range: Some(range.clone()),
                                target_model: model.name.to_string(),
                                target_file: file_path.to_string(),
                            })
                        }
                        (None, Some(seed)) => {
                            let file_path = seed.file_path.to_string();
                            let file_path = file_path
                                .strip_prefix(&project_root)
                                .unwrap_or(&file_path)
                                .to_string();
                            Some(Definition {
                                range: Some(range.clone()),
                                target_model: seed.name.to_string(),
                                target_file: file_path.to_string(),
                            })
                        }
                        _ => None,
                    },
                )
                .collect()
        })
        .collect();

    Ok(ReturnDefinitionLocationsForSqlResponse { definitions })
}

pub(crate) async fn return_sql_for_injected_model(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnSqlForInjectedModelRequest,
) -> Result<ReturnSqlForInjectedModelResponse, String> {
    let (sql, _) = project_and_fs_to_query_sql_for_model_sql(
        &database,
        &file_system,
        &request.project_root,
        &request.sql,
        None,
        &request.temporary_id,
    )
    .await?;
    Ok(ReturnSqlForInjectedModelResponse { sql })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use quary_core::database_duckdb::DatabaseQueryGeneratorDuckDB;
    use quary_core::database_redshift::DatabaseQueryGeneratorRedshift;
    use quary_core::database_sqlite::DatabaseQueryGeneratorSqlite;
    use quary_core::file_system_override::OverrideFileSystem;
    use quary_core::init::DuckDBAsset;
    use quary_proto::table::TableType;
    use quary_proto::{CacheViewInformation, CacheViewInformationPaths, FileSystem};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::time::SystemTime;

    // TODO Make all the others use this function
    fn files_to_file_system(files: Vec<(&str, &str)>) -> FileSystem {
        FileSystem {
            files: files
                .into_iter()
                .map(|(name, contents)| {
                    let contents = contents.to_string();
                    (
                        name.to_string(),
                        quary_proto::File {
                            name: name.to_string(),
                            contents: prost::bytes::Bytes::from(contents),
                        },
                    )
                })
                .collect(),
        }
    }

    #[tokio::test]
    async fn test_update_model_description_with_schema_present() {
        let (writer, written_files) = setup_file_mocks();

        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = files_to_file_system(vec![
            ("quary.yaml", "sqliteInMemory: {}"),
            ("models/shifts.sql", "SELECT 1"),
            (
                "models/schema.yaml",
                "
                            models:
                              - name: shifts
                            ",
            ),
        ]);

        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "shifts".to_string(),
            description: "Updated description".to_string(),
        };

        update_asset_description_internal(database, writer, file_system, request)
            .await
            .unwrap();

        let binding = written_files.borrow();
        let updated_content = binding.get(&"models/schema.yaml".to_string()).unwrap();

        assert_eq!(
            updated_content,
            "models:\n- name: shifts\n  description: Updated description\n"
        );
    }

    #[tokio::test]
    async fn test_update_asset_description_source_with_schema_present() {
        let (writer, written_files) = setup_file_mocks();

        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = files_to_file_system(vec![
            ("quary.yaml", "sqliteInMemory: {}"),
            (
                "models/staging/schema.yaml",
                "sources: [{name: raw_shifts, path: raw_shifts_real_table}]",
            ),
        ]);

        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "raw_shifts".to_string(),
            description: "Updated description".to_string(),
        };

        update_asset_description_internal(database, writer, file_system, request)
            .await
            .unwrap();

        let binding = written_files.borrow();
        let updated_content = binding
            .get(&"models/staging/schema.yaml".to_string())
            .unwrap();

        assert_eq!(
            updated_content,
            "sources:\n- name: raw_shifts\n  description: Updated description\n  path: raw_shifts_real_table\n"
        );
    }

    #[tokio::test]
    async fn test_update_snapshot_description_with_schema_present() {
        let (writer, written_files) = setup_file_mocks();

        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = files_to_file_system(vec![
            ("quary.yaml", "sqliteInMemory: {}"),
            ("models/orders_snapshot.snapshot.sql", "SELECT 1"),
            (
                "models/schema.yaml",
                "
                            snapshots:
                            - name: orders_snapshot
                              unique_key: order_id
                              strategy:
                                timestamp:
                                  updated_at: updated_at",
            ),
        ]);

        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "orders_snapshot".to_string(),
            description: "Updated description".to_string(),
        };

        update_asset_description_internal(database, writer, file_system, request)
            .await
            .unwrap();

        let binding = written_files.borrow();
        let updated_content = binding.get(&"models/schema.yaml".to_string()).unwrap();

        assert_eq!(
            updated_content,
            "snapshots:\n- name: orders_snapshot\n  description: Updated description\n  uniqueKey: order_id\n  strategy:\n    timestamp:\n      updatedAt: updated_at\n"
        );
    }

    #[tokio::test]
    async fn test_update_model_description_without_schema_present() {
        let (writer, written_files) = setup_file_mocks();

        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = FileSystem {
            files: vec![
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
                        contents: prost::bytes::Bytes::from("SELECT 1"),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };

        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "shifts".to_string(),
            description: "Updated description".to_string(),
        };

        update_asset_description_internal(database, writer, file_system, request)
            .await
            .unwrap();

        let binding = written_files.borrow();
        let updated_content = binding.get(&"models/schema.yaml".to_string()).unwrap();

        assert_eq!(
            updated_content,
            "models:\n- name: shifts\n  description: Updated description\n"
        );
    }

    #[tokio::test]
    async fn test_update_model_description_with_schema_present_without_definition() {
        let (writer, written_files) = setup_file_mocks();

        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = FileSystem {
            files: vec![
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
                        contents: prost::bytes::Bytes::from("SELECT 1"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "
                            models:
                            ",
                        ),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };

        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "shifts".to_string(),
            description: "Updated description".to_string(),
        };

        update_asset_description_internal(database, writer, file_system, request)
            .await
            .unwrap();

        let binding = written_files.borrow();
        let updated_content = binding.get(&"models/schema.yaml".to_string()).unwrap();

        assert_eq!(
            updated_content,
            "models:\n- name: shifts\n  description: Updated description\n"
        );
    }

    #[tokio::test]
    async fn test_update_snapshot_description_without_schema_present() {
        let (writer, _) = setup_file_mocks();

        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = FileSystem {
            files: vec![
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                    },
                ),
                (
                    "models/orders_snapshot.snapshot.sql".to_string(),
                    quary_proto::File {
                        name: "models/orders_snapshot.snapshot.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "
                            SELECT 1",
                        ),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(""),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };

        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "orders_snapshot".to_string(),
            description: "Updated description".to_string(),
        };

        let error_message =
            update_asset_description_internal(database, writer, file_system, request)
                .await
                .unwrap_err();
        assert_eq!(
            error_message,
            "Could not find snapshot definition for snapshot: orders_snapshot"
        );
    }

    #[tokio::test]
    async fn test_update_asset_description_internal_without_schema_present() {
        let (writer, _) = setup_file_mocks();
        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = FileSystem {
            files: vec![(
                "quary.yaml".to_string(),
                quary_proto::File {
                    name: "quary.yaml".to_string(),
                    contents: prost::bytes::Bytes::from("sqliteInMemory: {}".as_bytes()),
                },
            )]
            .into_iter()
            .collect(),
        };

        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "raw_table".to_string(),
            description: "Updated description".to_string(),
        };

        let error_message =
            update_asset_description_internal(database, writer, file_system, request)
                .await
                .unwrap_err();

        assert_eq!(error_message, "Asset raw_table not found");
    }

    #[tokio::test]
    async fn test_update_asset_description_internal_with_model_and_source_same_name() {
        let (writer, _) = setup_file_mocks();

        let database = DatabaseQueryGeneratorSqlite {};
        let file_system = FileSystem {
            files: vec![
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
                        contents: prost::bytes::Bytes::from("SELECT 1"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "
                        models:
                          - name: shifts
                        sources:
                          - name: shifts
                            path: staging.shifts
                        ",
                        ),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };
        let request = UpdateAssetDescriptionRequest {
            project_root: "".to_string(),
            asset_name: "shifts".to_string(),
            description: "Updated description".to_string(),
        };
        let result =
            update_asset_description_internal(database, writer, file_system, request).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid: model/source/snapshot with same name".to_string()
        );
    }

    #[tokio::test]
    async fn test_return_full_sql_for_asset_internal_without_cache() {
        let fixed_time = SystemTime::now();
        let fixed_time_utc: DateTime<Utc> = fixed_time.into();
        let formatted_time = fixed_time_utc.format("%Y-%m-%dT%H:%M:%SZ");

        let database = DatabaseQueryGeneratorDuckDB::new(None, Some(fixed_time));

        let file_system = FileSystem {
            files: vec![
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("duckdb: {}".as_bytes()),
                    },
                ),
                (
                    "models/orders_snapshot.snapshot.sql".to_string(),
                    quary_proto::File {
                        name: "models/orders_snapshot.snapshot.sql".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "
                            SELECT * FROM q.raw_orders",
                        ),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "
                            snapshots:
                            - name: orders_snapshot
                              unique_key: order_id
                              strategy:
                                timestamp:
                                  updated_at: updated_at
                            sources:
                            - name: raw_orders
                              path: test.test             
                            ",
                        ),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };

        let request = ReturnFullSqlForAssetRequest {
            project_root: "".to_string(),
            asset_name: "orders_snapshot".to_string(),
            cache_view_information: Some(CacheViewInformation {
                cache_view: Some(CacheView::DoNotUse(Default::default())),
            }),
        };

        let response = return_full_sql_for_asset_internal(&database, &file_system, request)
            .await
            .unwrap();

        assert_eq!(
            response,
            ReturnFullSqlForAssetResponse {
                full_sql: format!("WITH raw_orders AS (SELECT * FROM test.test) SELECT * FROM (SELECT\n                        *,\n                        CAST ('{formatted_time}' AS TIMESTAMP WITH TIME ZONE) AS quary_valid_from,\n                        CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,\n                        MD5(CAST(CONCAT(order_id, CAST(updated_at AS STRING)) AS STRING)) AS quary_scd_id\n                    FROM (\n                            SELECT * FROM raw_orders)) AS alias").to_string(),
                description: None,
                dag: Some(ProjectDag {
                    nodes: vec![Node {
                        id: "orders_snapshot".to_string(),
                        is_cached: false
                    }, Node {
                        id: "raw_orders".to_string(),
                        is_cached: false
                    }],
                    edges: vec![Edge { to: "orders_snapshot".to_string(), from: "raw_orders".to_string() }]
                }),
            }
        );
    }

    #[tokio::test]
    async fn test_return_full_sql_for_asset_internal_with_cache() {
        let database = DatabaseQueryGeneratorRedshift::new("schema".to_string(), None);

        let file_system = FileSystem {
            files: vec![
                (
                    "quary.yaml".to_string(),
                    quary_proto::File {
                        name: "quary.yaml".to_string(),
                        contents: prost::bytes::Bytes::from("duckdb: {}".as_bytes()),
                    },
                ),
                (
                    "models/orders_snapshot.snapshot.sql".to_string(),
                    quary_proto::File {
                        name: "models/orders_snapshot.snapshot.sql".to_string(),
                        contents: prost::bytes::Bytes::from("SELECT * FROM q.raw_orders"),
                    },
                ),
                (
                    "models/schema.yaml".to_string(),
                    quary_proto::File {
                        name: "models/schema.yaml".to_string(),
                        contents: prost::bytes::Bytes::from(
                            "
                            snapshots:
                            - name: orders_snapshot
                              unique_key: order_id
                              strategy:
                                timestamp:
                                  updated_at: updated_at
                            sources:
                            - name: raw_orders
                              path: test.test
                            ",
                        ),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };

        let request = ReturnFullSqlForAssetRequest {
            project_root: "".to_string(),
            asset_name: "orders_snapshot".to_string(),
            cache_view_information: Some(CacheViewInformation {
                cache_view: Some(CacheView::CacheViewInformation(CacheViewInformationPaths {
                    cache_view_paths: vec!["schema.qqq_orders_snapshot_e70d19a".to_string()],
                })),
            }),
        };

        let response = return_full_sql_for_asset_internal(&database, &file_system, request)
            .await
            .unwrap();

        assert_eq!(
            response,
            ReturnFullSqlForAssetResponse {
                full_sql: "SELECT * FROM schema.qqq_orders_snapshot_e70d19a".to_string(),
                description: None,
                dag: Some(ProjectDag {
                    nodes: vec![
                        Node {
                            id: "orders_snapshot".to_string(),
                            is_cached: true
                        },
                        Node {
                            id: "raw_orders".to_string(),
                            is_cached: false
                        }
                    ],
                    edges: vec![Edge {
                        to: "orders_snapshot".to_string(),
                        from: "raw_orders".to_string()
                    }]
                }),
            }
        );
    }

    #[tokio::test]
    async fn test_return_full_sql_for_asset_internal_chart_asset_reference_model() {
        let database = DatabaseQueryGeneratorDuckDB::new(None, None);
        let filesystem = DuckDBAsset {};

        let response = return_full_sql_for_asset_internal(
            &database,
            &filesystem,
            ReturnFullSqlForAssetRequest {
                project_root: "".to_string(),
                asset_name: "shifts_by_month_bar".to_string(),
                cache_view_information: Some(CacheViewInformation {
                    cache_view: Some(CacheView::DoNotUse(Default::default())),
                }),
            },
        )
        .await
        .unwrap();

        assert!(!response.full_sql.is_empty());
        assert!(!response.description.unwrap().is_empty());
        // TODO Fix the graph
        // assert!(response.dag.unwrap().nodes.iter().find(
        //     |node| node.id == "shifts_by_month_bar"
        // ).is_some())
    }

    #[tokio::test]
    async fn test_return_full_sql_for_asset_internal_chart_raw_sql() {
        let database = DatabaseQueryGeneratorDuckDB::new(None, None);
        let filesystem = DuckDBAsset {};
        let mut filesystem = OverrideFileSystem::new(Box::new(&filesystem));
        let chart_file = ChartFile {
            description: Some("This is the description".to_string()),
            tags: vec![],
            config: None,
            source: Some(chart_file::Source::RawSql("SELECT 1".to_string())),
        };
        let chart_file = chart_file_to_yaml(&chart_file).unwrap();
        filesystem.add_override("models/shifts_by_month_bar_raw_sql.chart.yaml", &chart_file);

        let response = return_full_sql_for_asset_internal(
            &database,
            &filesystem,
            ReturnFullSqlForAssetRequest {
                project_root: "".to_string(),
                asset_name: "shifts_by_month_bar_raw_sql".to_string(),
                cache_view_information: Some(CacheViewInformation {
                    cache_view: Some(CacheView::DoNotUse(Default::default())),
                }),
            },
        )
        .await
        .unwrap();

        assert_eq!(response.full_sql, "SELECT 1".to_string());
        assert_eq!(
            response.description.unwrap(),
            "This is the description".to_string()
        );
        assert!(response.dag.is_none());
    }

    #[tokio::test]
    async fn get_model_table_model_test_empty_project_root() {
        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()), None);
        let file_system = DuckDBAsset;
        let request = GetModelTableRequest {
            project_root: "".to_string(),
            model_name: "shifts_summary".to_string(),
        };

        let response = get_model_table_internal(&database, &file_system, request)
            .await
            .unwrap();
        match response.table.unwrap().table_type.unwrap() {
            TableType::Present(model) => {
                assert_eq!(model.rows.len(), 8)
            }
            _ => panic!("Expected model"),
        }
    }

    #[tokio::test]
    async fn get_model_table_test_with_project_root() {
        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()), None);
        let file_system = files_to_file_system(vec![
            ("quarylabs-123/quary.yaml", "duckdb: {}"),
            ("quarylabs-123/models/shifts_summary.sql", "SELECT 1"),
            (
                "quarylabs-123/models/schema.yaml",
                "
models:
  - name: shifts_summary
    columns:
      - name: column1
                            ",
            ),
        ]);
        let request = GetModelTableRequest {
            project_root: "quarylabs-123".to_string(),
            model_name: "shifts_summary".to_string(),
        };
        let response = get_model_table_internal(&database, &file_system, request)
            .await
            .unwrap();

        match response.table.unwrap().table_type.unwrap() {
            TableType::Present(model) => {
                assert_eq!(model.rows.len(), 1)
            }
            _ => panic!("Expected model"),
        }
    }

    #[tokio::test]
    async fn list_assets_skip_charts() {
        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()), None);
        let file_system = DuckDBAsset {};

        let request = ListAssetsRequest {
            project_root: "".to_string(),
            assets_to_skip: Some(quary_proto::list_assets_request::AssetsToSkip { charts: true }),
        };
        let response = list_assets_internal(&database, &file_system, request)
            .await
            .unwrap();

        // Find the chart
        let chart = response
            .assets
            .iter()
            .find(|asset| asset.name == "shifts_by_month_bar");
        assert!(chart.is_none());
    }

    #[tokio::test]
    async fn list_assets_include_charts() {
        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()), None);
        let file_system = DuckDBAsset {};

        let request = ListAssetsRequest {
            project_root: "".to_string(),
            assets_to_skip: Some(quary_proto::list_assets_request::AssetsToSkip { charts: false }),
        };
        let response = list_assets_internal(&database, &file_system, request)
            .await
            .unwrap();

        // Find the chart
        let chart = response
            .assets
            .iter()
            .find(|asset| asset.name == "shifts_by_month_bar")
            .unwrap();
        assert!(chart.description.is_some());
    }

    fn setup_file_mocks() -> (Writer, Rc<RefCell<HashMap<String, String>>>) {
        let written_files = Rc::new(RefCell::new(HashMap::new()));
        let writer: Writer = Box::new({
            let written_files = Rc::clone(&written_files);
            move |path, content| {
                written_files.borrow_mut().insert(path, content);
                Box::pin(async move { Ok(()) })
            }
        });
        (writer, written_files)
    }
}
