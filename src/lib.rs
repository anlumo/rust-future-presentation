use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Hello World!".into());

    let canvas: web_sys::HtmlCanvasElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .unchecked_into();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();

    // let image_element = web_sys::HtmlImageElement::new();
}
