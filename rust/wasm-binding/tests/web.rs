//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use quary_wasm_bindgen::get_columns_internal;
use quary_wasm_bindgen::to_array;
use quary_wasm_bindgen::GetColumnsResultInternal;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_columns_internal_test() {
        let input = "SELECT a, b, 123, myfunc(b)
FROM table_1
WHERE a > b AND b < 100
ORDER BY a DESC, b";
        let output = get_columns_internal(input.to_string());
        assert_eq!(
            output,
            Ok(GetColumnsResultInternal {
                columns: to_array(&["a", "b"]),
                should_be_aliased: to_array(&["123", "myfunc(b)"]),
            })
        );
    }
}
