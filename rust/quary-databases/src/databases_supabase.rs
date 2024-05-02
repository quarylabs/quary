use async_trait::async_trait;
use postgrest::Postgrest;
use quary_core::database_postgres::DatabaseQueryGeneratorPostgres;
use quary_core::databases::{
    ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator, QueryError, QueryResult,
};
use quary_proto::TableAddress;
use serde_json::Value;
use serde_json::json;
use std::fmt;

pub struct Supabase {
    client: Postgrest,
    schema: String,
}

impl Supabase {
    pub fn new(url: &str, key: &str, schema: &str) -> Self {
        let client = Postgrest::new(url).insert_header("apiKey", key).schema(schema);
        Self {
            client,
            schema: schema.to_string(),
        }
    }
}

impl Supabase {
    pub async fn list_table_like_query(
        &self,
        where_clause: &str,
    ) -> Result<Vec<TableAddress>, String> {
        let query = format!(
            "
            SELECT
                CASE
                    WHEN table_schema <> lower(table_schema) THEN '\"' || table_schema || '\"'
                    ELSE table_schema
                END AS table_schema,
                CASE
                    WHEN table_name <> lower(table_name) THEN '\"' || table_name || '\"'
                    ELSE table_name
                END AS table_name
            FROM information_schema.tables
            WHERE {}
            ORDER BY table_schema, table_name
            ",
            where_clause
        );

        let response = self
            .client
            .rpc(
                "quary",
                &json!({
                    "sql_statement": query,
                }).to_string(),
            )
            .execute()
            .await
            .map_err(|e| e.to_string())?;

        let json_response: Value = response.json().await.map_err(|e| e.to_string())?;

        print!("{:?}", json_response);
        let tables: Vec<TableAddress> = json_response
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                print!("{:?}", row);
                let table_schema = row["table_schema"].as_str().unwrap().to_string();
                let table_name = row["table_name"].as_str().unwrap().to_string();

                TableAddress {
                    name: table_name.clone(),
                    full_path: format!("{}.{}", table_schema, table_name),
                }
            })
            .collect();

        Ok(tables)
    }
}

