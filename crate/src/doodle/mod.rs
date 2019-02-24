use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent};

use super::examples;

#[wasm_bindgen]
pub fn start_doodle(ws_address: &str) -> Result<(), JsValue> {
    // log(&format!("bye-- {}", name()));
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    handle_context_events(context, canvas, ws_address)
}

fn handle_context_events(context: web_sys::CanvasRenderingContext2d, canvas: web_sys::HtmlCanvasElement, ws_address: &str) -> Result<(), JsValue> {
    let a = examples::BindgenExamples::new(0);
    a.console_log("bye".to_string());

    // color
    context.set_stroke_style(&JsValue::from_str("rgb(255,0,0,0.5)"));
    // thickness
    context.set_line_width(10.0);
    // 
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {

                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}

#[wasm_bindgen(module = "./messaging")]
extern "C" {
    fn name() -> String;
    // type Messenger;

    // #[wasm_bindgen(constructor)]
    // fn new(ws_address: &str) -> Messenger;

    // #[wasm_bindgen(method)]
    // fn sendMessage(this: &Messenger, msg: &str);
    // #[wasm_bindgen(method)]
    // fn onReceiveMessage(this: &Messenger, evt: MessageEvent);
    // #[wasm_bindgen(method)]
    // fn onCloseConnection(this: &Messenger);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
