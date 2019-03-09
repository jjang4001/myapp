use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
        let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            log(&"received message".to_string());
        }) as Box<dyn FnMut(_)>);

        // let a = Some(&closure);

        socket.set_onmessage(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
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
