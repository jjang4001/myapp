
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

    document
        .get_element_by_id("alpha-slider")
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

    document
        .get_element_by_id("width-slider")
        .expect("document should have #alpha-slider on DOM")
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("#alpha-slider should be an HtmlElement")
        .add_event_listener_with_callback("input", width_slider_closure.as_ref().unchecked_ref())
        .expect("event listener should be added");
        
    width_slider_closure.forget();
}

pub fn handle_color_event(context: &Rc<web_sys::CanvasRenderingContext2d>, document: &web_sys::Document) {
    let context = context.clone();

    let color_picker = document
        .get_element_by_id("myapp-color-picker")
        .expect("document should have #myapp-color-picker on DOM")
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let hex_input_field = document
        .get_element_by_id("color-picker-hexa")
        .expect("document should have #color-picker-hexa on DOM")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    log(&hex_input_field.value());

    let color_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let color_hex = &hex_input_field.value();
        context.set_stroke_style(&JsValue::from_str(color_hex));
    }) as Box<dyn FnMut(_)>);

    color_picker.add_event_listener_with_callback("mouseup", color_closure.as_ref().unchecked_ref()).unwrap();
    color_closure.forget();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
