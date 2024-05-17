use crate::rpc_helpers::{decode, encode};
use crate::rpc_proto_scaffolding::{database_query_generator_from_config, JsFileSystem};
use crate::uint8_reader::Uint8ArrayReader;
use js_sys::{Function, Promise, Uint8Array};
use quary_core::chart::{chart_file_from_yaml, chart_file_to_yaml};
use quary_core::database_snowflake::validate_snowfalke_account_identifier;
use quary_core::test_runner::{
    run_model_tests_internal, run_tests_internal, RunReturnResult, RunStatementFunc, RunTestError,
};
use quary_proto::TestRunner;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// write chart file to Uint8Array string encodes a chart file in Uint8Array proto and returns it
/// as yaml string.
#[wasm_bindgen]
pub fn write_chart_file(chart_file: Uint8Array) -> Result<Uint8Array, String> {
    let chart_file = decode::<quary_proto::ChartFile>(chart_file)?;
    let yaml = chart_file_to_yaml(&chart_file)?;
    Ok(Uint8Array::from(yaml.as_bytes()))
}

/// string to chart file parses a chart file in Uint8Array string and returns a Uint8Array chart
/// file encoded in proto.
#[wasm_bindgen]
pub fn parse_chart_file(file: Uint8Array) -> Result<Uint8Array, String> {
    let reader = Uint8ArrayReader::new(file);
    let chart_file = chart_file_from_yaml(reader)?;

    encode(chart_file)
}

/// add_limit_to_select returns a select statement where a LIMIT clause has been added to it.
#[wasm_bindgen]
pub fn add_limit_to_select(sql: &str, limit: usize) -> String {
    let lower_cased = sql.trim().to_lowercase();
    if lower_cased.starts_with("create") {
        return sql.to_string();
    }
    format!("{} LIMIT {};", sql.trim_end_matches(';'), limit)
}

/// Returns a string without any newlines or tabs and with only one space between words.
#[wasm_bindgen]
pub fn clean_up(value: &str) -> String {
    value
        .replace('\n', " ")
        .replace(char::is_whitespace, " ")
        .trim()
        .to_string()
}

#[wasm_bindgen]
pub async fn run_tests(
    test_runner: &str,
    database: Uint8Array,
    file_reader: Function,
    file_lister: Function,
    run_statement: Function,
    project_root: &str,
) -> Result<Uint8Array, String> {
    let file_system = JsFileSystem::new(file_reader, file_lister);
    let database = database_query_generator_from_config(database)?;
    let project = quary_core::project::parse_project(&file_system, &database, project_root).await?;

    let test_runner = match test_runner {
        "skip" => TestRunner::Skip,
        "all" => TestRunner::All,
        _ => panic!("Invalid test runner"),
    };

    let function = Rc::new(run_statement);
    let func = create_run_statement_func(function);

    let test_results = run_tests_internal(
        &database,
        &file_system,
        &project,
        project_root,
        database.get_dialect(),
        test_runner,
        func,
        false,
        Some(1),
    )
    .await
    .map_err(|err| match err {
        RunTestError::TestFailedToRun(test) => {
            format!(
                "Test '{}' failed to run: {}, ran sql '{}'",
                test.test_name, test.error, test.sql
            )
        }
        RunTestError::Other(s) => s,
    })?;

    encode(test_results)
}

#[wasm_bindgen]
pub async fn run_model_tests(
    database: Uint8Array,
    file_reader: Function,
    file_lister: Function,
    run_statement: Function,
    model_name: &str,
    project_root: &str,
    whether_to_include_model_to_source: bool,
) -> Result<Uint8Array, String> {
    let file_system = JsFileSystem::new(file_reader, file_lister);
    let database = database_query_generator_from_config(database)?;
    let project = quary_core::project::parse_project(&file_system, &database, project_root).await?;

    let function = Rc::new(run_statement);
    let func = create_run_statement_func(function);

    let test_results = run_model_tests_internal(
        &database,
        &file_system,
        &project,
        func,
        whether_to_include_model_to_source,
        Some(1),
        model_name,
    )
    .await
    .map_err(|err| match err {
        RunTestError::TestFailedToRun(test) => {
            format!(
                "Test '{}' failed to run: {}, ran sql '{}'",
                test.test_name, test.error, test.sql
            )
        }
        RunTestError::Other(s) => s,
    })?;

    encode(test_results)
}

