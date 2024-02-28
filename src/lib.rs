use wasm_bindgen::prelude::*;

mod image_future;
use image_future::ImageLoader;

#[wasm_bindgen(start)]
pub async fn run() {
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();

    let loading = document.create_element("div").unwrap();
    loading.set_class_name("loading");
    loading.set_inner_html("Loading...");
    document.body().unwrap().append_child(&loading).unwrap();

    let image = ImageLoader::load("image.png").await.unwrap();

    let canvas: web_sys::HtmlCanvasElement =
        document.create_element("canvas").unwrap().unchecked_into();
    canvas.set_width(800);
    canvas.set_height(800);
    document.body().unwrap().append_child(&canvas).unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context
        .draw_image_with_html_image_element_and_dw_and_dh(&image, 0.0, 0.0, 800.0, 800.0)
        .unwrap();
    loading.remove();
}
