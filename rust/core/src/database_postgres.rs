use crate::databases::{base_for_seeds_create_table_specifying_text_type, DatabaseQueryGenerator};
use sqlinference::dialect::Dialect;

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorPostgres {
    schema: String,
}

impl DatabaseQueryGeneratorPostgres {
    pub fn new(schema: String) -> DatabaseQueryGeneratorPostgres {
        DatabaseQueryGeneratorPostgres { schema }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorPostgres {
    fn validate_materialization_type(
        &self,
        materialization_type: &Option<String>,
    ) -> Result<(), String> {
        match materialization_type {
            None => Ok(()),
            Some(t) if t == "view" || t == "materialized_view" || t == "table" => Ok(()),
            Some(materialization_type) => Err(format!(
                "Materialization type {} is not supported. Supported types are 'view', 'table', 'materialized_view'.",
                materialization_type
            )),
        }
    }

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
            Some(materialization_type) if materialization_type == "table" => {
                Ok(format!("DROP TABLE IF EXISTS {}", object_name).to_string())
            }
            Some(materialization_type) if materialization_type == "materialized_view" => {
                Ok(format!("DROP MATERIALIZED VIEW IF EXISTS {}", object_name).to_string())
            }
            Some(materialization_type) => Err(format!(
                "Unsupported materialization type: {}",
                materialization_type
            )),
        }
    }

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
            Some("table") => Ok(format!(
                "CREATE TABLE {} AS {}",
                object_name, original_select_statement
            )),
            Some("materialized_view") => Ok(format!(
                "CREATE MATERIALIZED VIEW {} AS {}",
                // "REFRESH MATERIALIZED VIEW {}",
                // object_name
                object_name, original_select_statement
            )),
            _ => Err("Unsupported materialization type".to_string()),
        }
    }

    fn models_refresh_query(
        &self,
        object_name: &str,
        original_select_statement: &str,
        materialization_type: &Option<String>,
    ) -> Result<String, String> {
        let object_name = self.return_full_path_requirement(object_name);
        let object_name = self.database_name_wrapper(&object_name);
        match materialization_type.as_deref() {
            Some("materialized_view") => Ok(format!(
                "REFRESH MATERIALIZED VIEW {} as {}",
                object_name, original_select_statement
            )),
            Some("view") | Some("table") => Ok(format!(
                "REFRESH MATERIALIZED VIEW {:?}", materialization_type)),
            _ => Err("Only materialized views are refreshed".to_string()),
        }
    }

    fn seeds_create_table_query(&self, table_name: &str, columns: &[String]) -> String {
        let table_name = self.return_full_path_requirement(table_name);
        base_for_seeds_create_table_specifying_text_type("TEXT", &table_name, columns)
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        format!("{}.{}", self.schema, table_name)
    }

    fn seeds_drop_table_query(&self, table_name: &str) -> String {
        format!(
            "DROP TABLE IF EXISTS {} CASCADE",
            self.return_full_path_requirement(table_name)
        )
    }

    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String> {
        let split = full_path.split('.').collect::<Vec<&str>>();
        match split.as_slice() {
            [schema, table_name] => {
                if schema == &self.schema {
                    Ok(table_name)
                } else {
                    Err(format!(
                        "Schema {} does not match expected value {}",
                        schema, self.schema
                    ))
                }
            }
            _ => Err(format!(
                "Table name {} does not contain the expected schema",
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
        &Dialect::Postgres
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        name.to_string()
    }
}
