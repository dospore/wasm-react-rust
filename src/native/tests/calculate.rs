//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

// extern crate wasm_bindgen_test;
// use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use super::*;

    // #[wasm_bindgen_test]
    #[test]
    fn empty_calculate() {
        assert_eq!(Err(false), calculate(""));
    }

    #[wasm_bindgen_test]
    fn simple_calc() {
        assert_eq!(Ok("4"), calculate("2 + 2"));
    }
}
