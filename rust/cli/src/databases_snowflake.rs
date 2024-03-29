use arrow_array::array;
use async_trait::async_trait;
use quary_core::database_snowflake::{
    validate_snowfalke_account_identifier, DatabaseQueryGeneratorSnowflake,
};
use quary_core::databases::{
    DatabaseConnection, DatabaseQueryGenerator, QueryResult, TableAddress,
};
use regex::Regex;
use snowflake_api::QueryResult::{Arrow, Json};
use snowflake_api::SnowflakeApi;
use std::fmt::Debug;
use std::sync::Arc;

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
                    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'TABLE' AND TABLE_CATALOG = '{}' AND TABLE_SCHEMA = '{}'",
                    self.database, self.schema
                )
                .as_str(),
            )
            .await?;
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
                    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'VIEW' AND TABLE_CATALOG = '{}' AND TABLE_SCHEMA = '{}'",
                    self.database, self.schema
                )
                .as_str(),
            )
            .await?;
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

    async fn list_columns(&self, table: &str) -> Result<Vec<String>, String> {
        let tables = self
            .query(
                format!(
                    "SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = '{}'",
                    table
                )
                .as_str(),
            )
            .await?;
        Ok(tables
            .rows
            .iter()
            .map(|row| row[0].clone())
            .collect::<Vec<String>>())
    }

    async fn exec(&self, sql: &str) -> Result<(), String> {
        self.client
            .exec(sql)
            .await
            .map_err(|e| format!("Failed to run query '{}': {}", sql, e))?;
        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, String> {
        let rs = self
            .client
            .exec(query)
            .await
            .map_err(|e| format!("client error '{:?}'", e))?;

        return match rs {
            Arrow(results) => match &results[..] {
                [first] => {
                    let columns = first
                        .schema()
                        .fields()
                        .iter()
                        .map(|f| f.name().clone())
                        .collect::<Vec<String>>();
                    let rows = convert_array_to_vec_string(first.columns())?;
                    Ok(QueryResult { columns, rows })
                }
                _ => {
                    return Err("Multiple results not implemented".to_string());
                }
            },
            Json(_) => Err("Json results not implemented".to_string()),
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
}

pub fn convert_array_to_vec_string(
    array: &[Arc<dyn array::Array>],
) -> Result<Vec<Vec<String>>, String> {
    let num_rows = array[0].len();
    let num_columns = array.len();
    let mut rows = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {
        let row = vec!["".to_string(); num_columns];
        rows.push(row);
    }

    for i in 0..num_rows {
        for j in 0..array.len() {
            let array = &array[j];
            if let Some(string_array) = array.as_any().downcast_ref::<array::StringArray>() {
                rows[i][j] = string_array.value(i).to_string();
            } else if let Some(int32_array) = array.as_any().downcast_ref::<array::Int32Array>() {
                rows[i][j] = int32_array.value(i).to_string();
            } else if let Some(int64_array) = array.as_any().downcast_ref::<array::Int64Array>() {
                rows[i][j] = int64_array.value(i).to_string();
            } else if let Some(float32_array) = array.as_any().downcast_ref::<array::Float32Array>()
            {
                rows[i][j] = float32_array.value(i).to_string();
            } else if let Some(float64_array) = array.as_any().downcast_ref::<array::Float64Array>()
            {
                rows[i][j] = float64_array.value(i).to_string();
            } else if let Some(boolean_array) = array.as_any().downcast_ref::<array::BooleanArray>()
            {
                rows[i][j] = boolean_array.value(i).to_string();
            } else if let Some(date_array) = array.as_any().downcast_ref::<array::Date64Array>() {
                rows[i][j] = date_array.value(i).to_string();
            } else if let Some(date_array) = array.as_any().downcast_ref::<array::Date32Array>() {
                rows[i][j] = date_array.value(i).to_string();
            } else {
                return Err("Unsupported array type".to_string());
            }
        }
    }

    // Example for a specific array type, e.g., StringArray
    Ok(rows)
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn test_snowflake() {
//         let snowflake = Snowflake::new(
//             "actual_details.eu-west-2.aws",
//             "COMPUTE_WH",
//             "TEST_DATABASE",
//             "TEST_SCHEMA",
//             "USERNAME",
//             // Some("TEST_ROLE"),
//             None,
//             "rubbish_details",
//         )
//         .unwrap();
//
//         let tables = snowflake.list_tables().await.unwrap();
//         println!("Tables: {:?}", tables);
//         assert_eq!(tables.len(), 1);
//
//         let views = snowflake.list_views().await.unwrap();
//         println!("Views: {:?}", views);
//         assert_eq!(views.len(), 1);
//
//         let columns = snowflake.list_columns(&tables[0]).await.unwrap();
//         println!("Columns: {:?}", columns);
//         assert_eq!(columns.len(), 2);
//     }
// }
