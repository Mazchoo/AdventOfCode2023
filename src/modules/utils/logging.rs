#[cfg(target_arch = "wasm32")]
use web_sys::console;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
pub fn log(payload: &str) {
    console::log_1(&JsValue::from_str(payload));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(payload: &str) {
    println!("{}", payload);
}
