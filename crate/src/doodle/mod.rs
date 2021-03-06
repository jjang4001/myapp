use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::messaging;
use super::models;

mod canvas;
mod menu;

#[wasm_bindgen]
pub fn start_doodle(ws_address: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .expect("document should have #canvas on DOM")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("#canvas should be a HtmlCanvasElement");

    canvas.set_width(3840);
    canvas.set_height(2160);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    handle_context_events(document, context, canvas, ws_address)
}

fn handle_context_events(document: web_sys::Document, context: web_sys::CanvasRenderingContext2d, canvas: web_sys::HtmlCanvasElement, ws_address: &str) -> Result<(), JsValue> {

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    let messenger_context = context.clone();
    let m = messaging::messenger::Messenger::new(ws_address, messenger_context);
    let ms = Rc::new(m);

    context.set_stroke_style(&JsValue::from_str("#000000"));
    context.set_global_alpha(1.0);
    context.set_line_width(3.0);

    canvas::handle_mousedown_event(&context, &pressed, &canvas, &ms);
    canvas::handle_mousemove_event(&context, &pressed, &canvas, &ms);
    canvas::handle_mouseup_event(&context, &pressed, &canvas, &ms);

    menu::handle_doodle_size_input_event(&context, &document);
    menu::handle_color_event(&context, &document);
    menu::handle_color_picker_input_event(&context, &document);

    Ok(())
}
