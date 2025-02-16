use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day1;

#[test]
fn test_sample_input_1() {
    let path = Path::new("tests/example_input/day1_example1.txt");

    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result: i32 = day1::sum_calibration_values(&payload);

    assert_eq!(result, 142);
}

#[test]
fn test_utf_safety_part_1() {
    let payload: &str = "🐒💨 Oh no";
    let result: i32 = day1::sum_calibration_values(payload);
    assert_eq!(result, 0);
}

#[test]
fn test_sample_input_2() {
    let path = Path::new("tests/example_input/day1_example2.txt");

    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result: i32 = day1::sum_calibration_letters(&payload);

    assert_eq!(result, 281);
}

#[test]
fn test_utf_safety_part_2() {
    let payload: &str = "🐒💨 Oh no";
    let result: i32 = day1::sum_calibration_letters(payload);
    assert_eq!(result, 0);
}
