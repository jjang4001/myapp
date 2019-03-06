
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn handle_doodle_size_input_event(context: &Rc<web_sys::CanvasRenderingContext2d>, document: &web_sys::Document) {
    let context = context.clone();

    let size_slider_closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
        let size = &event
            .target().unwrap()
            .dyn_ref::<web_sys::HtmlInputElement>().unwrap()
            .value()
            .parse::<f64>().unwrap();
        context.set_line_width(*size);
    }) as Box<dyn FnMut(_)>);

    document
        .get_element_by_id("size-slider")
        .expect("document should have #alpha-slider on DOM")
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("#alpha-slider should be an HtmlElement")
        .add_event_listener_with_callback("input", size_slider_closure.as_ref().unchecked_ref())
        .expect("event listener should be added");
        
    size_slider_closure.forget();
}

pub fn handle_color_event(context: &Rc<web_sys::CanvasRenderingContext2d>, document: &web_sys::Document) {
    let context = context.clone();

    let color_picker = document
        .get_element_by_id("color-picker")
        .expect("document should have #color-picker on DOM")
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let hex_input_field = document
        .get_element_by_id("color-picker-hexa")
        .expect("document should have #color-picker-hexa on DOM")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    let alpha_input_field = document
        .get_element_by_id("color-picker-alpha")
        .expect("document should have #color-picker-alpha on DOM")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    let color_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let color_hex = &hex_input_field.value();
        let alpha = alpha_input_field.value().parse::<f64>().unwrap();
        context.set_stroke_style(&JsValue::from_str(color_hex));
        context.set_global_alpha(alpha);
    }) as Box<dyn FnMut(_)>);

    color_picker.add_event_listener_with_callback("mouseup", color_closure.as_ref().unchecked_ref()).unwrap();
    color_closure.forget();
}

pub fn handle_color_picker_input_event(context: &Rc<web_sys::CanvasRenderingContext2d>, document: &web_sys::Document) {
    let context = context.clone();

    // let input_element_ids: [str; 6] = ["color-picker-H", "color-picker-S", "color-picker-L", "color-picker-R", "color-picker-G", "color-picker-B"];

    let h = document.get_element_by_id("color-picker-H").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let s = document.get_element_by_id("color-picker-S").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let l = document.get_element_by_id("color-picker-L").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let r = document.get_element_by_id("color-picker-R").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let g = document.get_element_by_id("color-picker-G").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let b = document.get_element_by_id("color-picker-B").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();

    let alpha_input_field = document.get_element_by_id("color-picker-alpha").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let alpha_input_field_copy = alpha_input_field.clone();

    let hex_input_field = document
        .get_element_by_id("color-picker-hexa")
        .expect("document should have #color-picker-hexa on DOM")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    let color_input_closure = Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
        log(&"input closure".to_string());
        let color_hex = &hex_input_field.value();
        let alpha = alpha_input_field_copy.value().parse::<f64>().unwrap();
        context.set_stroke_style(&JsValue::from_str(color_hex));
        context.set_global_alpha(alpha);
    }) as Box<dyn FnMut(_)>);

    alpha_input_field.add_event_listener_with_callback("input", color_input_closure.as_ref().unchecked_ref()).unwrap();
    color_input_closure.forget();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
