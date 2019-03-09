use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::Result;
use serde::{Deserialize, Serialize};

use super::models;

pub struct Messenger {
    ws: web_sys::WebSocket,
    context: std::rc::Rc<web_sys::CanvasRenderingContext2d>,
}

impl Messenger {
    pub fn send_message(&self, msg: &str) {
        self.ws.send_with_str(msg).unwrap();
    }

    // fn on_receive_message(&self, _evt: web_sys::MessageEvent) {
    //     log(&"received message".to_string());

    // }

    fn _on_close_connection(&self) {

    }

    pub fn new(ws_address: &str, context: std::rc::Rc<web_sys::CanvasRenderingContext2d>) -> Messenger {
        let socket = web_sys::WebSocket::new(ws_address).unwrap();

        // let context2 = context.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            log(&event.data().as_string().unwrap());
            // use received message to draw
            let b = &event.data().as_string().unwrap();

            let stroke: models::stroke::Stroke = serde_json::from_str(b).unwrap();
            log(&stroke.rgb);
        }) as Box<dyn FnMut(_)>);

        socket.set_onmessage(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        Messenger {
            ws: socket,
            context,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    status: String,
    content: String,
    num: f64,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
