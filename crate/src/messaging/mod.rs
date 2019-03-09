use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent};

use super::models;

extern crate js_sys;
pub mod messenger;

#[wasm_bindgen(module = "../src/messaging/messaging")]
extern "C" {
    pub fn name() -> String;
    pub type Messenger;

    #[wasm_bindgen(constructor)]
    pub fn new(ws_address: &str) -> Messenger;

    #[wasm_bindgen(method)]
    pub fn sendMessage(this: &Messenger, msg: &str);
    #[wasm_bindgen(method)]
    fn onReceiveMessage(this: &Messenger, evt: MessageEvent);
    #[wasm_bindgen(method)]
    fn onCloseConnection(this: &Messenger);
}
