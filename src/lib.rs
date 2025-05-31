use modules::utils::logging::log;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn get_sum_valid_cube_configs(payload: &str) -> i32 {
    return modules::day2::get_sum_valid_cube_configs(payload);
}

#[wasm_bindgen]
pub fn get_mininmum_product_each_game(payload: &str) -> i32 {
    return modules::day2::get_mininmum_product_each_game(payload);
}

#[wasm_bindgen]
pub fn get_sum_touching_numbers(payload: &str) -> i32 {
    return modules::day3::get_sum_touching_numbers(payload);
}

#[wasm_bindgen]
pub fn get_gear_multiplication(payload: &str) -> i32 {
    return modules::day3::get_gear_multiplication(payload);
}

#[wasm_bindgen]
pub fn sum_scratch_card_values(payload: &str) -> u32 {
    return modules::day4::sum_scratch_card_values::<10>(payload);
}

#[wasm_bindgen]
pub fn sum_scratch_194_cards_recursive(payload: &str) -> u32 {
    return modules::day4::sum_recursive_scratch_cards::<10, 194>(payload);
}
