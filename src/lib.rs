use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlCanvasElement};

use piet::RenderContext;
use piet_web::WebRenderContext;

mod game;

#[wasm_bindgen(start)]
pub fn start() {
    console::log_1(&"Loading Flappy Teapot WASM module".into());

    let window = window().unwrap();
    let canvas =  window.document()                     .unwrap()
                        .get_element_by_id("gameArea")  .unwrap()
                        .dyn_into::<HtmlCanvasElement>().unwrap();
    let mut context = canvas.get_context("2d")                     .unwrap().unwrap()
                            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    let dpr = window.device_pixel_ratio();
    canvas.set_width((canvas.width() as f64 * dpr) as u32);
    canvas.set_height((canvas.height() as f64 * dpr) as u32);
    let _ = context.scale(dpr, dpr);

    let mut piet_context = WebRenderContext::new(&mut context, &window);

    game::mainloop(&piet_context);

    piet_context.finish().unwrap();
}
