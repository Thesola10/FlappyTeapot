use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("gameArea").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
}
