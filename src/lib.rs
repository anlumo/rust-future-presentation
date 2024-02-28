use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() {
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();

    let loading = document.create_element("div").unwrap();
    loading.set_class_name("loading");
    loading.set_inner_html("Loading...");
    document.body().unwrap().append_child(&loading).unwrap();

    let canvas: web_sys::HtmlCanvasElement =
        document.create_element("canvas").unwrap().unchecked_into();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    document.body().unwrap().append_child(&canvas).unwrap();

    // let image_element = web_sys::HtmlImageElement::new();
}
