use wasm_bindgen::prelude::*;
use wasm_console;


#[wasm_bindgen]
pub fn use_wasm_console() {
    wasm_console::log!("Hi!", true, 1.3);
}
