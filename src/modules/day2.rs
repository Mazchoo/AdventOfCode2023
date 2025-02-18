// Cobinations of cubes
use crate::modules::utils::streaming::{increment_slice, parse_number_from_stream};

struct RGB {
    red: i32,
    green: i32,
    blue: i32,
}

static CUBE_CONFIG: RGB = RGB {
    red: 12,
    green: 13,
    blue: 14,
};

static GREEN: &[u8] = "green".as_bytes();
static BLUE: &[u8] = "blue".as_bytes();
static RED: &[u8] = "red".as_bytes();
static GAME: &[u8] = "Game ".as_bytes();
const NEW_LINE: u8 = b'\n';

/// Sum up indices of lines which have color numbers greater than limits
/// ```
/// let result = crate::advent_code_wasm::modules::day2::get_sum_valid_cube_configs("Game 1: 13 red\nGame 2: 1 blue");
/// assert_eq!(result, 2);
/// ```
pub fn get_sum_valid_cube_configs(payload: &str) -> i32 {
    let mut new_line_ended: bool = true;
    let mut output: i32 = 0;
    let mut line_valid: bool = true;
    let mut slice: &[u8] = payload.as_bytes();
    let mut current_game: i32 = 1;
    let mut current_number: i32 = 0;

    while !slice.is_empty() {
        if matches!(slice[0], b' ' | b',' | b';' | b':') {
            slice = increment_slice(slice, 1);
        } else if slice[0].is_ascii_digit() {
            (current_number, slice) = parse_number_from_stream(slice);
        } else if slice.starts_with(GAME) {
            new_line_ended = false;
            slice = increment_slice(slice, GAME.len());
            while slice[0].is_ascii_digit() {
                slice = increment_slice(slice, 1);
            }
        } else if slice.starts_with(RED) {
            if current_number > CUBE_CONFIG.red {
                line_valid = false;
            }
            slice = increment_slice(slice, RED.len());
        } else if slice.starts_with(GREEN) {
            if current_number > CUBE_CONFIG.green {
                line_valid = false;
            }
            slice = increment_slice(slice, GREEN.len());
        } else if slice.starts_with(BLUE) {
            if current_number > CUBE_CONFIG.blue {
                line_valid = false;
            }
            slice = increment_slice(slice, BLUE.len());
        } else if slice[0] == NEW_LINE {
            if line_valid {
                output += current_game;
            }
            new_line_ended = true;
            current_game += 1;
            line_valid = true;
            slice = increment_slice(slice, 1);
        } else {
            slice = increment_slice(slice, 1);
        }
    }

    if !new_line_ended {
        if line_valid {
            output += current_game;
        }
    }

    return output;
}

/// Sum up indices of lines which have color numbers greater than limits
/// ```
/// let result = crate::advent_code_wasm::modules::day2::get_mininmum_product_each_game("Game 1: 3 red 1 blue 2 green");
/// assert_eq!(result, 6);
/// ```
pub fn get_mininmum_product_each_game(payload: &str) -> i32 {
    let mut new_line_ended: bool = true;
    let mut output: i32 = 0;
    let mut slice: &[u8] = payload.as_bytes();
    let mut current_number: i32 = 0;

    let mut red_max: i32 = 0;
    let mut green_max: i32 = 0;
    let mut blue_max: i32 = 0;

    while !slice.is_empty() {
        if matches!(slice[0], b' ' | b',' | b';' | b':') {
            slice = increment_slice(slice, 1);
        } else if slice[0].is_ascii_digit() {
            (current_number, slice) = parse_number_from_stream(slice);
        } else if slice.starts_with(GAME) {
            new_line_ended = false;
            slice = increment_slice(slice, GAME.len());
            while slice[0].is_ascii_digit() {
                slice = increment_slice(slice, 1);
            }
        } else if slice.starts_with(RED) {
            red_max = red_max.max(current_number);
            slice = increment_slice(slice, RED.len());
        } else if slice.starts_with(GREEN) {
            green_max = green_max.max(current_number);
            slice = increment_slice(slice, GREEN.len());
        } else if slice.starts_with(BLUE) {
            blue_max = blue_max.max(current_number);
            slice = increment_slice(slice, BLUE.len());
        } else if slice[0] == NEW_LINE {
            output += red_max * green_max * blue_max;
            red_max = 0;
            green_max = 0;
            blue_max = 0;
            new_line_ended = true;
            slice = increment_slice(slice, 1);
        } else {
            slice = increment_slice(slice, 1);
        }
    }

    if !new_line_ended {
        output += red_max * green_max * blue_max;
    }

    return output;
}
