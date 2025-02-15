// Find the sum of first and last digits in each string
#[cfg(target_arch = "wasm32")]
use web_sys::console;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[cfg(target_arch = "wasm32")]
pub fn log(payload: &str) {
    console::log_1(&JsValue::from_str(payload));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(payload: &str) {
    println!("{}", payload);
}

const BYTE_OFFSET: i32 = b'0' as i32;

fn get_digit_contribution(c1: char, c2: char) -> i32 {
    return (c1 as i32 - BYTE_OFFSET) * 10 + (c2 as i32 - BYTE_OFFSET);
}

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


fn map_word_to_value(word: &str) -> char {
    if word == "one" {
        return '1';
    } else if word == "two" {
        return '2';
    } else if word == "three" {
        return '3';
    } else if word == "four" {
        return '4';
    } else if word == "five" {
        return '5';
    } else if word == "six" {
        return '6';
    } else if word == "seven" {
        return '7';
    } else if word == "eight" {
        return '8';
    } else if word == "nine" {
        return '9';
    } else {
        return '0';
    }
}


#[derive(Debug, Default)]
struct CharacterTrieNode {
    children: HashMap<char, CharacterTrieNode>,
    value: char,
}

impl CharacterTrieNode {
    fn new() -> Self {
        CharacterTrieNode {
            children: HashMap::new(),
            value: '0',
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = self;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(CharacterTrieNode::new);
        }
        current.value = map_word_to_value(word);
    }

    fn update_char(&self, ch: &char) -> Option<&CharacterTrieNode> {
        match self.children.get(&ch) {
            Some(child) => return Some(child),
            None => return Option::None,
        }
    }
}

// Use a trie, popularised by Edward Fredkin
// Using just the first two chracters would have been a more C like solution
// Use lazy static intialization to create immutable static at runtime
static TRIE: Lazy<CharacterTrieNode> = Lazy::new(|| {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine"
    ];

    let mut trie = CharacterTrieNode::new();
    for word in WORDS.iter() {
        trie.insert(word);
    }
    return trie;
});

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
