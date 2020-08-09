use wasm_bindgen::prelude::*;
use wasm_console;


#[wasm_bindgen]
pub fn use_wasm_console() {
    wasm_console::log!("Hi!", true, 1.3);
    wasm_console::debug!("Hi!", true, 1.3);
    wasm_console::info!("Hi!", true, 1.3);
    wasm_console::warn!("Hi!", true, 1.3);
    wasm_console::error!("Hi!", true, 1.3);
}
