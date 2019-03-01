
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn handle_color_input_event(context: &Rc<web_sys::CanvasRenderingContext2d>, document: &web_sys::Document) {
    let context = context.clone();

    let color_picker_closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
        let color = &event
            .target().unwrap()
            .dyn_ref::<web_sys::HtmlInputElement>().unwrap()
            .value();
        context.set_stroke_style(&JsValue::from_str(color));
    }) as Box<dyn FnMut(_)>);

    document
        .get_element_by_id("color-picker")
        .expect("document should have #color-picker on DOM")
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("#color-picker should be an HtmlElement")
        .add_event_listener_with_callback("input", color_picker_closure.as_ref().unchecked_ref())
        .expect("event listener should be added");

    color_picker_closure.forget();
}

pub fn handle_alpha_input_event(context: &Rc<web_sys::CanvasRenderingContext2d>, document: &web_sys::Document) {
    let context = context.clone();

    let alpha_slider_closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
        let alpha = &event
            .target().unwrap()
            .dyn_ref::<web_sys::HtmlInputElement>().unwrap()
            .value()
            .parse::<f64>().unwrap();
        context.set_global_alpha(*alpha);
    }) as Box<dyn FnMut(_)>);

    document.get_element_by_id("alpha-slider")
        .expect("document should have #alpha-slider on DOM")
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("#alpha-slider should be an HtmlElement")
        .add_event_listener_with_callback("input", alpha_slider_closure.as_ref().unchecked_ref())
        .expect("event listener should be added");
    alpha_slider_closure.forget();
}

pub fn handle_width_input_event(context: &Rc<web_sys::CanvasRenderingContext2d>, document: &web_sys::Document) {
    let context = context.clone();

    let width_slider_closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
        let width = &event
            .target().unwrap()
            .dyn_ref::<web_sys::HtmlInputElement>().unwrap()
            .value()
            .parse::<f64>().unwrap();
        context.set_line_width(*width);
    }) as Box<dyn FnMut(_)>);

    document.get_element_by_id("width-slider")
        .expect("document should have #alpha-slider on DOM")
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("#alpha-slider should be an HtmlElement")
        .add_event_listener_with_callback("input", width_slider_closure.as_ref().unchecked_ref())
        .expect("event listener should be added");
        
    width_slider_closure.forget();
}
