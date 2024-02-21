use crate::databases::{base_for_seeds_create_table_specifying_text_type, DatabaseQueryGenerator};
use sqlinference::dialect::Dialect;

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorDuckDB {
    schema: Option<String>,
}

impl DatabaseQueryGeneratorDuckDB {
    pub fn new(schema: Option<String>) -> DatabaseQueryGeneratorDuckDB {
        DatabaseQueryGeneratorDuckDB { schema }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorDuckDB {
    fn seeds_create_table_query(&self, table_name: &str, columns: &[String]) -> String {
        let table_name = self.return_full_path_requirement(table_name);
        base_for_seeds_create_table_specifying_text_type("TEXT", table_name.as_str(), columns)
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        match &self.schema {
            Some(schema) => format!("{}.{}", schema, table_name),
            None => table_name.to_string(),
        }
    }

    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String> {
        let split = full_path.split('.').collect::<Vec<&str>>();
        match (&split[..], &self.schema) {
            ([table_name], None) => Ok(table_name),
            ([schema, table_name], Some(self_schema)) => {
                if schema == self_schema {
                    Ok(table_name)
                } else {
                    Err(format!(
                        "Schema {} does not match {} expected format: schema.table_name",
                        schema, full_path
                    ))
                }
            }
            _ => Err(format!(
                "Table name {} does not contain project ID and dataset ID",
                full_path
            )),
        }
    }

    fn automatic_cache_sql_create_statement(
        &self,
        model: &str,
        model_cache_name: &str,
    ) -> Vec<String> {
        vec![format!(
            "CREATE OR REPLACE VIEW {} AS SELECT * FROM {}",
            self.return_full_path_requirement(model_cache_name),
            self.return_full_path_requirement(model)
        )]
    }

    fn get_dialect(&self) -> &Dialect {
        &Dialect::DuckDB
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        name.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_full_path_requirement() {
        let database = DatabaseQueryGeneratorDuckDB::new(None);
        assert_eq!(
            database.return_full_path_requirement("table_name"),
            "table_name"
        );

        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()));
        assert_eq!(
            database.return_full_path_requirement("table_name"),
            "schema.table_name"
        );
    }

    #[test]
    fn test_return_name_from_full_path() {
        let database = DatabaseQueryGeneratorDuckDB::new(None);
        let query = database.return_name_from_full_path("table_name");
        assert_eq!(query, Ok("table_name"));

        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()));
        let query = database.return_name_from_full_path("schema.table_name");
        assert_eq!(query, Ok("table_name"));

        let query = database.return_name_from_full_path("error.table_name");
        assert!(query.is_err());
    }
}
