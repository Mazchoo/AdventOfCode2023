// Find the sum of first and last digits in each string
use crate::modules::structs::character_trie::{CharacterTrieNode, TRIE};

const BYTE_OFFSET: i32 = b'0' as i32;

/// Convert two chars into a two digit integer
/// ```
/// let result = crate::advent_code_wasm::modules::day1::get_digit_contribution('2', '3');
/// assert_eq!(result, 23);
/// ```
pub fn get_digit_contribution(c1: char, c2: char) -> i32 {
    return (c1 as i32 - BYTE_OFFSET) * 10 + (c2 as i32 - BYTE_OFFSET);
}

/// Find the total of the first and last digit on each line
/// ```
/// let result = crate::advent_code_wasm::modules::day1::sum_calibration_values("235");
/// assert_eq!(result, 25);
/// ```
pub fn sum_calibration_values(payload: &str) -> i32 {
    let mut total: i32 = 0;

    let mut new_line: bool = true;
    let mut first_number: char = '0';
    let mut last_number: char = '0';

    for c in payload.chars() {
        if c.is_numeric() {
            if new_line {
                first_number = c;
                last_number = c;
                new_line = false;
            } else {
                last_number = c;
            }
        } else if c == '\n' {
            total += get_digit_contribution(first_number, last_number);
            new_line = true;
            first_number = '0';
            last_number = '0';
        }
    }

    if !new_line {
        total += get_digit_contribution(first_number, last_number);
    }

    return total;
}

/// Find the total of the first and last digit on each line, including words
/// ```
/// let result = crate::advent_code_wasm::modules::day1::sum_calibration_letters("eightwo");
/// assert_eq!(result, 82);
/// ```
pub fn sum_calibration_letters(payload: &str) -> i32 {
    let mut total: i32 = 0;

    let mut new_line: bool = true;
    let mut first_number: char = '0';
    let mut last_number: char = '0';
    let trie: &CharacterTrieNode = &*TRIE;
    let mut current_solutions: Vec<&CharacterTrieNode> = [].to_vec();

    'characters: for c in payload.chars() {
        if c.is_numeric() {
            if new_line {
                first_number = c;
                last_number = c;
                new_line = false;
            } else {
                last_number = c;
            }
            current_solutions.clear();
        } else if c == '\n' {
            total += get_digit_contribution(first_number, last_number);
            new_line = true;
            first_number = '0';
            last_number = '0';
            current_solutions.clear();
        } else {
            // old solutions takes ownership of solutions and clears original
            let old_solutions = std::mem::take(&mut current_solutions);
            for solution in old_solutions.iter() {
                if let Some(updated_solution) = solution.update_char(&c) {
                    // If we have found a word solution, update numbers
                    if updated_solution.value != '0' {
                        if new_line {
                            first_number = updated_solution.value;
                            last_number = updated_solution.value;
                            new_line = false;
                        } else {
                            last_number = updated_solution.value;
                        }
                        current_solutions.clear();

                        if let Some(new_solution) = trie.update_char(&c) {
                            current_solutions.push(new_solution);
                        }

                        continue 'characters;
                    } else {
                        current_solutions.push(updated_solution);
                    }
                }
            }

            if let Some(new_solution) = trie.update_char(&c) {
                current_solutions.push(new_solution);
            }
        }
    }

    if !new_line {
        total += get_digit_contribution(first_number, last_number);
    }

    return total;
}
