use crate::database_arrow_helper::record_batches_to_result;
use arrow_flight::sql::client::FlightSqlServiceClient;
use async_trait::async_trait;
use base64::encode;
use futures_util::StreamExt;
use quary_core::database_dremio::DatabaseQueryGeneratorDremio;
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::connection_config::ConnectionConfigDremio;
use quary_proto::TableAddress;
use tonic::transport::Channel;

#[derive(Debug)]
pub struct Dremio {
    dremio: DatabaseQueryGeneratorDremio,
    auth: DremioAuth,
    use_ssl: bool,
    host: String,
    port: String,
}

#[derive(Debug)]
pub enum DremioAuth {
    UsernamePassword(String, String),
    UsernamePersonalAccessToken(String, String),
}

impl Dremio {
    pub async fn new(
        config: ConnectionConfigDremio,
        auth: DremioAuth,
        use_ssl: bool,
        host: String,
        port: String,
    ) -> Result<Self, String> {
        let dremio =
            DatabaseQueryGeneratorDremio::new(config.dremio_space, config.dremio_space_folder);
        Ok(Dremio {
            dremio,
            auth,
            use_ssl,
            host,
            port,
        })
    }

    pub async fn get_client(&self) -> Result<FlightSqlServiceClient<Channel>, String> {
        // Add Timeout
        let channel = Channel::from_shared(format!("http://{}:{}", self.host, self.port))
            .map_err(|e| format!("Failed to create channel to Dremio {}", e))?
            .connect()
            .await
            .map_err(|e| format!("Failed to connect to Dremio {}", e))?;
        if self.use_ssl {
            return Err("SSL not implemented".to_string());
        }
        let client = match &self.auth {
            DremioAuth::UsernamePassword(username, password) => {
                let auth_header = format!("{}:{}", username, password);
                let encoded_auth = encode(auth_header);
                let auth_value = format!("Basic {}", encoded_auth);
                let mut intercepted_client = FlightSqlServiceClient::new(channel);
                intercepted_client.set_header("authorization", auth_value);
                Ok(intercepted_client)
            }
            DremioAuth::UsernamePersonalAccessToken(_, _) => {
                Err("Personal Access Token authentication not implemented".to_string())
            }
        }?;
        Ok(client)
    }
}

