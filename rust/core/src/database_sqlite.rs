use crate::databases::{DatabaseQueryGenerator, SnapshotGenerator};
use sqruff_lib_core::dialects::Dialect;
use sqruff_lib_dialects::sqlite;

#[derive(Debug, Default)]
pub struct DatabaseQueryGeneratorSqlite;

impl DatabaseQueryGenerator for DatabaseQueryGeneratorSqlite {
    fn get_name(&self) -> &'static str {
        "sqlite"
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        table_name.to_string()
    }

    /// seeds_create_table_query returns the query to create a table in SQLite. Sqlite doesn't support
    /// schemas, so the table name is the full path.
    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String> {
        if full_path.contains('.') {
            Err(format!(
                "table name {} contains a dot, which is not allowed in SQLite",
                full_path
            ))
        } else {
            Ok(full_path)
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
        sqlite::dialect()
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        format!("`{}`", name)
    }
}

impl SnapshotGenerator for DatabaseQueryGeneratorSqlite {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automatic_cache_sql_create_statement() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let model = "shifts_summary";
        let model_cache_name = "qqq_shifts_summary_fbas143";
        let sql = database.automatic_cache_sql_create_statement(model, model_cache_name);

        assert_eq!(
            sql,
            vec![
                "DROP VIEW IF EXISTS qqq_shifts_summary_fbas143",
                "CREATE VIEW qqq_shifts_summary_fbas143 AS SELECT * FROM shifts_summary",
            ]
        );
    }

    #[test]
    fn test_return_table_view_from_full_path() {
        let database = DatabaseQueryGeneratorSqlite::default();
        let query = database.return_name_from_full_path("table_name");
        assert_eq!(query, Ok("table_name"));

        let query = database.return_name_from_full_path("schema.table_name");
        assert_eq!(
            query,
            Err(
                "table name schema.table_name contains a dot, which is not allowed in SQLite"
                    .to_string()
            )
        );
    }
}
