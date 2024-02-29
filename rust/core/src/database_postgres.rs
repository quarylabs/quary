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
        format!("{}", name)
    }
}
