use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::messaging;

mod canvas;
mod menu;

#[wasm_bindgen]
pub fn start_doodle(ws_address: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_attribute("id", "myapp-canvas")?;

    let first = document.body().unwrap().child_nodes().get(0).unwrap();
    document
        .body()
        .unwrap()
        .insert_before(&canvas, Some(&first))?;

    let mut width = window.inner_width().unwrap().as_f64().unwrap();
    let mut height = window.inner_height().unwrap().as_f64().unwrap();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    canvas.style().set_property("border", "solid")?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    handle_context_events(window, document, context, canvas, &mut width, &mut height, ws_address)
}

fn handle_context_events(window: web_sys::Window, document: web_sys::Document, context: web_sys::CanvasRenderingContext2d, canvas: web_sys::HtmlCanvasElement, width: &mut f64, height: &mut f64, ws_address: &str) -> Result<(), JsValue> {

    let m = messaging::Messenger::new(ws_address);
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    context.set_stroke_style(&JsValue::from_str("#ff0000"));
    context.set_global_alpha(1.0);
    context.set_line_width(3.0);

    canvas::handle_mousedown_event(&context, &pressed, &canvas);
    canvas::handle_mousemove_event(&context, &pressed, &canvas, m);
    canvas::handle_mouseup_event(&context, &pressed, &canvas);
    menu::handle_color_input_event(&context, &document);
    menu::handle_alpha_input_event(&context, &document);
    menu::handle_width_input_event(&context, &document);
    // {
    //     let tmp_window = window.clone();
    //     let context = context.clone();

    //     let window_resize_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
    //         let tmp_image_data = context.get_image_data(0.0, 0.0, *width, *height).unwrap();
    //         let tmp_width = tmp_window.inner_width().unwrap().as_f64().unwrap();
    //         let tmp_height = tmp_window.inner_height().unwrap().as_f64().unwrap();
    //         context.canvas().unwrap().set_width(tmp_width as u32);
    //         context.canvas().unwrap().set_height(tmp_height as u32);
    //         context.put_image_data(&tmp_image_data, tmp_width, tmp_height);
    //         *width = tmp_width;
    //         *height = tmp_height;
            
    //         log(&width.to_string());
    //         log(&"resize event".to_string());
    //     }) as Box<dyn FnMut(_)>);
        
    //     window.add_event_listener_with_callback("resize", window_resize_closure.as_ref().unchecked_ref())?;
    //     window_resize_closure.forget();
    // }
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
