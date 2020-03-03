use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() {
    console::log_1(&"Hit wasm start() function!".into());
}
