use crate::databases::{
    base_for_seeds_create_table_specifying_text_type, DatabaseQueryGenerator, SnapshotGenerator,
    Timestamp,
};
use chrono::{DateTime, Utc};
#[cfg(target_arch = "wasm32")]
use js_sys::Date;
use quary_proto::snapshot::snapshot_strategy::StrategyType;
use sqlinference::dialect::Dialect;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorRedshift {
    schema: String,
    /// override_now is used to override the current timestamp in the generated SQL. It is primarily
    /// used for testing purposes.
    override_now: Option<SystemTime>,
}

impl DatabaseQueryGeneratorRedshift {
    pub fn new(schema: String, override_now: Option<SystemTime>) -> DatabaseQueryGeneratorRedshift {
        DatabaseQueryGeneratorRedshift {
            schema,
            override_now,
        }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorRedshift {
    fn validate_materialization_type(
        &self,
        materialization_type: &Option<String>,
    ) -> Result<(), String> {
        match materialization_type {
            None => Ok(()),
            Some(materialization_type) if materialization_type == "view" => Ok(()),
            Some(materialization_type) if materialization_type == "table" => Ok(()),
            Some(materialization_type) if materialization_type == "materialized_view" => Ok(()),
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
                object_name, original_select_statement
            )),
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

impl SnapshotGenerator for DatabaseQueryGeneratorRedshift {
    fn generate_snapshot_sql(
        &self,
        path: &str,
        templated_select: &str,
        unique_key: &str,
        strategy: &StrategyType,
        table_exists: Option<bool>,
    ) -> Result<Vec<String>, String> {
        match strategy {
            StrategyType::Timestamp(timestamp) => {
                let updated_at = &timestamp.updated_at;
                let now = self.get_current_timestamp();

                // Redshift does not support CREATE TABLE IF NOT EXISTS (w/ AS (...))
                let create_table_sql = format!(
                    "CREATE TABLE {path} AS (
                        SELECT
                            ts.*,
                            {now} AS quary_valid_from,
                            CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,
                            MD5(CAST(CONCAT({unique_key}, CAST({updated_at} AS TEXT)) AS TEXT)) AS quary_scd_id
                        FROM ({templated_select}) AS ts
                    )"
                );

                let update_sql = format!(
                    "UPDATE {path} AS target
                    SET quary_valid_to = source.quary_valid_from
                    FROM (
                        SELECT
                            ts.*,
                            {now} AS quary_valid_from,
                            MD5(CAST(CONCAT({unique_key}, CAST({updated_at} AS TEXT)) AS TEXT)) AS quary_scd_id
                        FROM ({templated_select}) AS ts
                    ) AS source
                    WHERE target.{unique_key} = source.{unique_key}
                        AND target.quary_valid_to IS NULL
                        AND CAST(source.{updated_at} AS TIMESTAMP) > CAST(target.{updated_at} AS TIMESTAMP)"
                );

                let insert_sql = format!(
                    "INSERT INTO {path}
                    SELECT
                        *,
                        {now} AS quary_valid_from,
                        CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,
                        MD5(CAST(CONCAT({unique_key}, CAST({updated_at} AS TEXT)) AS TEXT)) AS quary_scd_id
                    FROM ({templated_select}) AS source
                    WHERE NOT EXISTS (
                        SELECT 1
                        FROM {path} AS target
                        WHERE target.quary_scd_id = MD5(CAST(CONCAT(source.{unique_key}, CAST(source.{updated_at} AS TEXT)) AS TEXT))
                    )"
                );

                let mut sql_statements = vec![update_sql, insert_sql];

                match table_exists {
                    Some(exists) => {
                        if !exists {
                            sql_statements.insert(0, create_table_sql);
                        }
                        Ok(sql_statements)
                    }
                    None => Err("table_exists is required for Redshift snapshots".to_string()),
                }
            }
        }
    }

    fn generate_snapshot_query(
        &self,
        templated_select: &str,
        unique_key: &str,
        strategy: &StrategyType,
        now: &str,
    ) -> Result<String, String> {
        match strategy {
            StrategyType::Timestamp(timestamp) => {
                let updated_at = &timestamp.updated_at;
                Ok(format!(
                    "SELECT
                        ts.*,
                        {now} AS quary_valid_from,
                        CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,
                        MD5(CAST(CONCAT({unique_key}, CAST({updated_at} AS TEXT)) AS TEXT)) AS quary_scd_id
                    FROM ({templated_select}) AS ts"
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::databases::DatabaseQueryGenerator;

    #[test]
    fn test_get_current_timestamp() {
        let generator = super::DatabaseQueryGeneratorRedshift::new("schema".to_string(), None);
        let now = generator.get_current_timestamp();

        assert!(now.starts_with("CAST('20"));
    }

    #[test]
    fn get_current_timestamp_override() {
        let generator = super::DatabaseQueryGeneratorRedshift::new(
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
