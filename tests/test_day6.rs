use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day6;

#[test]
fn test_sample_input() {
    let path = Path::new("tests/example_input/day6_example.txt");
    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result = day6::boat_race_ownage(&payload);

    assert_eq!(result, 288);
}
