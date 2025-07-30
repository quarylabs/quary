use crate::databases::{
    base_for_seeds_create_table_specifying_text_type, DatabaseQueryGenerator, SnapshotGenerator,
};
use sqruff_lib_core::dialects::Dialect;
use sqruff_lib_dialects::bigquery;

#[derive(Debug, Clone)]
pub struct DatabaseQueryGeneratorBigQuery {
    project_id: String,
    dataset_id: String,
}

impl DatabaseQueryGeneratorBigQuery {
    pub fn new(project_id: String, dataset_id: String) -> DatabaseQueryGeneratorBigQuery {
        DatabaseQueryGeneratorBigQuery {
            project_id,
            dataset_id,
        }
    }
}

impl DatabaseQueryGenerator for DatabaseQueryGeneratorBigQuery {
    fn get_name(&self) -> &'static str {
        "bigquery"
    }

    fn seeds_create_table_query(&self, table_name: &str, columns: &[String]) -> String {
        let table_name = self.return_full_path_requirement(table_name);
        base_for_seeds_create_table_specifying_text_type("STRING", table_name.as_str(), columns)
    }

    fn return_full_path_requirement(&self, table_name: &str) -> String {
        format!("{}.{}.{}", self.project_id, self.dataset_id, table_name)
    }

    fn return_name_from_full_path<'a>(&self, full_path: &'a str) -> Result<&'a str, String> {
        let split = full_path.split('.').collect::<Vec<&str>>();
        match &split[..] {
            [project_id, dataset_id, table_name] => {
                if project_id == &self.project_id && dataset_id == &self.dataset_id {
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

    fn get_dialect(&self) -> Dialect {
        bigquery::dialect()
    }

    fn database_name_wrapper(&self, name: &str) -> String {
        format!("`{}`", name)
    }
}

impl SnapshotGenerator for DatabaseQueryGeneratorBigQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automatic_cache_sql_create_statement() {
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_id".to_string(), "dataset_id".to_string());
        let model = "shifts_summary";
        let model_cache_name = "qqq_shifts_summary_fbas143";
        let sql = database.automatic_cache_sql_create_statement(model, model_cache_name);

        assert_eq!(sql, vec!["CREATE OR REPLACE VIEW project_id.dataset_id.qqq_shifts_summary_fbas143 AS SELECT * FROM project_id.dataset_id.shifts_summary"]);
    }

    #[test]
    fn test_return_table_view_from_full_path() {
        let database =
            DatabaseQueryGeneratorBigQuery::new("project_id".to_string(), "dataset_id".to_string());
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
}
