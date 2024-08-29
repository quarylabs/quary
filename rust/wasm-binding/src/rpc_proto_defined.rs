use crate::rpc_proto_defined_functions::{
    add_column_test_to_model_or_source_column, add_column_to_model_or_source,
    create_model_chart_file, create_model_schema_entry, generate_project_files,
    generate_source_files, get_model_table, get_project_config, init_files, is_path_empty,
    list_assets, parse_project, remove_column_test_from_model_or_source_column,
    remove_object_column, render_schema, return_dashboard_with_sql, return_data_for_doc_view,
    return_definition_locations_for_sql, return_full_project_dag, return_full_sql_for_asset,
    return_sql_for_injected_model, return_sql_for_seeds_and_models, stringify_project_file,
    update_asset_description, update_model_source_column_description,
};
use crate::rpc_proto_prompt_functions_edit::return_edit_model_prompt;
use crate::rpc_proto_prompt_functions_explain::return_explain_model_prompt;
use crate::rpc_proto_prompt_functions_generate::return_generate_model_prompt;
use crate::rpc_proto_scaffolding::{
    create_file_writer, database_query_generator_from_config, wrapper, wrapper_without_db,
};
use crate::utils::set_panic_hook;
use js_sys::{Function, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn rpc_wrapper_with_database(
    method: &str,
    database: Uint8Array,
    file_writer: JsValue,
    file_reader: Function,
    recursive_file_lister: Function,
    request: Uint8Array,
) -> Result<Uint8Array, String> {
    set_panic_hook();

    let database = database_query_generator_from_config(database)?;
    let file_writer = create_file_writer(file_writer);

    let f = match method {
        "ListAssets" => Ok(wrapper(list_assets)),
        "ParseProject" => Ok(wrapper(parse_project)),
        "RenderSchema" => Ok(wrapper(render_schema)),
        "ReturnFullSqlForAsset" => Ok(wrapper(return_full_sql_for_asset)),
        "ReturnDataForDocView" => Ok(wrapper(return_data_for_doc_view)),
        "ReturnFullProjectDag" => Ok(wrapper(return_full_project_dag)),
        "ReturnSQLForSeedsAndModels" => Ok(wrapper(return_sql_for_seeds_and_models)),
        "GetModelTable" => Ok(wrapper(get_model_table)),
        "CreateModelSchemaEntry" => Ok(wrapper(create_model_schema_entry)),
        "UpdateAssetDescription" => Ok(wrapper(update_asset_description)),
        "AddColumnToModelOrSource" => Ok(wrapper(add_column_to_model_or_source)),
        "UpdateModelOrSourceColumnDescription" => {
            Ok(wrapper(update_model_source_column_description))
        }
        "AddColumnTestToModelOrSourceColumn" => {
            Ok(wrapper(add_column_test_to_model_or_source_column))
        }
        "RemoveColumnTestFromModelOrSourceColumn" => {
            Ok(wrapper(remove_column_test_from_model_or_source_column))
        }
        "GenerateSourceFiles" => Ok(wrapper(generate_source_files)),
        "ReturnDefinitionLocationsForSQL" => Ok(wrapper(return_definition_locations_for_sql)),
        "ReturnSQLForInjectedModel" => Ok(wrapper(return_sql_for_injected_model)),
        "ReturnDashboardWithSql" => Ok(wrapper(return_dashboard_with_sql)),
        "RemoveObjectColumn" => Ok(wrapper(remove_object_column)),
        "ReturnExplainModelPrompt" => Ok(wrapper(return_explain_model_prompt)),
        "ReturnGenerateModelPrompt" => Ok(wrapper(return_generate_model_prompt)),
        "ReturnEditModelPrompt" => Ok(wrapper(return_edit_model_prompt)),
        _ => Err(format!("Unknown method: {}", method)),
    }?;

    let response = f(
        database,
        file_writer,
        file_reader,
        recursive_file_lister,
        request,
    )
    .await?;

    Ok(response)
}

#[wasm_bindgen]
pub async fn rpc_wrapper_without_database(
    method: &str,
    file_writer: JsValue,
    file_reader: Function,
    recursive_file_lister: Function,
    request: Uint8Array,
) -> Result<Uint8Array, String> {
    set_panic_hook();

    let file_writer = create_file_writer(file_writer);

    let f = match method {
        "InitFiles" => Ok(wrapper_without_db(init_files)),
        "IsPathEmpty" => Ok(wrapper_without_db(is_path_empty)),
        "GetProjectConfig" => Ok(wrapper_without_db(get_project_config)),
        "GenerateProjectFiles" => Ok(wrapper_without_db(generate_project_files)),
        "StringifyProjectFile" => Ok(wrapper_without_db(stringify_project_file)),
        "CreateModelChartFile" => Ok(wrapper_without_db(create_model_chart_file)),
        _ => Err(format!("Unknown method: {}", method)),
    }?;

    let response = f(file_writer, file_reader, recursive_file_lister, request).await?;

    Ok(response)
}
