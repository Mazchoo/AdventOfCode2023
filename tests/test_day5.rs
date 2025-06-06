use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day5;

#[test]
fn test_sample_input_part1() {
    let path = Path::new("tests/example_input/day5_example.txt");
    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result: i64 = day5::lowest_seed_location(&payload);

    assert_eq!(result, 35);
}
