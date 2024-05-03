use async_trait::async_trait;
use quary_core::database_snowflake::{
    validate_snowfalke_account_identifier, DatabaseQueryGeneratorSnowflake,
};
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::TableAddress;
use regex::Regex;
use snowflake_api::QueryResult::{Arrow, Json};
use snowflake_api::SnowflakeApi;
use std::fmt::Debug;
use std::sync::Arc;
use crate::databases_duckdb::convert_array_to_vec_string;

pub struct Snowflake {
    client: SnowflakeApi,
    database: String,
    schema: String,
}

impl Debug for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Snowflake").finish()
    }
}

const IS_VALID_ROLE_REGEX: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*$";
fn is_valid_role_name(name: &str) -> bool {
    #[allow(clippy::unwrap_used)]
    let re = Regex::new(IS_VALID_ROLE_REGEX).unwrap();
    re.is_match(name)
}

const IS_VALID_USERNAME_REGEX: &str = r"^[a-zA-Z_][a-zA-Z0-9_.]*$";
fn is_valid_username(name: &str) -> bool {
    #[allow(clippy::unwrap_used)]
    let re = Regex::new(IS_VALID_USERNAME_REGEX).unwrap();
    re.is_match(name)
}

const IS_VALID_SCHEMA_REGEX: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*$";
fn is_valid_schema_name(name: &str) -> bool {
    #[allow(clippy::unwrap_used)]
    let re = Regex::new(IS_VALID_SCHEMA_REGEX).unwrap();
    re.is_match(name)
}

const IS_VALID_DATABASE_REGEX: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*$";
fn is_valid_database_name(name: &str) -> bool {
    #[allow(clippy::unwrap_used)]
    let re = Regex::new(IS_VALID_DATABASE_REGEX).unwrap();
    re.is_match(name)
}

impl Snowflake {
    pub fn new(
        account_identifier: &str,
        warehouse: &str,
        database: &str,
        schema: &str,
        username: &str,
        role: Option<&str>,
        password: &str,
    ) -> Result<Snowflake, String> {
        if !is_valid_database_name(database) {
            return Err(format!(
                "Database name {} does not match regex {}",
                database, IS_VALID_DATABASE_REGEX
            ));
        }
        if !is_valid_schema_name(schema) {
            return Err(format!(
                "Schema name {} does not match regex {}",
                schema, IS_VALID_SCHEMA_REGEX
            ));
        }
        if !is_valid_username(username) {
            return Err(format!(
                "Username {} does not match regex {}",
                username, IS_VALID_USERNAME_REGEX
            ));
        }
        if let Some(role) = role {
            if !is_valid_role_name(role) {
                return Err(format!(
                    "Role {} does not match regex {}",
                    role, IS_VALID_ROLE_REGEX
                ));
            }
        }

        validate_snowfalke_account_identifier(account_identifier)?;
        let client = SnowflakeApi::with_password_auth(
            account_identifier,
            Some(warehouse),
            Some(database),
            Some(schema),
            username,
            role,
            password,
        )
        .map_err(|e| format!("Failed to create Snowflake client: {}", e))?;
        Ok(Snowflake {
            client,
            database: database.to_string(),
            schema: schema.to_string(),
        })
    }
}

