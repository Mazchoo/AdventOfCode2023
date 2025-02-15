use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day1 as day1;

#[test]
fn test_create_pattern_object_example_file() {
    let path = Path::new("tests/example_input/day1_example.txt");

    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result = day1::sum_calibration_values(&payload);

    assert_eq!(result, 0);
}