fn create_run_statement_func(function: Rc<Function>) -> RunStatementFunc {
    let func: RunStatementFunc = Box::new(move |sql: &str| {
        let sql = sql.to_owned(); // Convert &str to String
        let function = function.clone(); // Clone the Rc<Function>
        Box::pin(async move {
            let result = function.call1(&JsValue::NULL, &JsValue::from_str(sql.as_str()));
            match result {
                Ok(js_value) => {
                    let promise: Result<Promise, _> = js_value.dyn_into();
                    match promise {
                        Ok(promise) => {
                            let js_future = JsFuture::from(promise);
                            let js_value = js_future.await.map_err(|err| {
                                format!("Failed to await js function to : {:?}", err)
                            })?;

                            let array: js_sys::Array = js_value.try_into().map_err(|err| {
                                format!("Failed to map js value to array: {:?}", err)
                            })?;
                            let length = array.length();
                            if length != 2 {
                                return Err(format!(
                                    "Expected array of length 2, got length {}",
                                    length
                                ));
                            }

                            let js_value = array.get(0);
                            let js_string: String = js_value.try_into().map_err(|err| {
                                format!("Failed to map js value to string: {:?}", err)
                            })?;

                            match js_string.as_str() {
                                "success_call" => {
                                    let js_value = array.get(1);
                                    let uint_8_array: Uint8Array =
                                        js_value.try_into().map_err(|err| {
                                            format!(
                                                "Failed to map js value to Uint8Array: {:?}",
                                                err
                                            )
                                        })?;
                                    let result: quary_proto::QueryResult = decode(uint_8_array)?;
                                    match result.columns.get(0) {
                                        Some(column) => {
                                            if column.values.is_empty() {
                                                Ok(RunReturnResult::Passed)
                                            } else {
                                                Ok(RunReturnResult::QueryResult(result))
                                            }
                                        }
                                        None => Ok(RunReturnResult::Passed),
                                    }
                                }
                                "error_call" => {
                                    let js_value = array.get(1);
                                    let js_string: String = js_value.try_into().map_err(|err| {
                                        format!("Failed to map js value to string: {:?}", err)
                                    })?;
                                    Ok(RunReturnResult::Error(js_string))
                                }
                                _ => Err(format!(
                                    "Failed to map js value to string: {:?}",
                                    js_string
                                )),
                            }
                        }
                        Err(err) => Err(format!("Failed to map promise: {:?}", err)),
                    }
                }
                Err(err) => Err(format!("Failed to call function: {:?}", err)),
            }
        })
    });
    func
}

#[wasm_bindgen]
pub fn validate_snowflake_account_identifier(account_identifier: &str) -> bool {
    validate_snowfalke_account_identifier(account_identifier).is_ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_limit_to_select_create_statement() {
        let sql = "CREATE TABLE table (id int)";
        let expected = "CREATE TABLE table (id int)";
        assert_eq!(super::add_limit_to_select(sql, 1000), expected);
    }

    #[test]
    fn test_add_limit_to_select_select_statement() {
        let sql = "select * from table";
        let expected = "select * from table LIMIT 1000;";
        assert_eq!(super::add_limit_to_select(sql, 1000), expected);
    }

    #[test]
    fn test_add_limit_to_select_select_statement_with_semi_colon() {
        let sql = "select * from table;";
        let expected = "select * from table LIMIT 1000;";
        assert_eq!(super::add_limit_to_select(sql, 1000), expected);
    }
}
