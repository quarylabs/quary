use async_trait::async_trait;
use sqlinference::dialect::Dialect;
use std::fmt::Debug;

pub trait DatabaseQueryGenerator: Debug + Sync {
    /// Validates the materialization flag defined in the model schema.yaml file.
    fn validate_materialization_type(
        &self,
        materialization_type: &Option<String>,
    ) -> Result<(), String> {
        match materialization_type {
            None => Ok(()),
            Some(t) if t == "view" || t == "materialized_view" || t == "table" => Ok(()),
            // Some(materialization_type) if materialization_type == "view" => Ok(()),
            // Some(materialization_type) if materialization_type == "materialized_view" => Ok(()),
            // Some(materialization_type) if materialization_type == "table" => Ok(()),
            Some(materialization_type) => Err(format!(
                "Materialization type {} is not supported. Supported types are 'view'.",
                materialization_type
            )),
        }
    }

    // For Models section
    fn models_refresh_query(
        &self,
        object_name: &str,
        // original_select_statement: &str,
        materialization_type: &Option<String>,
    ) -> Result<String, String> {
        let object_name = self.return_full_path_requirement(object_name);
        let object_name = self.database_name_wrapper(&object_name);
        match materialization_type.as_deref() {
            Some("materialized_view") => Ok(format!(
                "REFRESH MATERIALIZED VIEW {}",
                object_name
            )),
            Some("view") | Some("table") => Ok(format!(
                "REFRESH MATERIALIZED VIEW {:?}", materialization_type)),
            _ => Err("Only materialized views are refreshed".to_string()),
        }
    }
    /// ModelsDropQuery drops the model with type defined by the materialization setting
    fn models_drop_query(
        &self,
        object_name: &str,
        materialization_type: &Option<String>,
    ) -> Result<String, String> {
        let object_name = self.return_full_path_requirement(object_name);
        let object_name = self.database_name_wrapper(&object_name);
        match materialization_type {
            None => Ok(format!("DROP VIEW IF EXISTS {}", object_name).to_string()),
            Some(materialization_type) if materialization_type == "view" => {
                Ok(format!("DROP VIEW IF EXISTS {}", object_name).to_string())
            }
            _ => Err("Unsupported materialization type".to_string()),
        }
    }

    /// ModelsDropQuery creates the model with type defined by the materialization setting
    fn models_create_query(
        &self,
        object_name: &str,
        original_select_statement: &str,
        materialization_type: &Option<String>,
    ) -> Result<String, String> {
        let object_name = self.return_full_path_requirement(object_name);
        let object_name = self.database_name_wrapper(&object_name);
        match materialization_type.as_deref() {
            None => Ok(format!(
                "CREATE VIEW {} AS {}",
                object_name, original_select_statement
            )),
            Some("view") => Ok(format!(
                "CREATE VIEW {} AS {}",
                object_name, original_select_statement
            )),
            _ => Err("Unsupported materialization type".to_string()),
        }
    }

    // For Seeds section

    /// SeedsDropTableQuery drops a table if it exists.
    fn seeds_drop_table_query(&self, table_name: &str) -> String {
        format!(
            "DROP TABLE IF EXISTS {}",
            self.return_full_path_requirement(table_name)
        )
    }

    /// SeedsCreateTableQuery drops a table if it exists where the columns are Text/String equivalent.
    fn seeds_create_table_query(&self, table_name: &str, columns: &[String]) -> String {
        let table_name = self.return_full_path_requirement(table_name);
        base_for_seeds_create_table_specifying_text_type("TEXT", table_name.as_str(), columns)
    }

