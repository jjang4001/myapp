use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde_json::Result;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;

use super::models;

pub struct Messenger {
    ws: web_sys::WebSocket,
    context: std::rc::Rc<web_sys::CanvasRenderingContext2d>,
}

impl Messenger {
    pub fn send_message(&self, msg: &str) {
        self.ws.send_with_str(msg).unwrap();
    }

    fn _on_close_connection(&self) {

    }

    pub fn new(ws_address: &str, context: std::rc::Rc<web_sys::CanvasRenderingContext2d>) -> Messenger {
        let socket = web_sys::WebSocket::new(ws_address).unwrap();

        let response_context = context.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            // get previous context
            let prev_stroke_style = response_context.stroke_style().as_string().unwrap();
            let prev_alpha = response_context.global_alpha();
            let prev_size = response_context.line_width();

            let stroke_response = &event.data().as_string().unwrap();
            let stroke: models::stroke::Stroke = serde_json::from_str(stroke_response).unwrap();

            match stroke.endpoint.as_str() {
                "START" => {
                    // set context for received stroke
                    response_context.set_stroke_style(&JsValue::from_str(&stroke.rgb));
                    response_context.set_global_alpha(stroke.alpha);
                    response_context.set_line_width(stroke.size);

                    response_context.begin_path();
                    response_context.move_to(stroke.x, stroke.y);
                },
                "MOVE" => {
                    // set context for received stroke
                    response_context.set_stroke_style(&JsValue::from_str(&stroke.rgb));
                    response_context.set_global_alpha(stroke.alpha);
                    response_context.set_line_width(stroke.size);

                    response_context.line_to(stroke.x, stroke.y);
                    response_context.stroke();
                    response_context.begin_path();
                    response_context.move_to(stroke.x, stroke.y);
                },
                "END" => {
                    // set context for received stroke
                    response_context.set_stroke_style(&JsValue::from_str(&stroke.rgb));
                    response_context.set_global_alpha(stroke.alpha);
                    response_context.set_line_width(stroke.size);

                    response_context.line_to(stroke.x, stroke.y);
                    response_context.stroke();
                },
                _ => log(&stroke.endpoint),
            }

            // reset to current context
            response_context.set_stroke_style(&JsValue::from_str(&prev_stroke_style));
            response_context.set_global_alpha(prev_alpha);
            response_context.set_line_width(prev_size);
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
