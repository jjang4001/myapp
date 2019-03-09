use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::models;
use super::messaging;

pub fn handle_mousedown_event(context: &Rc<web_sys::CanvasRenderingContext2d>, pressed: &Rc<Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        context.begin_path();
        context.move_to(event.offset_x() as f64, event.offset_y() as f64);
        pressed.set(true);
    }) as Box<dyn FnMut(_)>);
    
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget(); 
}

pub fn handle_mousemove_event(
    context: &Rc<web_sys::CanvasRenderingContext2d>,
    pressed: &Rc<Cell<bool>>,
    canvas: &web_sys::HtmlCanvasElement,
    a: messaging::messenger::Messenger
) {
    let context = context.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if pressed.get() {
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
            };
            let msg = stroke.to_string();
            a.send_message(&msg);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
        }
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget(); 
}

pub fn handle_mouseup_event(context: &Rc<web_sys::CanvasRenderingContext2d>, pressed: &Rc<Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        pressed.set(false);
        context.line_to(event.offset_x() as f64, event.offset_y() as f64);
        context.stroke();
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
