use async_trait::async_trait;
use gcp_bigquery_client::auth::Authenticator;
use gcp_bigquery_client::error::BQError;
use gcp_bigquery_client::model::query_request::QueryRequest;
use gcp_bigquery_client::Client;
use google_cloud_auth::project::{create_token_source, Config};
use google_cloud_auth::token_source::TokenSource;
use quary_core::database_bigquery::DatabaseQueryGeneratorBigQuery;
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::TableAddress;
use std::fmt::Debug;
use std::sync::Arc;
use yup_oauth2::error::AuthErrorCode;

pub struct BigQuery {
    project_id: String,
    dataset_id: String,
    client: Client,
}

impl Debug for BigQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BigQuery")
            .field("project_id", &self.project_id)
            .field("dataset_id", &self.dataset_id)
            .finish()
    }
}

#[derive(Clone)]
struct AccessTokenProvider {
    token_source: Arc<dyn TokenSource>,
}

impl AccessTokenProvider {
    pub fn new(token_source: Arc<dyn TokenSource>) -> Self {
        AccessTokenProvider { token_source }
    }
}

// TODO This should be removed and favoured for proper token source
#[derive(Clone, Debug)]
struct AccessTokenProviderHolder {
    token: String,
}

impl AccessTokenProviderHolder {
    pub fn new(token: String) -> Self {
        AccessTokenProviderHolder { token }
    }
}

#[async_trait]
impl Authenticator for AccessTokenProviderHolder {
    async fn access_token(&self) -> Result<String, BQError> {
        return Ok(self.token.clone());
    }
}

#[async_trait]
impl Authenticator for AccessTokenProvider {
    async fn access_token(&self) -> Result<String, BQError> {
        let token_source = &self.token_source;
        let token = token_source.token().await.map_err(|_| {
            BQError::AuthError(yup_oauth2::error::AuthError {
                error: AuthErrorCode::ExpiredToken,
                error_description: None,
                error_uri: None,
            })
        })?;
        Ok(token.access_token)
    }
}

impl BigQuery {
    pub async fn new(
        project_id: String,
        dataset_id: String,
        access_token: Option<String>,
    ) -> Result<Self, String> {
        if let Some(access_token) = access_token {
            let authenticator = AccessTokenProviderHolder::new(access_token);
            let client = Client::from_authenticator(Arc::new(authenticator));
            Ok(BigQuery {
                client,
                project_id,
                dataset_id,
            })
        } else {
            let audience = "https://bigquery.googleapis.com/";
            let scopes = ["https://www.googleapis.com/auth/bigquery"];
            let config = Config {
                // audience is required only for service account jwt-auth
                // https://developers.google.com/identity/protocols/oauth2/service-account#jwt-auth
                audience: Some(audience),
                // scopes is required only for service account Oauth2
                // https://developers.google.com/identity/protocols/oauth2/service-account
                scopes: Some(&scopes),
                sub: None,
            };
            // Assuming `create_token_source` returns a `Result<impl TokenSource, Error>`
            let ts = create_token_source(config)
                .await
                .map_err(|e| format!("Failed to create token source: {}", e))?;

            // Create an Arc<dyn TokenSource> directly without boxing
            let token_source: Arc<dyn TokenSource> = Arc::from(ts);

            let authenticator = AccessTokenProvider::new(token_source);

            let client = Client::from_authenticator(Arc::new(authenticator));

            Ok(BigQuery {
                client,
                project_id,
                dataset_id,
            })
        }
    }
}

impl BigQuery {
    // retrieves all the datasets in the project_id
    async fn get_all_table_like_things(
        &self,
    ) -> Result<Vec<gcp_bigquery_client::model::table_list_tables::TableListTables>, String> {
        let mut collected_tables = vec![];

        let datasets = self
            .client
            .dataset()
            .list(&self.project_id, Default::default())
            .await
            .map_err(|e| format!("Failed to list datasets: {}", e))?;

        for dataset in datasets.datasets {
            let dataset_id = dataset.dataset_reference.dataset_id;
            let mut next_page_token: Option<String> = None;

            loop {
                let mut options = gcp_bigquery_client::table::ListOptions::default();
                if let Some(next_page_token) = &next_page_token {
                    options = options.page_token(next_page_token.to_string());
                }

                let tables = self
                    .client
                    .table()
                    .list(&self.project_id, &dataset_id, options)
                    .await
                    .map_err(|e| {
                        format!("Failed to list tables in dataset {}: {}", dataset_id, e)
                    })?;

                collected_tables.extend(tables.tables.unwrap_or_default());

                if tables.next_page_token.is_none() {
                    break;
                }
                next_page_token = tables.next_page_token;
            }
        }

        Ok(collected_tables)
    }

