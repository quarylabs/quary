use crate::databases::{
    base_for_seeds_create_table_specifying_text_type, DatabaseQueryGenerator, SnapshotGenerator,
    Timestamp,
};
use chrono::{DateTime, Utc};
#[cfg(target_arch = "wasm32")]
use js_sys::Date;
use quary_proto::snapshot::snapshot_strategy::StrategyType;

use sqruff_lib_core::dialects::base::Dialect;
use sqruff_lib_dialects::duckdb;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorDuckDB {
    schema: Option<String>,
    /// override_now is used to override the current timestamp in the generated SQL. It is primarily
    /// used for testing purposes.
    override_now: Option<SystemTime>,
}

impl DatabaseQueryGeneratorDuckDB {
    pub fn new(
        schema: Option<String>,
        override_now: Option<SystemTime>,
    ) -> DatabaseQueryGeneratorDuckDB {
        DatabaseQueryGeneratorDuckDB {
            schema,
            override_now,
        }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorDuckDB {
    fn get_name(&self) -> &'static str {
        "duckdb"
    }

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

    fn get_dialect(&self) -> Dialect {
        duckdb::dialect()
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        name.into()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_current_timestamp(&self) -> Timestamp {
        let datetime = self
            .override_now
            .map(|time| -> DateTime<Utc> { time.into() })
            .unwrap_or(SystemTime::now().into());
        format!(
            "CAST ('{}' AS TIMESTAMP WITH TIME ZONE)",
            datetime.format("%Y-%m-%dT%H:%M:%SZ")
        )
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

impl SnapshotGenerator for DatabaseQueryGeneratorDuckDB {
    fn generate_snapshot_sql(
        &self,
        path: &str,
        templated_select: &str,
        unique_key: &str,
        strategy: &StrategyType,
        table_exists: Option<bool>,
    ) -> Result<Vec<String>, String> {
        assert_eq!(
            table_exists, None,
            "table_exists is not necessary for DuckDB snapshots."
        );

        let now = self.get_current_timestamp();
        let snapshot_query =
            self.generate_snapshot_query(templated_select, unique_key, strategy, now.as_str())?;

        match strategy {
            StrategyType::Timestamp(timestamp) => {
                let updated_at = &timestamp.updated_at;
                let create_table_sql = format!(
                    "CREATE TABLE IF NOT EXISTS {path} AS (
                      {snapshot_query}
                    )"
                );

                let update_sql = format!(
                    "UPDATE {path} AS target
                    SET quary_valid_to = source.quary_valid_from
                    FROM (
                        SELECT
                            *,
                            {now} AS quary_valid_from,
                            MD5(CAST(CONCAT({unique_key}, CAST({updated_at} AS STRING)) AS STRING)) AS quary_scd_id
                        FROM ({templated_select})
                    ) AS source
                    WHERE target.{unique_key} = source.{unique_key}
                        AND target.quary_valid_to IS NULL
                        AND CAST(source.{updated_at} AS TIMESTAMP WITH TIME ZONE) > CAST(target.{updated_at} AS TIMESTAMP WITH TIME ZONE)"
                );

                let insert_sql = format!(
                    "INSERT INTO {path}
                    SELECT
                        *,
                        {now} AS quary_valid_from,
                        CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,
                        MD5(CAST(CONCAT({unique_key}, CAST({updated_at} AS STRING)) AS STRING)) AS quary_scd_id
                    FROM ({templated_select}) AS source
                    WHERE NOT EXISTS (
                        SELECT 1
                        FROM {path} AS target
                        WHERE target.quary_scd_id = MD5(CAST(CONCAT(source.{unique_key}, CAST(source.{updated_at} AS STRING)) AS STRING))
                    )"
                );

                Ok(vec![create_table_sql, update_sql, insert_sql])
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
                        *,
                        {now} AS quary_valid_from,
                        CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,
                        MD5(CAST(CONCAT({unique_key}, CAST({updated_at} AS STRING)) AS STRING)) AS quary_scd_id
                    FROM ({templated_select})"
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_full_path_requirement() {
        let database = DatabaseQueryGeneratorDuckDB::new(None, None);
        assert_eq!(
            database.return_full_path_requirement("table_name"),
            "table_name"
        );

        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()), None);
        assert_eq!(
            database.return_full_path_requirement("table_name"),
            "schema.table_name"
        );
    }

    #[test]
    fn test_return_name_from_full_path() {
        let database = DatabaseQueryGeneratorDuckDB::new(None, None);
        let query = database.return_name_from_full_path("table_name");
        assert_eq!(query, Ok("table_name"));

        let database = DatabaseQueryGeneratorDuckDB::new(Some("schema".to_string()), None);
        let query = database.return_name_from_full_path("schema.table_name");
        assert_eq!(query, Ok("table_name"));

        let query = database.return_name_from_full_path("error.table_name");
        assert!(query.is_err());
    }

    #[test]
    fn test_get_current_timestamp() {
        let override_now = SystemTime::now();
        let database = DatabaseQueryGeneratorDuckDB::new(None, Some(override_now));

        // TODO Improve test
        let result = database.get_current_timestamp();
        let expected_datetime: DateTime<Utc> = override_now.into();
        let expected_result = format!(
            "CAST ('{}' AS TIMESTAMP WITH TIME ZONE)",
            expected_datetime.format("%Y-%m-%dT%H:%M:%SZ")
        );
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_generate_snapshot_sql() {
        let time_override = "2021-01-01T00:00:00Z";
        let override_now = DateTime::parse_from_rfc3339(time_override)
            .unwrap()
            .with_timezone(&Utc);
        let system_time = SystemTime::from(override_now);

        let database = DatabaseQueryGeneratorDuckDB::new(None, Some(system_time));
        let path = "mytable";
        let templated_select = "SELECT * FROM mytable";
        let unique_key = "id";
        let updated_at = "updated_at";
        let strategy = StrategyType::Timestamp(
            quary_proto::snapshot::snapshot_strategy::TimestampStrategy {
                updated_at: updated_at.to_string(),
            },
        );

        let result = database
            .generate_snapshot_sql(path, templated_select, unique_key, &strategy, None)
            .unwrap();

        assert_eq!(
            vec!["CREATE TABLE IF NOT EXISTS mytable AS (\n                      SELECT\n                        *,\n                        CAST ('2021-01-01T00:00:00Z' AS TIMESTAMP WITH TIME ZONE) AS quary_valid_from,\n                        CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,\n                        MD5(CAST(CONCAT(id, CAST(updated_at AS STRING)) AS STRING)) AS quary_scd_id\n                    FROM (SELECT * FROM mytable)\n                    )", "UPDATE mytable AS target\n                    SET quary_valid_to = source.quary_valid_from\n                    FROM (\n                        SELECT\n                            *,\n                            CAST ('2021-01-01T00:00:00Z' AS TIMESTAMP WITH TIME ZONE) AS quary_valid_from,\n                            MD5(CAST(CONCAT(id, CAST(updated_at AS STRING)) AS STRING)) AS quary_scd_id\n                        FROM (SELECT * FROM mytable)\n                    ) AS source\n                    WHERE target.id = source.id\n                        AND target.quary_valid_to IS NULL\n                        AND CAST(source.updated_at AS TIMESTAMP WITH TIME ZONE) > CAST(target.updated_at AS TIMESTAMP WITH TIME ZONE)", "INSERT INTO mytable\n                    SELECT\n                        *,\n                        CAST ('2021-01-01T00:00:00Z' AS TIMESTAMP WITH TIME ZONE) AS quary_valid_from,\n                        CAST(NULL AS TIMESTAMP WITH TIME ZONE) AS quary_valid_to,\n                        MD5(CAST(CONCAT(id, CAST(updated_at AS STRING)) AS STRING)) AS quary_scd_id\n                    FROM (SELECT * FROM mytable) AS source\n                    WHERE NOT EXISTS (\n                        SELECT 1\n                        FROM mytable AS target\n                        WHERE target.quary_scd_id = MD5(CAST(CONCAT(source.id, CAST(source.updated_at AS STRING)) AS STRING))\n                    )"],
            result.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
        );
    }
}
