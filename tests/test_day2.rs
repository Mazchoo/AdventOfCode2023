use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day2;

#[test]
fn test_sample_input_1() {
    let path = Path::new("tests/example_input/day2_example1.txt");

    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result: i32 = day2::get_sum_valid_cube_configs(&payload);

    assert_eq!(result, 8);
}

#[test]
fn test_utf_safety_part_1() {
    let payload: &str = "🐒💨 Oh no";
    let result: i32 = day2::get_sum_valid_cube_configs(payload);
    assert_eq!(result, 0);
}

#[test]
fn test_empty_part_1() {
    let payload: &str = "";
    let result: i32 = day2::get_sum_valid_cube_configs(payload);
    assert_eq!(result, 0);
}