    // retrieves all the datasets in the target quary schema
    async fn get_all_local_table_like_things(
        &self,
    ) -> Result<Vec<gcp_bigquery_client::model::table_list_tables::TableListTables>, String> {
        let mut next_page_token: Option<String> = None;
        let mut collected_tables = vec![];
        loop {
            // TODO Need to wrap around next page token
            let mut options = gcp_bigquery_client::table::ListOptions::default();
            if let Some(next_page_token) = &next_page_token {
                options = options.page_token(next_page_token.to_string());
            }
            let tables = self
                .client
                .table()
                .list(&self.project_id, &self.dataset_id, options)
                .await
                .map_err(|e| format!("Failed to list tables: {}", e))?;
            collected_tables.extend(tables.tables.unwrap_or_default());
            if tables.next_page_token.is_none() {
                break;
            }
            next_page_token = tables.next_page_token;
        }
        Ok(collected_tables)
    }
}

#[async_trait]
impl DatabaseConnection for BigQuery {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let tables = self.get_all_table_like_things().await?;

        let table_addresses = tables
            .into_iter()
            .filter(|t| t.r#type == Some("TABLE".to_string()))
            .map(|t| {
                let dataset_id = t.table_reference.dataset_id.clone();
                let name = t
                    .friendly_name
                    .clone()
                    .unwrap_or_else(|| t.table_reference.table_id.clone());
                TableAddress {
                    full_path: format!("{}.{}.{}", self.project_id, dataset_id, name),
                    name,
                }
            })
            .collect();

        Ok(table_addresses)
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let tables = self.get_all_table_like_things().await?;

        let view_addresses = tables
            .into_iter()
            .filter(|t| t.r#type == Some("VIEW".to_string()))
            .map(|t| {
                let dataset_id = t.table_reference.dataset_id.clone();
                let name = t
                    .friendly_name
                    .clone()
                    .unwrap_or_else(|| t.table_reference.table_id.clone());
                TableAddress {
                    full_path: format!("{}.{}.{}", self.project_id, dataset_id, name),
                    name,
                }
            })
            .collect();

        Ok(view_addresses)
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        self.get_all_local_table_like_things()
            .await?
            .iter()
            .filter(|t| t.r#type == Some("TABLE".to_string()))
            .map(|t| {
                let name = t
                    .friendly_name
                    .clone()
                    .ok_or("Failed to get friendly name of table".to_string())?;
                Ok(TableAddress {
                    full_path: format!("{}.{}.{}", self.project_id, self.dataset_id, name),
                    name,
                })
            })
            .collect()
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        self.get_all_local_table_like_things().await?
            .iter()
            .filter(|t| t.r#type == Some("VIEW".to_string()))
            .map(|t| {
                let friendly_name = t
                    .friendly_name
                    .clone()
                    .ok_or("Failed to get friendly name of table".to_string())?;
                Ok(TableAddress {
                    full_path: format!(
                        "{}.{}.{}",
                        self.project_id, self.dataset_id, friendly_name,
                    ),
                    name: friendly_name,
                })
            })
            .collect()
    }

    async fn list_columns(&self, path: &str) -> Result<Vec<ColumnWithDetails>, String> {
        let bigquery_path_parts: Vec<&str> = path.split('.').collect();
        if bigquery_path_parts.len() != 3 {
            return Err(format!("Invalid fully qualified path: {}", path));
        }

        let (project_id, dataset_id, table_id) = (
            bigquery_path_parts[0],
            bigquery_path_parts[1],
            bigquery_path_parts[2],
        );

        let tables = self
            .client
            .table()
            .get(project_id, dataset_id, table_id, None)
            .await
            .map_err(|e| format!("Failed to get table {}: {}", path, e))?;
        let fields = tables.schema.fields.unwrap_or_default();
        let columns = fields
            .iter()
            .map(|f| f.name.clone())
            .map(|row| ColumnWithDetails {
                name: row,
                ..Default::default()
            })
            .collect::<Vec<_>>();
        Ok(columns)
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        self.client
            .job()
            .query(&self.project_id, QueryRequest::new(query))
            .await
            .map_err(|e| format!("Failed to run query '{}': {}", query, e))?;

        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let mut rs = self
            .client
            .job()
            .query(self.project_id.as_str(), QueryRequest::new(query))
            .await
            .map_err(|e| {
                QueryError::new(query.to_string(), format!("Failed to run query: {:?}", e))
            })?;

        let mut rows: Vec<Vec<String>> = vec![];
        let columns = rs.column_names();
        while rs.next_row() {
            let mut row: Vec<String> = vec![];
            for column in &columns {
                let value = rs
                    .get_string_by_name(column)
                    .map_err(|e| {
                        QueryError::new(query.to_string(), format!("Failed to get value: {:?}", e))
                    })?
                    .ok_or(QueryError::new(
                        query.to_string(),
                        format!("Failed to get value: {}", column),
                    ))?;
                row.push(value);
            }
            rows.push(row);
        }

        Ok(QueryResult {
            columns: columns.into_iter().map(|c| (c, None)).collect(),
            rows,
        })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorBigQuery::new(
            self.project_id.to_string(),
            self.dataset_id.to_string(),
        ))
    }

    async fn table_exists(&self, _path: &str) -> Result<Option<bool>, String> {
        Ok(None) // not implemented
    }
}