#[async_trait]
impl DatabaseConnection for Supabase {
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String> {
        let response = self
        .client
        .rpc(
            "execute_dynamic_sql",
            &json!({
                "sql_command": "SELECT * FROM information_schema.tables",
            }).to_string(),
        )
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    let json_response: Value = response.json().await.map_err(|e| e.to_string())?;

    print!("{:?}", json_response);
    let tables: Vec<TableAddress> = json_response
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            print!("{:?}", row);
            let table_schema = row["table_schema"].as_str().unwrap().to_string();
            let table_name = row["table_name"].as_str().unwrap().to_string();

            TableAddress {
                name: table_name.clone(),
                full_path: format!("{}.{}", table_schema, table_name),
            }
        })
        .collect();

    Ok(tables)
    }

    async fn list_views(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = "table_type = 'VIEW' AND table_schema != 'information_schema' AND table_schema != 'pg_catalog'";
        self.list_table_like_query(where_clause).await
    }

    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = format!(
            "table_schema = '{}' AND table_type = 'BASE TABLE'",
            self.schema
        );
        self.list_table_like_query(where_clause.as_str()).await
    }

    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String> {
        let where_clause = format!("table_schema = '{}' AND table_type = 'VIEW'", self.schema);
        self.list_table_like_query(where_clause.as_str()).await
    }

    async fn list_columns(&self, table: &str) -> Result<Vec<ColumnWithDetails>, String> {
        let (schema, table) = match table.split('.').collect::<Vec<&str>>().as_slice() {
            [schema, table] => Ok((schema.to_string(), table.to_string())),
            [table] => Ok((self.schema.to_string(), table.to_string())),
            _ => Err(format!(
                "Table name {} does not contain the expected schema",
                table
            )),
        }?;

        let query = format!(
            "
            SELECT
                CASE
                    WHEN c.column_name <> lower(c.column_name) THEN '\"' || c.column_name || '\"'
                    ELSE c.column_name
                END AS column_name,
                pgd.description AS column_comment,
                c.is_nullable,
                CASE
                    WHEN tc.constraint_type = 'UNIQUE' THEN 'YES'
                    ELSE 'NO'
                END AS is_unique,
                c.data_type
            FROM
                information_schema.columns c
            LEFT JOIN
                pg_catalog.pg_statio_all_tables AS st ON c.table_schema = st.schemaname AND c.table_name = st.relname
            LEFT JOIN
                pg_catalog.pg_description pgd ON pgd.objoid = st.relid AND pgd.objsubid = c.ordinal_position
            LEFT JOIN
                information_schema.key_column_usage kcu ON c.table_schema = kcu.table_schema
                AND c.table_name = kcu.table_name
                AND c.column_name = kcu.column_name
            LEFT JOIN
                information_schema.table_constraints tc ON kcu.constraint_schema = tc.constraint_schema
                AND kcu.constraint_name = tc.constraint_name
                AND tc.constraint_type = 'UNIQUE'
            WHERE
                c.table_name = '{}' AND c.table_schema = '{}'
            ORDER BY
                c.ordinal_position
            ",
            table,
            schema
        );

        let response = self
            .client
            .rpc(
                "query",
                &json!({
                    "sql_statement": query,
                }).to_string(),
            )
            .execute()
            .await
            .map_err(|e| e.to_string())?;

        let json_response: Value = response.json().await.map_err(|e| e.to_string())?;

        let columns: Vec<ColumnWithDetails> = json_response
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|row| {
                let description = row["column_comment"].as_str().map(|s| s.to_string());
                let is_nullable = row["is_nullable"].as_str().unwrap_or("").to_string();
                let is_unique = row["is_unique"].as_str().unwrap_or("").to_string();
                let data_type = row["data_type"].as_str().unwrap_or("").to_string();

                ColumnWithDetails {
                    name: row["column_name"].as_str().unwrap_or("").to_string(),
                    description,
                    data_type: Some(data_type),
                    is_nullable: Some(is_nullable == "YES"),
                    is_unique: Some(is_unique == "YES"),
                }
            })
            .collect();

        Ok(columns)
    }
    
    async fn exec(&self, query: &str) -> Result<(), String> {
        self.client
            .rpc(
                "query",
                &json!({
                    "sql_statement": query,
                }).to_string(),
            )
            .execute()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn query(&self, query: &str) -> Result<QueryResult, QueryError> {
        let response = self
            .client
            .rpc(
                "query",
                &json!({
                    "sql_statement": query,
                }).to_string(),
            )
            .execute()
            .await
            .map_err(|e| QueryError::new(e.to_string(), query.to_string()))?;

        let json_response: Value = response.json().await.map_err(|e| {
            QueryError::new(e.to_string(), query.to_string())
        })?;

        if json_response.is_null() || json_response.as_array().unwrap_or(&Vec::new()).is_empty() {
            return Ok(QueryResult {
                columns: Vec::new(),
                rows: Vec::new(),
            });
        }

        let columns: Vec<String> = json_response
            .as_array()
            .unwrap()
            .first()
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.to_string())
            .collect();

        let rows: Vec<Vec<String>> = json_response
            .as_array()
            .unwrap()
            .iter()
            .map(|row| {
                columns
                    .iter()
                    .map(|col| {
                        let value = &row[col];
                        match value {
                            Value::Null => "NULL".to_string(),
                            Value::Bool(v) => v.to_string(),
                            Value::Number(v) => v.to_string(),
                            Value::String(v) => v.to_string(),
                            _ => "Unsupported type".to_string(),
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(QueryResult {
            columns: columns.into_iter().map(|c| (c, None)).collect(),
            rows,
        })
    }

    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator> {
        Box::new(DatabaseQueryGeneratorPostgres::new(
            self.schema.clone(),
            None,
        ))
    }

    async fn table_exists(&self, _path: &str) -> Result<Option<bool>, String> {
        Ok(None) // not implemented
    }
}

impl fmt::Debug for Supabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Supabase")
            .field("schema", &self.schema)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use prost::bytes::Bytes;
    use quary_core::database_redshift::DatabaseQueryGeneratorRedshift;
    use quary_core::project::{
        parse_project, project_and_fs_to_sql_for_snapshots, project_and_fs_to_sql_for_views,
    };
    use quary_core::project_tests::return_tests_sql;
    use quary_proto::{File, FileSystem};
    use std::time::SystemTime;

    #[tokio::test]
    #[ignore]
    async fn test_supabase_basic() {
        let quary_postgres = Supabase::new("", "", "");


        let tables = quary_postgres.list_tables().await.unwrap();
        assert_eq!(
            tables,
            vec![
                TableAddress {
                    name: "test_table".to_string(),
                    full_path: "public.test_table".to_string(),
                },
                TableAddress {
                    name: "wrong_table".to_string(),
                    full_path: "public.wrong_table".to_string(),
                },
            ]
        );

        let views = quary_postgres.list_local_views().await.unwrap();
        assert_eq!(
            views,
            vec![
                TableAddress {
                    name: "test_view".to_string(),
                    full_path: "public.test_view".to_string(),
                },
                TableAddress {
                    name: "wrong_view".to_string(),
                    full_path: "public.wrong_view".to_string(),
                },
            ]
        );

        let columns = quary_postgres.list_columns("test_table").await.unwrap();
        assert_eq!(
            columns,
            vec!["id", "name"]
                .into_iter()
                .map(|name| {
                    ColumnWithDetails {
                        name: name.to_string(),
                        is_nullable: Some(true),
                        is_unique: Some(false),
                        ..Default::default()
                    }
                })
                .collect::<Vec<ColumnWithDetails>>()
        );

        let result = quary_postgres
            .query("SELECT * FROM test_table")
            .await
            .unwrap();
        assert_eq!(
            result
                .columns
                .iter()
                .map(|(column, _)| column)
                .collect::<Vec<_>>(),
            vec!["id", "name"]
        );
        assert_eq!(result.rows, vec![vec!["1", "test"], vec!["2", "rubbish"]]);
    }

}