    /// SeedsInsertIntoTableQuery inserts values into a table.
    fn seeds_insert_into_table_query(
        &self,
        table_name: &str,
        columns: &[String],
        values: &[Vec<String>],
    ) -> String {
        let table_name = self.return_full_path_requirement(table_name);
        let columns = columns
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let values = values
            .iter()
            .map(|x| {
                format!(
                    "'{}'",
                    x.iter()
                        .map(|y| self.escape_seed_value(y))
                        .collect::<Vec<String>>()
                        .join("', '")
                )
            })
            .collect::<Vec<String>>()
            .join("), (");
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, columns, values
        )
    }

    fn escape_seed_value(&self, seed_value: &str) -> String {
        seed_value.replace('\'', "''")
    }

    // For Helpers section

    /// ReturnFullPathRequirement takes in the name of the target table and prefixes it with any necessary schema/paths
    /// to make it a full path.
    fn return_full_path_requirement(&self, table_name: &str) -> String;

    /// return_name_from_full_path takes in the full path of a table and returns the table/view name.
    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String>;

    /// automatic_cache_sql_create_statement returns the SQL statements to create the automatic cache table.
    fn automatic_cache_sql_create_statement(
        &self,
        model: &str,
        model_cache_name: &str,
    ) -> Vec<String>;

    /// get_dialect returns the dialect of the database.
    fn get_dialect(&self) -> &Dialect;

    /// database_name_wrapper returns a full path or name wrapped in quotes that work for the specific database
    fn database_name_wrapper(&self, name: &str) -> String;
}

impl DatabaseQueryGenerator for Box<dyn DatabaseQueryGenerator> {
    fn validate_materialization_type(
        &self,
        materialization_type: &Option<String>,
    ) -> Result<(), String> {
        self.as_ref()
            .validate_materialization_type(materialization_type)
    }
    fn models_drop_query(
        &self,
        view_name: &str,
        materialization_type: &Option<String>,
    ) -> Result<String, String> {
        self.as_ref()
            .models_drop_query(view_name, materialization_type)
    }

    fn models_create_query(
        &self,
        view_name: &str,
        original_select_statement: &str,
        materialization_type: &Option<String>,
    ) -> Result<String, String> {
        self.as_ref().models_create_query(
            view_name,
            original_select_statement,
            materialization_type,
        )
    }

    fn models_refresh_query(
        &self,
        view_name: &str,
        // original_select_statement: &str,
        materialization_type: &Option<String>,
    ) -> Result<String, String> {
        self.as_ref().models_refresh_query(
            view_name,
            // original_select_statement,
            materialization_type,
        )
    }

    fn seeds_drop_table_query(&self, table_name: &str) -> String {
        self.as_ref().seeds_drop_table_query(table_name)
    }

    fn seeds_create_table_query(&self, table_name: &str, columns: &[String]) -> String {
        self.as_ref().seeds_create_table_query(table_name, columns)
    }

    fn seeds_insert_into_table_query(
        &self,
        table_name: &str,
        columns: &[String],
        values: &[Vec<String>],
    ) -> String {
        self.as_ref()
            .seeds_insert_into_table_query(table_name, columns, values)
    }

    fn escape_seed_value(&self, seed_value: &str) -> String {
        self.as_ref().escape_seed_value(seed_value)
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        self.as_ref().return_full_path_requirement(table_name)
    }

    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String> {
        self.as_ref().return_name_from_full_path(full_path)
    }

    fn automatic_cache_sql_create_statement(
        &self,
        model: &str,
        model_cache_name: &str,
    ) -> Vec<String> {
        self.as_ref()
            .automatic_cache_sql_create_statement(model, model_cache_name)
    }

    fn get_dialect(&self) -> &Dialect {
        self.as_ref().get_dialect()
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        self.as_ref().database_name_wrapper(name)
    }
}

pub fn base_for_seeds_create_table_specifying_text_type(
    text_type: &str,
    table_name: &str,
    columns: &[String],
) -> String {
    let values = columns
        .iter()
        .map(|x| format!("{} {}", x, text_type))
        .collect::<Vec<String>>()
        .join(", ");
    format!("CREATE TABLE {} ({})", table_name, values)
}

/// TableAddress is a struct that represents a table in a database. It contains the name of the table and the full path.
#[derive(Debug, Clone, PartialEq)]
pub struct TableAddress {
    pub name: String,
    pub full_path: String,
}

