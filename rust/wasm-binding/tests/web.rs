//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use sqlinference::columns::get_columns_internal;
use sqruff_lib_core::parser::Parser;
use sqruff_lib_dialects::ansi;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn get_columns_internal_test() {
    let input = "SELECT a, b, 123, myfunc(b)
FROM table_1
WHERE a > b AND b < 100
ORDER BY a DESC, b";
    let dialect = ansi::dialect(None);
    let parser = Parser::from(&dialect);
    let output = get_columns_internal(&parser, input);
    assert_eq!(
        output,
        Ok((
            vec!["a".to_string(), "b".to_string()],
            vec!["123".to_string(), "myfunc(b)".to_string()],
        ))
    );
}
