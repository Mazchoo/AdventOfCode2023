use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day3;

#[test]
fn test_sample_input_1() {
    let path = Path::new("tests/example_input/day3_example.txt");

    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result: i32 = day3::get_sum_touching_numbers(&payload);

    assert_eq!(result, 4361);
}

#[test]
fn test_utf_safety_part_1() {
    let payload: &str = "🐒💨 Oh no";
    let result: i32 = day3::get_sum_touching_numbers(payload);
    assert_eq!(result, 0);
}

#[test]
fn test_empty_part_1() {
    let payload: &str = "";
    let result: i32 = day3::get_sum_touching_numbers(payload);
    assert_eq!(result, 0);
}


#[test]
fn test_no_pitch_part_1() {
    let payload: &str = "\n\n\n\n";
    let result: i32 = day3::get_sum_touching_numbers(payload);
    assert_eq!(result, 0);
}

