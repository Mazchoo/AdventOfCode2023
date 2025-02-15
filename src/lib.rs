use wasm_bindgen::prelude::*;
use modules::utils::logging::log;

pub mod modules;

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello to the console {}", name));
}

#[wasm_bindgen]
pub fn get_calibration_value(payload: &str) -> i32 {
    return modules::day1::sum_calibration_values(payload);
}

#[wasm_bindgen]
pub fn get_calibration_letters(payload: &str) -> i32 {
    return modules::day1::sum_calibration_letters(payload);
}
