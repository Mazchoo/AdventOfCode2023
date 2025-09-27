use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day7;

#[test]
fn test_sample_input_pt1() {
    let path = Path::new("tests/example_input/day7_example.txt");
    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result = day7::multiply_bids_and_order(&payload);

    assert_eq!(result, 6440);
}