#[async_trait]
impl DatabaseConnection for Dremio {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let query = r#"
SELECT TABLE_SCHEMA, TABLE_NAME 
FROM INFORMATION_SCHEMA."TABLES"
WHERE TABLE_TYPE = 'TABLE'
ORDER BY TABLE_SCHEMA, TABLE_NAME
"#;
        let results = self
            .query(query)
            .await
            .map_err(|e| format!("Failed to list views from Dremio {}", e.error))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[1].clone(),
                full_path: format!("{}.{}", row[0], row[1]),
            })
            .collect())
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let query = r#"
SELECT TABLE_SCHEMA, TABLE_NAME 
FROM INFORMATION_SCHEMA.VIEWS
ORDER BY TABLE_SCHEMA, TABLE_NAME
"#;
        let results = self
            .query(query)
            .await
            .map_err(|e| format!("Failed to list views from Dremio {:?}", e))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[1].clone(),
                full_path: format!("{}.{}", row[0], row[1]),
            })
            .collect())
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let query = format!(
            r#"
SELECT TABLE_SCHEMA, TABLE_NAME 
FROM INFORMATION_SCHEMA."TABLES"
WHERE TABLE_SCHEMA = '{}.{}' AND TABLE_TYPE = 'TABLE'
ORDER BY TABLE_SCHEMA, TABLE_NAME
"#,
            self.dremio.space, self.dremio.folder_path
        );
        let results = self
            .query(&query)
            .await
            .map_err(|e| format!("Failed to list views from Dremio {}", e.error))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[1].clone(),
                full_path: format!("{}.{}", row[0], row[1]),
            })
            .collect())
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let query = format!(
            r#"
SELECT TABLE_SCHEMA, TABLE_NAME 
FROM INFORMATION_SCHEMA.VIEWS 
WHERE TABLE_SCHEMA = '{}.{}'
ORDER BY TABLE_SCHEMA, TABLE_NAME
"#,
            self.dremio.space, self.dremio.folder_path
        );
        let results = self
            .query(&query)
            .await
            .map_err(|e| format!("Failed to list views from Dremio {}", e.error))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| TableAddress {
                name: row[1].clone(),
                full_path: format!("{}.{}", row[0], row[1]),
            })
            .collect())
    }

    async fn list_columns(&self, path: &str) -> Result<Vec<ColumnWithDetails>, String> {
        todo!()
    }

    async fn exec(&self, query: &str) -> Result<(), String> {
        let mut client = self.get_client().await?;
        let info = client
            .execute(query.to_string(), None)
            .await
            .map_err(|e| format!("Failed to execute query {}", e))?;

        let tickets: Vec<_> = info.endpoint.into_iter().filter_map(|e| e.ticket).collect();
        if tickets.len() != 1 {
            return Err(format!("Expected 1 ticket, got {}", tickets.len()));
        }
        let ticket = tickets.first().unwrap().clone();
        let mut stream = client
            .do_get(ticket)
            .await
            .map_err(|e| format!("Failed to get ticket {}", e))?;

        while let Some(data) = stream.next().await {
            data.map_err(|e| format!("Failed to get data from stream {}", e))?;
        }
        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let mut client = self
            .get_client()
            .await
            .map_err(|e| QueryError::new(query.to_string(), e))?;
        let info = client
            .execute(query.to_string(), None)
            .await
            .map_err(|e| QueryError::new(query.to_string(), e.to_string()))?;

        let tickets: Vec<_> = info.endpoint.into_iter().filter_map(|e| e.ticket).collect();
        if tickets.len() != 1 {
            return Err(QueryError::new(
                query.to_string(),
                format!("Expected 1 ticket, got {}", tickets.len()),
            ));
        }
        let ticket = tickets.first().unwrap().clone();
        let mut stream = client
            .do_get(ticket)
            .await
            .map_err(|e| QueryError::new(query.to_string(), e.to_string()))?;

        let mut rbs = vec![];
        while let Some(data) = stream.next().await {
            let data = data.map_err(|e| QueryError::new(query.to_string(), e.to_string()))?;
            rbs.push(data);
        }
        record_batches_to_result(query, rbs)
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(self.dremio.clone())
    }

    async fn table_exists(&self, path: &str) -> Result<Option<bool>, String> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    // The following tests expect a local running docker version of dremio with user admin, password "fht4jyx9HAY!jxk1ydg"
    // It should be running on the following ports
    // docker run -p 9047:9047 -p 31010:31010 -p 32010:32010 -p 45678:45678 dremio/dremio-oss
    // 1. Create test space
    // 2. Create test folder inside the test space
    // 3. Create the samples source
    use super::*;

    async fn get_client() -> Dremio {
        let config = ConnectionConfigDremio {
            object_storage_source: None,
            object_storage_path: None,
            dremio_space: Some("test".to_string()),
            dremio_space_folder: Some("test".to_string()),
        };
        let auth =
            DremioAuth::UsernamePassword("admin".to_string(), "fht4jyx9HAY!jxk1ydg".to_string());
        let dremio = Dremio::new(
            config,
            auth,
            false,
            "localhost".to_string(),
            "32010".to_string(),
        )
        .await
        .unwrap();
        dremio
    }

    #[tokio::test]
    #[ignore]
    async fn simple_create_view() {
        let dremio = get_client().await;
        let exec = dremio.exec("SELECT * FROM sys.options").await;

        assert!(exec.is_ok(), "Failed to execute query {:?}", exec);

        let exec = dremio
            .exec(r#"CREATE OR REPLACE VIEW test.test.test_view AS SELECT * FROM Samples."samples.dremio.com"."NYC-taxi-trips.csv""#)
            .await
            .unwrap();

        let create_view_when_exists = dremio
            .exec(r#"CREATE VIEW test.test.test_view AS SELECT * FROM Samples."samples.dremio.com"."NYC-taxi-trips.csv""#)
            .await;
        assert!(create_view_when_exists.is_err())
    }

    #[tokio::test]
    #[ignore]
    async fn simple_get_data() {
        let dremio = get_client().await;
        let exec = dremio.exec("SELECT * FROM sys.options").await;

        assert!(exec.is_ok(), "Failed to execute query {:?}", exec);

        let exec = dremio
            .query(
                r#"
           SELECT * FROM Samples."samples.dremio.com"."NYC-taxi-trips.csv" LIMIT 100
            "#,
            )
            .await
            .unwrap();

        assert_eq!(
            exec.rows.len(),
            100,
            "Expected 100 rows, got {}",
            exec.rows.len()
        );
    }

    #[tokio::test]
    #[ignore]
    async fn list_local_view() {
        let dremio = get_client().await;
        let views = dremio.list_local_views().await.unwrap();

        assert_eq!(
            views,
            vec![TableAddress {
                name: "test_view".to_string(),
                full_path: "test.test.test_view".to_string()
            }]
        )
    }

    #[tokio::test]
    #[ignore]
    async fn list_local_tables() {
        let dremio = get_client().await;
        let views = dremio.list_local_tables().await.unwrap();

        assert!(views.is_empty())
    }

    #[tokio::test]
    #[ignore]
    async fn list_tables() {
        let dremio = get_client().await;
        let views = dremio.list_tables().await.unwrap();
        assert!(views.len() > 0)
    }

    #[tokio::test]
    #[ignore]
    async fn list_views() {
        let dremio = get_client().await;
        let views = dremio.list_views().await.unwrap();
        assert!(views.len() > 0)
    }
}
