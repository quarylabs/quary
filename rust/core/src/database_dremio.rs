use sqruff_lib_core::dialects::Dialect;
use sqruff_lib_dialects::postgres;

use crate::databases::{DatabaseQueryGenerator, SnapshotGenerator};

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorDremio {
    pub space: String,
    pub folder_path: String,
}

const SPACE_DEAFULT: &str = "@user";
const FOLDER_PATH_DEFAULT: &str = "no_schema";

impl DatabaseQueryGeneratorDremio {
    pub fn new(space: Option<String>, folder_path: Option<String>) -> Self {
        Self {
            space: space.unwrap_or_else(|| SPACE_DEAFULT.to_string()),
            folder_path: folder_path.unwrap_or_else(|| FOLDER_PATH_DEFAULT.to_string()),
        }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorDremio {
    fn get_name(&self) -> &'static str {
        "dremio"
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        format!(
            "\"{}\".\"{}\".\"{}\"",
            self.space, self.folder_path, table_name
        )
    }

    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String> {
        if full_path.starts_with(format!("{}.{}", self.space, self.folder_path).as_str()) {
            Ok(full_path
                .split('.')
                .last()
                .ok_or("table name does not contain a dot, which is not allowed in SQLite")?)
        } else {
            Err(format!(
                "table name {} contains a dot, which is not allowed in SQLite",
                full_path
            ))
        }
    }

    fn automatic_cache_sql_create_statement(
        &self,
        model: &str,
        model_cache_name: &str,
    ) -> Vec<String> {
        let drop = format!(
            "DROP VIEW IF EXISTS {}",
            self.return_full_path_requirement(model_cache_name)
        );
        let create = format!(
            "CREATE VIEW {} AS SELECT * FROM {}",
            self.return_full_path_requirement(model_cache_name),
            self.return_full_path_requirement(model)
        );
        vec![drop, create]
    }

    fn get_dialect(&self) -> Dialect {
        postgres::dialect()
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        format!("{}", name)
    }
}

impl SnapshotGenerator for DatabaseQueryGeneratorDremio {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_name_from_full_path() {
        // TODO Need to add test cases for "SPACE"."FODLER_PATH".table_name
        let database = DatabaseQueryGeneratorDremio::new(
            Some("SPACE".to_string()),
            Some("FOLDER_PATH".to_string()),
        );

        let query =
            database.return_name_from_full_path("SPACE.FOLDER_PATH.qqq_shifts_summary_fbas143");
        assert_eq!(query, Ok("qqq_shifts_summary_fbas143"));
    }

    #[test]
    fn return_full_name_requirement() {
        let database = DatabaseQueryGeneratorDremio::new(
            Some("SPACE".to_string()),
            Some("FOLDER_PATH".to_string()),
        );
        let query = database.return_full_path_requirement("qqq_shifts_summary_fbas143");
        assert_eq!(
            query,
            "\"SPACE\".\"FOLDER_PATH\".\"qqq_shifts_summary_fbas143\""
        );
    }

    #[test]
    fn test_automatic_cache_sql_create_statement() {
        let database = DatabaseQueryGeneratorDremio::new(
            Some("SPACE".to_string()),
            Some("FOLDER_PATH".to_string()),
        );
        let model = "shifts_summary";
        let model_cache_name = "qqq_shifts_summary_fbas143";
        let sql = database.automatic_cache_sql_create_statement(model, model_cache_name);

        assert_eq!(
            sql,
            vec!["DROP VIEW IF EXISTS \"SPACE\".\"FOLDER_PATH\".\"qqq_shifts_summary_fbas143\"", "CREATE VIEW \"SPACE\".\"FOLDER_PATH\".\"qqq_shifts_summary_fbas143\" AS SELECT * FROM \"SPACE\".\"FOLDER_PATH\".\"shifts_summary\""]
        );
    }

    #[test]
    fn test_return_table_view_from_full_path() {
        let database = DatabaseQueryGeneratorDremio::new(
            Some("SPACE".to_string()),
            Some("FOLDER_PATH".to_string()),
        );
        let query =
            database.return_name_from_full_path("SPACE.FOLDER_PATH.qqq_shifts_summary_fbas143");
        assert_eq!(query, Ok("qqq_shifts_summary_fbas143"));

        let query = database.return_name_from_full_path("schema.asdfasdf.table_name");
        assert!(query.is_err());
    }
}
