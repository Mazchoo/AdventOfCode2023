use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod modules;

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello to the console {}", name).into());
}

#[wasm_bindgen]
pub fn get_calibration_value(payload: &str) -> i32 {
    return modules::day1::sum_calibration_values(payload);
}