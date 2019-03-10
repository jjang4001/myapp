use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::models;
use super::messaging;

pub fn handle_mousedown_event(
    context: &Rc<web_sys::CanvasRenderingContext2d>,
    pressed: &Rc<Cell<bool>>,
    canvas: &web_sys::HtmlCanvasElement,
    messenger: &Rc<messaging::messenger::Messenger>
) {
    let context = context.clone();
    let pressed = pressed.clone();
    let messenger = messenger.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let rgb = context.stroke_style().as_string().unwrap();
        let alpha = context.global_alpha();
        let x = event.offset_x() as f64; 
        let y = event.offset_y() as f64;
        let size = context.line_width();
        let stroke = models::stroke::Stroke {
            rgb: rgb,
            alpha: alpha,
            x: x,
            y: y,
            size: size,
            endpoint: String::from("START"),
        };
        let msg = stroke.to_string();
        messenger.send_message(&msg);
        
        context.begin_path();
        context.move_to(x, y);
        pressed.set(true);
    }) as Box<dyn FnMut(_)>);
    
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget(); 
}

pub fn handle_mousemove_event(
    context: &Rc<web_sys::CanvasRenderingContext2d>,
    pressed: &Rc<Cell<bool>>,
    canvas: &web_sys::HtmlCanvasElement,
    messenger: &Rc<messaging::messenger::Messenger>
) {
    let context = context.clone();
    let pressed = pressed.clone();
    let messenger = messenger.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if pressed.get() {
            let rgb = context.stroke_style().as_string().unwrap();
            let alpha = context.global_alpha();
            let x = event.offset_x() as f64; 
            let y = event.offset_y() as f64;
            let size = context.line_width();
            // 
            let stroke = models::stroke::Stroke {
                rgb: rgb,
                alpha: alpha,
                x: x,
                y: y,
                size: size,
                endpoint: String::from("MOVE"),
            };
            let msg = stroke.to_string();
            messenger.send_message(&msg);
            context.line_to(x, y);
            context.stroke();
            context.begin_path();
            context.move_to(x, y);
        }
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget(); 
}

pub fn handle_mouseup_event(
    context: &Rc<web_sys::CanvasRenderingContext2d>,
    pressed: &Rc<Cell<bool>>,
    canvas: &web_sys::HtmlCanvasElement,
    messenger: &Rc<messaging::messenger::Messenger>
) {
    let context = context.clone();
    let pressed = pressed.clone();
    let messenger = messenger.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let rgb = context.stroke_style().as_string().unwrap();
        let alpha = context.global_alpha();
        let x = event.offset_x() as f64; 
        let y = event.offset_y() as f64;
        let size = context.line_width();
        let stroke = models::stroke::Stroke {
            rgb: rgb,
            alpha: alpha,
            x: x,
            y: y,
            size: size,
            endpoint: String::from("END"),
        };
        let msg = stroke.to_string();
        messenger.send_message(&msg);
        
        pressed.set(false);
        context.line_to(x, y);
        context.stroke();
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
