use std::time::SystemTime;

use chrono::{DateTime, Utc};

#[cfg(target_arch = "wasm32")]
use js_sys::Date;
use pbjson_types::Struct;
use sqlinference::dialect::Dialect;

use crate::databases::{
    base_for_seeds_create_table_specifying_text_type, CacheStatus, DatabaseQueryGenerator,
    SnapshotGenerator, Timestamp, MATERIALIZATION_TYPE_TABLE, MATERIALIZATION_TYPE_VIEW,
};

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorClickhouse {
    schema: String,
    /// override_now is used to override the current timestamp in the generated SQL. It is primarily
    /// used for testing purposes.
    override_now: Option<SystemTime>,
}

impl DatabaseQueryGeneratorClickhouse {
    pub fn new(
        schema: String,
        override_now: Option<SystemTime>,
    ) -> DatabaseQueryGeneratorClickhouse {
        DatabaseQueryGeneratorClickhouse {
            schema,
            override_now,
        }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorClickhouse {
    fn supported_materialization_types(&self) -> &'static [&'static str] {
        &[MATERIALIZATION_TYPE_VIEW, MATERIALIZATION_TYPE_TABLE]
    }

    fn models_drop_query(
        &self,
        object_name: &str,
        materialization_type: &Option<String>,
        _: &CacheStatus,
    ) -> Result<Option<String>, String> {
        let object_name = self.return_full_path_requirement(object_name);
        let object_name = self.database_name_wrapper(&object_name);
        match materialization_type {
            None => Ok(Some(format!("DROP VIEW IF EXISTS {}", object_name))),
            Some(materialization_type) if materialization_type == MATERIALIZATION_TYPE_VIEW => {
                Ok(Some(format!("DROP VIEW IF EXISTS {}", object_name)))
            }
            Some(materialization_type) if materialization_type == MATERIALIZATION_TYPE_TABLE => {
                Ok(Some(format!("DROP TABLE IF EXISTS {}", object_name)))
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
            Some(MATERIALIZATION_TYPE_TABLE) => Ok(Some(vec![format!(
                "CREATE TABLE {} AS {}",
                object_name, original_select_statement
            )])),
            _ => Err("Unsupported materialization type".to_string()),
        }
    }

    fn seeds_drop_table_query(&self, table_name: &str) -> String {
        format!(
            "DROP TABLE IF EXISTS {} CASCADE",
            self.return_full_path_requirement(table_name)
        )
    }

    fn seeds_create_table_query(&self, table_name: &str, columns: &[String]) -> String {
        let table_name = self.return_full_path_requirement(table_name);
        base_for_seeds_create_table_specifying_text_type("TEXT", &table_name, columns)
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        format!("{}.{}", self.schema, table_name)
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

    #[cfg(not(target_arch = "wasm32"))]
    fn get_current_timestamp(&self) -> Timestamp {
        let datetime = self
            .override_now
            .map(|time| -> DateTime<Utc> { time.into() })
            .unwrap_or(SystemTime::now().into())
            .format("%Y-%m-%dT%H:%M:%SZ");
        format!("CAST('{}' AS TIMESTAMP WITH TIME ZONE)", datetime)
    }

    #[cfg(target_arch = "wasm32")]
    fn get_current_timestamp(&self) -> Timestamp {
        let datetime = self
            .override_now
            .map(|time| -> DateTime<Utc> { time.into() })
            .unwrap_or(Date::new_0().into());
        format!(
            "CAST ('{}' AS TIMESTAMP WITH TIME ZONE)",
            datetime.format("%Y-%m-%dT%H:%M:%SZ")
        )
    }
}

impl SnapshotGenerator for DatabaseQueryGeneratorClickhouse {}

#[cfg(test)]
mod test {
    use crate::databases::DatabaseQueryGenerator;

    #[test]
    fn test_get_current_timestamp() {
        let generator = super::DatabaseQueryGeneratorClickhouse::new("schema".to_string(), None);
        let now = generator.get_current_timestamp();

        assert!(now.starts_with("CAST('20"));
    }

    #[test]
    fn get_current_timestamp_override() {
        let generator = super::DatabaseQueryGeneratorClickhouse::new(
            "schema".to_string(),
            Some(std::time::SystemTime::UNIX_EPOCH),
        );
        let now = generator.get_current_timestamp();
        assert_eq!(
            now,
            "CAST('1970-01-01T00:00:00Z' AS TIMESTAMP WITH TIME ZONE)".to_string()
        );
    }

    // TODO MAKE SURE WE HAVE A TEST for sql generation
}
