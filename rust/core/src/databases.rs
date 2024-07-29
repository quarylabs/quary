use async_trait::async_trait;
use pbjson_types::Struct;
use quary_proto::snapshot::snapshot_strategy::StrategyType;
use quary_proto::TableAddress;
use sqlinference::dialect::Dialect;
use std::fmt::Debug;

/// CacheStatus defines whether the cache exists and matches the current model, if so it is possible
/// to behave differently
pub enum CacheStatus {
    CachedAndMatching,
    NotMatching,
}

pub trait DatabaseQueryGenerator: SnapshotGenerator + Debug + Sync {
    /// default_materialization_type returns the default materialization type that the database
    /// implements
    fn default_materalization_type(&self) -> MaterializationType {
        MATERIALIZATION_TYPE_VIEW
    }

    /// supported_materialization_types returns the types of materialization that the database
    /// supports
    fn supported_materialization_types(&self) -> &'static [MaterializationType] {
        &[MATERIALIZATION_TYPE_VIEW]
    }

    // For Models section

    /// ModelsDropQuery drops the model with type defined by the materialization setting
    fn models_drop_query(
        &self,
        object_name: &str,
        materialization_type: &Option<String>,
        _: &CacheStatus,
    ) -> Result<Option<String>, String> {
        let object_name = self.return_full_path_requirement(object_name);
        let object_name = self.database_name_wrapper(&object_name);
        match materialization_type {
            None => Ok(Some(
                format!("DROP VIEW IF EXISTS {}", object_name).to_string(),
            )),
            Some(materialization_type) if materialization_type == MATERIALIZATION_TYPE_VIEW => Ok(
                Some(format!("DROP VIEW IF EXISTS {}", object_name).to_string()),
            ),
            _ => Err("Unsupported materialization type".to_string()),
        }
    }

    /// ModelsDropQuery creates the model with type defined by the materialization setting
    fn models_create_query(
        &self,
        object_name: &str,
        original_select_statement: &str,
        materialization_type: &Option<String>,
        _database_config: &Option<Struct>,
        _cache_status: &CacheStatus,
    ) -> Result<Option<Vec<String>>, String> {
        let object_name = self.return_full_path_requirement(object_name);
        let object_name = self.database_name_wrapper(&object_name);
        match materialization_type.as_deref() {
            None => Ok(Some(vec![format!(
                "CREATE VIEW {} AS {}",
                object_name, original_select_statement
            )])),
            Some(MATERIALIZATION_TYPE_VIEW) => Ok(Some(vec![format!(
                "CREATE VIEW {} AS {}",
                object_name, original_select_statement
            )])),
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

    /// get_current_timestamp returns the current timestamp (with TZ + hour in database format)
    fn get_current_timestamp(&self) -> Timestamp {
        panic!("get_current_timestamp not implemented for this database")
    }
}

impl DatabaseQueryGenerator for Box<dyn DatabaseQueryGenerator> {
    fn supported_materialization_types(&self) -> &'static [&'static str] {
        self.as_ref().supported_materialization_types()
    }

    fn default_materalization_type(&self) -> &'static str {
        self.as_ref().default_materalization_type()
    }

    fn models_drop_query(
        &self,
        view_name: &str,
        materialization_type: &Option<String>,
        cache_status: &CacheStatus,
    ) -> Result<Option<String>, String> {
        self.as_ref()
            .models_drop_query(view_name, materialization_type, cache_status)
    }

    fn models_create_query(
        &self,
        object_name: &str,
        original_select_statement: &str,
        materialization_type: &Option<String>,
        database_config: &Option<Struct>,
        cache_status: &CacheStatus,
    ) -> Result<Option<Vec<String>>, String> {
        self.as_ref().models_create_query(
            object_name,
            original_select_statement,
            materialization_type,
            database_config,
            cache_status,
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

    fn get_current_timestamp(&self) -> String {
        self.as_ref().get_current_timestamp()
    }
}

pub trait SnapshotGenerator {
    /// GenerateSnapshotSQL generates the SQL statements to create a snapshot of a table.
    ///
    /// Inputs:
    /// - `path`: The path of the snapshot table to be created.
    /// - `templated_select`: The templated SELECT statement used to build the snapshot table. This means:
    ///   - The raw SELECT statement has been normalised  (i.e. ; have been stripped as a suffix)
    ///   - The variables referenced in the SELECT statement have been replaced with the values defined in the quary.yaml file
    ///   - The q. references have been replaced with the underlying path of the referenced seed or source
    /// - `unique_key`: The column that uniquely identify each row in the snapshot source table.
    /// - `strategy`: The snapshot strategy to be used (e.g., timestamp)
    /// - `table_exists`: A boolean used to check if the snapshot table already exists in the database.
    fn generate_snapshot_sql(
        &self,
        _path: &str,
        _templated_select: &str,
        _unique_key: &str,
        _strategy: &StrategyType,
        _table_exists: Option<bool>,
    ) -> Result<Vec<String>, String> {
        Err("Database does not support snapshots".to_string())
    }

    /// GenerateSnapshotQuery generates the simulated SQL statements to build a snapshot of a table.
    /// This is used to render a simulated view of what the snapshot table would look like once built for the first time using quary snapshot.
    /// It is used by GenerateSnapshotSql to build the initial snapshot table
    ///
    /// This function serves a similar purpose to `generate_snapshot_sql`, but it generates a simulated query instead of the actual SQL statements
    /// required to create and maintain the snapshot table.
    ///
    /// Inputs:
    /// - `templated_select`: The templated SELECT statement used to build the snapshot table. This means:
    ///   - The raw SELECT statement has been normalised  (i.e. ; have been stripped as a suffix)
    ///   - The variables referenced in the SELECT statement have been replaced with the values defined in the quary.yaml file
    ///   - The q. references have been replaced with the underlying path of the referenced seed or source
    /// - `unique_key`: The column that uniquely identify each row in the snapshot source table.
    /// - `strategy`: The snapshot strategy to be used (e.g., timestamp)
    /// - `now`: The current timestamp to be used in the simulated query.
    /// TODO Find a way to remove the &self parameter
    fn generate_snapshot_query(
        &self,
        _templated_select: &str,
        _unique_key: &str,
        _strategy: &StrategyType,
        _now: &str,
    ) -> Result<String, String> {
        Err("Database does not support snapshots".to_string())
    }
}

impl SnapshotGenerator for Box<dyn DatabaseQueryGenerator> {
    fn generate_snapshot_sql(
        &self,
        path: &str,
        templated_select: &str,
        unique_key: &str,
        strategy: &StrategyType,
        table_exists: Option<bool>,
    ) -> Result<Vec<String>, String> {
        self.as_ref().generate_snapshot_sql(
            path,
            templated_select,
            unique_key,
            strategy,
            table_exists,
        )
    }

    fn generate_snapshot_query(
        &self,
        templated_select: &str,
        unique_key: &str,
        strategy: &StrategyType,
        now: &str,
    ) -> Result<String, String> {
        self.as_ref()
            .generate_snapshot_query(templated_select, unique_key, strategy, now)
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

#[derive(Debug, Clone)]
pub struct QueryError {
    pub query: String,
    pub error: String,
}

impl QueryError {
    pub fn new(query: String, error: String) -> Self {
        Self { query, error }
    }
}

#[async_trait]
pub trait DatabaseConnection: Debug {
    /// list_tables returns the names of all the tables accessible in the database, not just the ones
    /// in the schema/dataset that the database connection is connected to. This is useful for
    /// listing tables in other schemas/datasets for building sources
    async fn list_tables(&self) -> Result<Vec<TableAddress>, String>;
    /// list_views returns the names of all the views accessible in the database, not just the ones
    /// in the schema/dataset that the database connection is connected to. This is useful for
    /// listing views in other schemas/datasets for building sources
    async fn list_views(&self) -> Result<Vec<TableAddress>, String>;
    /// list_local_tables returns the names of all the tables in the specific schema/dataset/database
    /// that the database connection is connected to. So if the project is set up to use a specific
    /// schema, only the tables in that schema will be returned.
    async fn list_local_tables(&self) -> Result<Vec<TableAddress>, String>;
    /// list_local_views returns the names of all the views in the schema/dataset/database that the database
    /// connection is connected to. So if the project is set up to use a specific schema, only the
    /// tables in that schema will be returned.
    async fn list_local_views(&self) -> Result<Vec<TableAddress>, String>;
    /// list_columns returns the columns of a table in the order they are defined in the table.
    /// If the table does not exist, an error is returned.
    ///
    /// list_columns should also be able to return the columns for a full path to a table.
    async fn list_columns(&self, path: &str) -> Result<Vec<ColumnWithDetails>, String>;
    /// exec executes a query that does not return any results. This is useful for executing DDL
    /// queries like CREATE TABLE, DROP TABLE, etc.
    async fn exec(&self, query: &str) -> Result<(), String>;

    /// query returns the results of a query as a vector of rows. The first vector is the headers of
    /// the columns. The second vector is the rows.
    async fn query(&self, query: &str) -> Result<QueryResult, QueryError>;

    /// query_generator returns the appropriate query generator
    fn query_generator(&self) -> Box<dyn DatabaseQueryGenerator>;

    /// table_exists returns a boolean whether a table at a given path exists in the database
    /// if the path is fully qualified i.e. analaytics.table_name it will search for this else it will lean on the schema defined in the configuration
    /// Returns an optional boolean: Some(true) if the table exists, Some(false) if it doesn't, and None if the operation is unsupported by the database.
    /// TODO: This should be changed to return a Result<bool, String> instead of an Option<bool>
    async fn table_exists(&self, path: &str) -> Result<Option<bool>, String>;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColumnWithDetails {
    pub name: String,
    pub description: Option<String>,
    pub data_type: Option<String>,
    pub is_nullable: Option<bool>,
    pub is_unique: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QueryResult {
    // The first element of the tuple is the name of the column and the second element is the type
    // of the column. The second is an option as if the type is not known, it will be None.
    pub columns: Vec<(String, Option<String>)>,
    pub rows: Vec<Vec<String>>,
}

impl QueryResult {
    pub fn new(columns: Vec<(String, Option<String>)>, rows: Vec<Vec<String>>) -> Self {
        Self { columns, rows }
    }

    pub fn to_proto(&self) -> Result<quary_proto::QueryResult, String> {
        let values = self
            .columns
            .iter()
            .enumerate()
            .map(|(i, (column, column_type))| {
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
                    r#type: column_type.clone(),
                    values,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(quary_proto::QueryResult { columns: values })
    }
}

// Timestamp is a type alias for a String that represents a formatted database timestamp.
pub type Timestamp = String;

// Materialization Type is a type alias for materializations possible in the databases.
pub type MaterializationType = &'static str;

pub(crate) const MATERIALIZATION_TYPE_VIEW: MaterializationType = "view";
pub(crate) const MATERIALIZATION_TYPE_TABLE: MaterializationType = "table";
pub(crate) const MATERIALIZATION_TYPE_MATERIALIZED_VIEW: MaterializationType = "materialized_view";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_result_to_proto_success() {
        // Arrange
        let columns = ["id".to_string(), "name".to_string()];
        let rows = vec![
            vec!["1".to_string(), "Alice".to_string()],
            vec!["2".to_string(), "Bob".to_string()],
        ];
        let query_result = QueryResult::new(
            columns
                .iter()
                .map(|x| (x.to_string(), None))
                .collect::<Vec<(String, Option<String>)>>(),
            rows,
        );

        // Act
        let proto_result = query_result.to_proto();

        // Assert
        let expected_columns = vec![
            quary_proto::QueryResultColumn {
                name: "id".to_string(),
                r#type: None,
                values: vec!["1".to_string(), "2".to_string()],
            },
            quary_proto::QueryResultColumn {
                name: "name".to_string(),
                r#type: None,
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
        let columns = ["id".to_string(), "name".to_string()];
        // The second row is missing a value
        let rows = vec![
            vec!["1".to_string(), "Alice".to_string()],
            vec!["2".to_string()],
        ];
        let query_result = QueryResult::new(
            columns
                .iter()
                .map(|x| (x.to_string(), None))
                .collect::<Vec<(String, Option<String>)>>(),
            rows,
        );

        let proto_result = query_result.to_proto();

        assert!(proto_result.is_err());
    }
}
