use std::fs;
use std::path::Path;

use advent_code_wasm::modules::day4;

#[test]
fn test_sample_input_part1() {
    let path = Path::new("tests/example_input/day4_example.txt");
    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result: u32 = day4::sum_scratch_card_values::<5>(&payload);

    assert_eq!(result, 13);
}

#[test]
fn test_sample_input_part2() {
    let path = Path::new("tests/example_input/day4_example.txt");
    let payload: String = fs::read_to_string(path).expect("Unable to read text file");

    let result: u32 = day4::sum_recursive_scratch_cards::<5, 6>(&payload);

    assert_eq!(result, 30);
}
