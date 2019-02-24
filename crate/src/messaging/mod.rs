use wasm_bindgen::prelude::*;

extern crate js_sys;

use super::examples;

#[wasm_bindgen]
pub fn test(console_message: &str) {
    let a = examples::BindgenExamples::new(0);
    a.console_log(console_message.to_string());
}
