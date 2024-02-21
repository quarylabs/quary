use async_trait::async_trait;
use sqlinference::dialect::Dialect;
use std::fmt::Debug;

pub trait DatabaseQueryGenerator: Debug {
    // For Models section

    /// ModelsDropViewQuery drops a view if it exists.
    fn models_drop_view_query(&self, view_name: &str) -> String {
        let view_name = self.return_full_path_requirement(view_name);
        let view_name = self.database_name_wrapper(&view_name);
        format!("DROP VIEW IF EXISTS {}", view_name)
    }

    /// ModelsCreateViewQuery creates a view if it exists.
    fn models_create_view_query(&self, view_name: &str, original_select_statement: &str) -> String {
        let view_name = self.return_full_path_requirement(view_name);
        let view_name = self.database_name_wrapper(&view_name);
        format!("CREATE VIEW {} AS {}", view_name, original_select_statement)
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
    fn models_drop_view_query(&self, view_name: &str) -> String {
        self.as_ref().models_drop_view_query(view_name)
    }

    fn models_create_view_query(&self, view_name: &str, original_select_statement: &str) -> String {
        self.as_ref()
            .models_create_view_query(view_name, original_select_statement)
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

#[derive(Debug, Clone, PartialEq)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
