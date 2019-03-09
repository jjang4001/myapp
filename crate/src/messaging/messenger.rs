use wasm_bindgen::prelude::*;

pub struct Messenger {
    ws: web_sys::WebSocket,
    context: std::rc::Rc<web_sys::CanvasRenderingContext2d>,
}

impl Messenger {
    pub fn send_message(&self, msg: &str) {
        log(&"sending message from rust messenger".to_string());
        self.ws.send_with_str(msg).unwrap();
    }

    fn _on_receive_message(&self, _evt: web_sys::MessageEvent) {

    }

    fn _on_close_connection(&self) {

    }

    pub fn new(ws_address: &str, context: std::rc::Rc<web_sys::CanvasRenderingContext2d>) -> Messenger {
        let socket = web_sys::WebSocket::new(ws_address).unwrap();
        // let context = context.clone();
        Messenger {
            ws: socket,
            context,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