#[async_trait]
pub trait DatabaseConnection: Debug {
    /// list_tables returns the names of all the tables in the schema/dataset/database that the database connection is
    /// connected to.
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String>;
    /// list_views returns the names of all the views in the schema/dataset/database that the database connection is
    /// connected to.
    async fn list_views(&self) -> Result<Vec<TableAddress>, String>;
    /// list_materialized views returns the names of all the views in the schema/dataset/database that the database connection is
    /// connected to.
    async fn list_materialized_views(&self) -> Result<Vec<TableAddress>, String>;
    /// list_columns returns the columns of a table in the order they are defined in the table. If the table does not
    /// exist, an error is returned.
    async fn list_columns(&self, table: &str) -> Result<Vec<String>, String>;
    async fn exec(&self, query: &str) -> Result<(), String>;
    /// query returns the results of a query as a vector of rows. The first vector is the headers of
    /// the columns. The second vector is the rows.
    async fn query(&self, query: &str) -> Result<QueryResult, String>;
    /// query_generator returns the appropriate query generator
    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator>;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl QueryResult {
    pub fn new(columns: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { columns, rows }
    }

    pub fn to_proto(&self) -> Result<quary_proto::QueryResult, String> {
        let values = self
            .columns
            .iter()
            .enumerate()
            .map(|(i, column)| {
                let values = self
                    .rows
                    .iter()
                    .enumerate()
                    .map(|(j, row)| {
                        let value = row.get(i).ok_or_else(|| {
                            format!("row {} does not have a value for column {}", j, column)
                        })?;
                        Ok::<String, String>(value.to_string())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(quary_proto::QueryResultColumn {
                    name: column.clone(),
                    r#type: None,
                    values,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(quary_proto::QueryResult { columns: values })
    }
}

#[cfg(test)]
mod tests {
    use crate::database_sqlite::DatabaseQueryGeneratorSqlite;

    use super::*;

    #[test]
    fn test_query_result_to_proto_success() {
        // Arrange
        let columns = vec!["id".to_string(), "name".to_string()];
        let rows = vec![
            vec!["1".to_string(), "Alice".to_string()],
            vec!["2".to_string(), "Bob".to_string()],
        ];
        let query_result = QueryResult::new(columns, rows);

        // Act
        let proto_result = query_result.to_proto();

        // Assert
        let expected_columns = vec![
            quary_proto::QueryResultColumn {
                name: "id".to_string(),
                values: vec!["1".to_string(), "2".to_string()],
            },
            quary_proto::QueryResultColumn {
                name: "name".to_string(),
                values: vec!["Alice".to_string(), "Bob".to_string()],
            },
        ];
        let expected = Ok(quary_proto::QueryResult {
            columns: expected_columns,
        });

        assert_eq!(proto_result, expected);
    }

    #[test]
    fn test_query_result_to_proto_error_missing_value() {
        let columns = vec!["id".to_string(), "name".to_string()];
        // The second row is missing a value
        let rows = vec![
            vec!["1".to_string(), "Alice".to_string()],
            vec!["2".to_string()],
        ];
        let query_result = QueryResult::new(columns, rows);

        let proto_result = query_result.to_proto();

        assert!(proto_result.is_err());
    }

    #[test]
    fn test_validate_materialization_type_success() {
        let database = Box::new(DatabaseQueryGeneratorSqlite {});
        let materialization_type = Some("view".to_string());
        let result = database.validate_materialization_type(&materialization_type);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_materialization_type_undefined() {
        let database = Box::new(DatabaseQueryGeneratorSqlite {});
        let materialization_type = None;
        let result = database.validate_materialization_type(&materialization_type);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_materialization_type_error() {
        let database = Box::new(DatabaseQueryGeneratorSqlite {});
        let materialization_type = Some("garbage".to_string());
        let result = database.validate_materialization_type(&materialization_type);
        assert!(result.is_err());
    }
}
