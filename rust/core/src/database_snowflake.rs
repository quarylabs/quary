use crate::databases::{base_for_seeds_create_table_specifying_text_type, DatabaseQueryGenerator};
use sqlinference::dialect::Dialect;

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorSnowflake {
    database: String,
    schema: String,
}

impl DatabaseQueryGeneratorSnowflake {
    pub fn new(database: String, schema: String) -> DatabaseQueryGeneratorSnowflake {
        DatabaseQueryGeneratorSnowflake { database, schema }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorSnowflake {
    fn seeds_create_table_query(&self, table_name: &str, columns: &[String]) -> String {
        let table_name = self.return_full_path_requirement(table_name);
        base_for_seeds_create_table_specifying_text_type("STRING", table_name.as_str(), columns)
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        format!("{}.{}.{}", self.database, self.schema, table_name)
    }

    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String> {
        let split = full_path.split('.').collect::<Vec<&str>>();
        match &split[..] {
            [project_id, dataset_id, table_name] => {
                if project_id == &self.database && dataset_id == &self.schema {
                    Ok(table_name)
                } else {
                    Err(format!(
                        "Project ID {} or dataset ID {} does not match {} expected format: project_id.dataset_id.table_name",
                        project_id, dataset_id, full_path
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
        &Dialect::Snowflake
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        name.to_string()
    }
}

/// validate_snowfalke_account_identifier validates a Snowflake account identifier.
pub fn validate_snowfalke_account_identifier(account_identifier: &str) -> Result<(), String> {
    #[allow(clippy::unwrap_used)]
    let regex = regex::Regex::new(r"^[.a-zA-Z0-9_-]+$").unwrap();

    if account_identifier.contains("https://")
        || account_identifier.ends_with("snowflakecomputing.com")
        || !regex.is_match(account_identifier)
    {
        Err(format!(
            "Account identifier {} is invalid, it should be only the identifier from <account_identifier>.snowflakecomputing.com ",
            account_identifier,
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automatic_cache_sql_create_statement() {
        let database = DatabaseQueryGeneratorSnowflake::new(
            "project_id".to_string(),
            "dataset_id".to_string(),
        );

        let model = "shifts_summary";
        let model_cache_name = "qqq_shifts_summary_fbas143";
        let sql = database.automatic_cache_sql_create_statement(model, model_cache_name);

        assert_eq!(sql, vec!["CREATE OR REPLACE VIEW project_id.dataset_id.qqq_shifts_summary_fbas143 AS SELECT * FROM project_id.dataset_id.shifts_summary"]);
    }

    #[test]
    fn test_return_table_view_from_full_path() {
        let database = DatabaseQueryGeneratorSnowflake::new(
            "project_id".to_string(),
            "dataset_id".to_string(),
        );

        let query = database.return_name_from_full_path("project_id.dataset_id.table_name");
        assert_eq!(query, Ok("table_name"));

        let query = database.return_name_from_full_path("schema.table_name");
        assert_eq!(
            query,
            Err(
                "Table name schema.table_name does not contain project ID and dataset ID"
                    .to_string()
            )
        );
    }

    #[test]
    fn valid_snowflake_account_identifiers() {
        let valid_identifiers = vec![
            "xy12345",
            "xy12345.us-gov-west-1.aws",
            "xy12345.fhplus.us-gov-west-1.aws",
            "xy12345.us-east-2.aws",
            "xy12345.us-east-1",
            "xy12345.us-east-1-gov.aws",
            "xy12345.ca-central-1.aws",
            "xy12345.sa-east-1.aws",
            "xy12345.eu-west-1",
            "xy12345.eu-west-2.aws",
            "xy12345.eu-west-3.aws",
            "xy12345.eu-central-1",
            "xy12345.eu-north-1.aws",
            "xy12345.ap-northeast-1.aws",
            "xy12345.ap-northeast-3.aws",
            "xy12345.ap-northeast-2.aws",
            "xy12345.ap-south-1.aws",
            "xy12345.ap-southeast-1",
            "xy12345.ap-southeast-2",
            "xy12345.ap-southeast-3.aws",
            "xy12345.us-central1.gcp",
            "xy12345.us-east4.gcp",
            "xy12345.europe-west2.gcp",
            "xy12345.europe-west4.gcp",
            "xy12345.west-us-2.azure",
            "xy12345.central-us.azure",
            "xy12345.south-central-us.azure",
            "xy12345.east-us-2.azure",
            "xy12345.us-gov-virginia.azure",
            "xy12345.canada-central.azure",
            "xy12345.uk-south.azure",
            "xy12345.north-europe.azure",
            "xy12345.west-europe.azure",
            "xy12345.switzerland-north.azure",
            "xy12345.uae-north.azure",
            "xy12345.central-india.azure",
            "xy12345.japan-east.azure",
            "xy12345.southeast-asia.azure",
            "xy12345.australia-east.azure",
            "acme-marketing_test_account",
            "acme-marketing-test-account",
            "test.us-east-2.aws",
            "acme-test_aws_us_east_2",
            "test.west-us-2.azure",
            "acme-test_azure_west_us_2",
        ];
        for identifier in valid_identifiers {
            assert_eq!(validate_snowfalke_account_identifier(identifier), Ok(()));
        }

        let invalid_identifiers = vec![
            "xy12345.snowflakecomputing.com",
            "xy12345.snowflakecomputing.com:443",
            "https://xy12345.snowflakecomputing.com",
        ];
        for identifier in invalid_identifiers {
            assert!(validate_snowfalke_account_identifier(identifier).is_err());
        }
    }
}
