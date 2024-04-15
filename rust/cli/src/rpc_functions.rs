use crate::commands::{Cli, RpcArgs};
use crate::rpc_scaffolding::rpc_wrapper;
use crate::{generate_sources, get_config_file};
use quary_core::databases::DatabaseConnection;
use quary_databases::databases_connection::database_from_config;
use quary_proto::{
    ExecRequest, ExecResponse, ListColumnsRequest, ListColumnsResponse, ListSourcesRequest,
    ListSourcesResponse, ListTablesRequest, ListTablesResponse, ListViewsRequest,
    ListViewsResponse, QueryRequest, QueryResponse,
};

pub async fn rpc(args: &Cli, rpc_args: &RpcArgs) -> Result<(), String> {
    let config = get_config_file(&args.project_file)?;
    let database = database_from_config(&config).await?;

    let method = &rpc_args.method;
    let request = &rpc_args.request;

    let call = match method.as_str() {
        "ListTables" => Ok(rpc_wrapper(list_tables)),
        "ListViews" => Ok(rpc_wrapper(list_views)),
        "ListColumns" => Ok(rpc_wrapper(list_columns)),
        "Execute" => Ok(rpc_wrapper(execute)),
        "Query" => Ok(rpc_wrapper(query)),
        "ListSources" => Ok(rpc_wrapper(list_sources)),
        _ => Err("error Method not found"),
    }?;

    let response = call(request.to_string(), database).await?;

    println!("{}", response);

    Ok(())
}

async fn list_tables(
    _: ListTablesRequest,
    database: Box<dyn DatabaseConnection>,
) -> Result<ListTablesResponse, String> {
    let tables = database
        .list_local_tables()
        .await
        .map_err(|e| format!("Failed to list tables: {}", e))?;
    let response = ListTablesResponse { tables };
    Ok(response)
}

async fn list_views(
    _: ListViewsRequest,
    database: Box<dyn DatabaseConnection>,
) -> Result<ListViewsResponse, String> {
    let views = database
        .list_views()
        .await
        .map_err(|e| format!("Failed to list views: {}", e))?;
    let response = ListViewsResponse { views };
    Ok(response)
}

async fn list_columns(
    req: ListColumnsRequest,
    database: Box<dyn DatabaseConnection>,
) -> Result<ListColumnsResponse, String> {
    let columns = database
        .list_columns(req.table_name.as_str())
        .await
        .map_err(|e| format!("Failed to list columns: {}", e))?;
    let columns = columns.into_iter().map(|c| c.name).collect();
    let response = ListColumnsResponse { columns };
    Ok(response)
}

async fn execute(
    req: ExecRequest,
    database: Box<dyn DatabaseConnection>,
) -> Result<ExecResponse, String> {
    database
        .exec(&req.query)
        .await
        .map_err(|e| format!("Failed to execute query: {}", e))?;
    Ok(ExecResponse {})
}

async fn query(
    req: QueryRequest,
    database: Box<dyn DatabaseConnection>,
) -> Result<QueryResponse, String> {
    let result = database
        .query(&req.query)
        .await
        .map_err(|e| format!("Failed to execute query: {:?}", e))?;

    Ok(QueryResponse {
        result: Some(result.to_proto()?),
    })
}

async fn list_sources(
    _: ListSourcesRequest,
    database: Box<dyn DatabaseConnection>,
) -> Result<ListSourcesResponse, String> {
    let sources = generate_sources(&database)
        .await
        .map_err(|e| format!("Failed to list sources: {}", e))?;
    Ok(ListSourcesResponse { sources })
}