#[async_trait]
impl DatabaseConnection for Snowflake {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let results = self
            .query(
                format!(
                    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'TABLE' AND TABLE_CATALOG = '{}'",
                    self.database,
                )
                    .as_str(),
            )
            .await
            .map_err(|e| format!("Failed to list tables: {:?}", e))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| {
                let row = row
                    .first()
                    .ok_or("Failed to get first column of row".to_string())?;
                Ok(TableAddress {
                    name: row.to_string(),
                    full_path: format!("{}.{}.{}", self.database, self.schema, row),
                })
            })
            .collect::<Result<Vec<_>, String>>()?)
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let results = self
            .query(
                format!(
                    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'VIEW' AND TABLE_CATALOG = '{}'",
                    self.database,
                )
                    .as_str(),
            )
            .await
            .map_err(|e| format!("Failed to list views: {:?}", e))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| {
                let row = row
                    .first()
                    .ok_or("Failed to get first column of row".to_string())?;
                Ok(TableAddress {
                    name: row.to_string(),
                    full_path: format!("{}.{}.{}", self.database, self.schema, row),
                })
            })
            .collect::<Result<Vec<_>, String>>()?)
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let results = self
            .query(
                format!(
                    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'TABLE' AND TABLE_CATALOG = '{}' AND TABLE_SCHEMA = '{}'",
                    self.database, self.schema
                )
                .as_str(),
            )
            .await
            .map_err(|e| format!("Failed to list tables: {:?}", e))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| {
                let row = row
                    .first()
                    .ok_or("Failed to get first column of row".to_string())?;
                Ok(TableAddress {
                    name: row.to_string(),
                    full_path: format!("{}.{}.{}", self.database, self.schema, row),
                })
            })
            .collect::<Result<Vec<_>, String>>()?)
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let results = self
            .query(
                format!(
                    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'VIEW' AND TABLE_CATALOG = '{}' AND TABLE_SCHEMA = '{}'",
                    self.database, self.schema
                )
                .as_str(),
            )
            .await.map_err(|e| format!("Failed to list views: {:?}", e))?;
        Ok(results
            .rows
            .into_iter()
            .map(|row| {
                let row = row
                    .first()
                    .ok_or("Failed to get first column of row".to_string())?;
                Ok(TableAddress {
                    name: row.to_string(),
                    full_path: format!("{}.{}.{}", self.database, self.schema, row),
                })
            })
            .collect::<Result<Vec<_>, String>>()?)
    }

    async fn list_columns(&self, table: &str) -> Result<Vec<ColumnWithDetails>, String> {
        let tables = self
            .query(
                format!(
                    "SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = '{}'",
                    table
                )
                .as_str(),
            )
            .await
            .map_err(|e| format!("Failed to list columns: {:?}", e))?;
        Ok(tables
            .rows
            .iter()
            .map(|row| row[0].clone())
            .map(|row| ColumnWithDetails {
                name: row,
                ..Default::default()
            })
            .collect::<Vec<ColumnWithDetails>>())
    }

    async fn exec(&self, sql: &str) -> Result<(), String> {
        self.client
            .exec(sql)
            .await
            .map_err(|e| format!("Failed to run query '{}': {}", sql, e))?;
        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let rs =
            self.client.exec(query).await.map_err(|e| {
                QueryError::new(query.to_string(), format!("client error '{:?}'", e))
            })?;

        return match rs {
            Arrow(results) => match &results[..] {
                [first] => {
                    let columns = first
                        .schema()
                        .fields()
                        .iter()
                        .map(|f| f.name().clone())
                        .collect::<Vec<String>>();
                    let rows = convert_array_to_vec_string(first.columns())
                        .map_err(|e| QueryError::new(query.to_string(), e))?;
                    Ok(QueryResult {
                        columns: columns.into_iter().map(|c| (c, None)).collect(),
                        rows,
                    })
                }
                _ => {
                    return Err(QueryError::new(
                        query.to_string(),
                        "Multiple results not implemented".to_string(),
                    ));
                }
            },
            Json(_) => Err(QueryError::new(
                query.to_string(),
                "Json results not implemented".to_string(),
            )),
            snowflake_api::QueryResult::Empty => Ok(QueryResult {
                columns: vec![],
                rows: vec![],
            }),
        };
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorSnowflake::new(
            self.database.to_string(),
            self.schema.to_string(),
        ))
    }

    async fn table_exists(&self, _path: &str) -> Result<Option<bool>, String> {
        Ok(None) // not implemented
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn failing_new() {
        let invalid_username = Snowflake::new(
            "actual_details.eu-west-2.aws",
            "COMPUTE_WH",
            "TEST_DATABASE",
            "TEST_SCHEMA",
            "123_@£$!@£$INVALID",
            // Some("TEST_ROLE"),
            None,
            "PASSWORD",
        );
        assert!(invalid_username.is_err());

        let invalid_role = Snowflake::new(
            "actual_details.eu-west-2.aws",
            "COMPUTE_WH",
            "TEST_DATABASE",
            "TEST_SCHEMA",
            "USERNAME",
            Some("123_@£$!@£$INVALID"),
            "PASSWORD",
        );
        assert!(invalid_role.is_err());

        let invalid_database = Snowflake::new(
            "actual_details.eu-west-2.aws",
            "COMPUTE_WH",
            "123_@£$!@£$INVALID",
            "TEST_SCHEMA",
            "USERNAME",
            // Some("TEST_ROLE"),
            None,
            "PASSWORD",
        );
        assert!(invalid_database.is_err());

        let invalid_schema = Snowflake::new(
            "actual_details.eu-west-2.aws",
            "COMPUTE_WH",
            "TEST_DATABASE",
            "123_@£$!@£$INVALID",
            "USERNAME",
            // Some("TEST_ROLE"),
            None,
            "PASSWORD",
        );
        assert!(invalid_schema.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_snowflake() {
        let snowflake = Snowflake::new(
            "actual_details.eu-west-2.aws",
            "COMPUTE_WH",
            "TEST_DATABASE",
            "TEST_SCHEMA",
            "USERNAME",
            // Some("TEST_ROLE"),
            None,
            "rubbish_details",
        )
        .unwrap();

        let tables = snowflake.list_tables().await.unwrap();
        println!("Tables: {:?}", tables);
        assert_eq!(tables.len(), 1);

        let views = snowflake.list_views().await.unwrap();
        println!("Views: {:?}", views);
        assert_eq!(views.len(), 1);

        let columns = snowflake.list_columns(&tables[0].full_path).await.unwrap();
        println!("Columns: {:?}", columns);
        assert_eq!(columns.len(), 2);
    }
}
