use ndarray::Array;

const BYTE_OFFSET: u8 = b'0';
const GEAR_SYMBOL: u8 = b'*';

/// Sum all digits that touch a symbol in a grid
/// ```
/// let result1 = crate::advent_code_wasm::modules::day3::get_sum_touching_numbers(".2.\n*.3\n.14");
/// assert_eq!(result1, 16);
/// let result2 = crate::advent_code_wasm::modules::day3::get_sum_touching_numbers("2%.\n..3\n.14");
/// assert_eq!(result2, 5);
/// let result3 = crate::advent_code_wasm::modules::day3::get_sum_touching_numbers("1..\n14&\n100");
/// assert_eq!(result3, 114);
/// let result4 = crate::advent_code_wasm::modules::day3::get_sum_touching_numbers(".4.\n1.2\n.$.");
/// assert_eq!(result4, 3);
/// ```
pub fn get_sum_touching_numbers(payload: &str) -> i32 {
    let payload_length = payload.bytes().len();

    if payload_length <= 1 {
        return 0; // expect non-trivial payload
    }

    let pitch;
    if let Some(ind) = payload.bytes().position(|b| b == b'\n') {
        pitch = ind + 1;
    } else {
        return 0; // expect multiple rows
    }

    if pitch < 2 {
        return 0; // expect row length > 1
    }

    let padding = if payload_length % pitch == 0 {
        0
    } else {
        pitch - payload_length % pitch
    };
    let mut mask = Array::from_elem(payload_length + padding, false);
    let last_line_ind = payload_length - pitch;

    for (i, value) in payload.bytes().enumerate() {
        if matches!(value, b'.' | b'\r' | b'\n') || value.is_ascii_digit() {
            {};
        } else {
            if i % pitch > 0 {
                mask[i - 1] = true;
                if i > pitch - 1 {
                    mask[i - pitch] = true;
                    mask[i - pitch - 1] = true;
                    if i % pitch < pitch - 1 {
                        mask[i + 1] = true;
                        mask[i - pitch + 1] = true;
                    }
                }
                if i < last_line_ind {
                    mask[i + pitch] = true;
                    mask[i + pitch - 1] = true;
                    if i % pitch < pitch - 1 {
                        mask[i + 1] = true;
                        mask[i + pitch + 1] = true;
                    }
                }
            } else {
                mask[i + 1] = true;
                if i > pitch - 1 {
                    mask[i - pitch] = true;
                    mask[i - pitch + 1] = true;
                }
                if i < last_line_ind {
                    mask[i + pitch] = true;
                    mask[i + pitch + 1] = true;
                }
            }
        }
    }

    let mut output: i32 = 0;
    let mut sub_total: i32 = 0;
    let mut touching_number: bool = true;
    let mut touching_mask: bool = false;

    for (i, value) in payload.bytes().enumerate() {
        if value.is_ascii_digit() {
            if !touching_number {
                sub_total = (value - BYTE_OFFSET) as i32;
                touching_number = true;
            } else {
                sub_total *= 10;
                sub_total += (value - BYTE_OFFSET) as i32;
            }
            if mask[i] {
                touching_mask = true;
            }
        } else {
            if touching_mask {
                output += sub_total;
            }
            touching_number = false;
            touching_mask = false;
        }
    }

    if touching_mask && sub_total > 0 {
        output += sub_total;
    }

    return output;
}

/// Ensure two numbers are still unique
/// ```
/// let result1 = crate::advent_code_wasm::modules::day3::two_numbers_stil_unique(&mut 1, &mut 2, &0);
/// assert_eq!(result1, true);
/// let result2 = crate::advent_code_wasm::modules::day3::two_numbers_stil_unique(&mut 1, &mut 0, &2);
/// assert_eq!(result2, true);
/// let result3 = crate::advent_code_wasm::modules::day3::two_numbers_stil_unique(&mut 0, &mut 0, &1);
/// assert_eq!(result3, true);
/// let result4 = crate::advent_code_wasm::modules::day3::two_numbers_stil_unique(&mut 1, &mut 2, &3);
/// assert_eq!(result4, false);
/// ```
pub fn two_numbers_stil_unique(num1: &mut i32, num2: &mut i32, new_num: &i32) -> bool {
    if *new_num > 0 {
        if *num1 == 0 {
            *num1 = *new_num;
            return true;
        } else if *num1 == *new_num {
            return true;
        } else if *num2 == 0 {
            *num2 = *new_num;
            return true;
        } else if *num2 == *new_num {
            return true;
        } else {
            return false;
        }
    }
    return true;
}

/// Sum product of gear symbols
/// ```
/// let result1 = crate::advent_code_wasm::modules::day3::get_gear_multiplication(".2.\n*.3\n.14");
/// assert_eq!(result1, 28);
/// let result2 = crate::advent_code_wasm::modules::day3::get_gear_multiplication("2..\n.*3\n.14");
/// assert_eq!(result2, 0);
/// let result3 = crate::advent_code_wasm::modules::day3::get_gear_multiplication("1..\n14*\n100");
/// assert_eq!(result3, 1400);
/// let result4 = crate::advent_code_wasm::modules::day3::get_gear_multiplication(".4.\n1.2\n.*.");
/// assert_eq!(result4, 2);
/// let result5 = crate::advent_code_wasm::modules::day3::get_gear_multiplication("456\n1..\n.*.");
/// assert_eq!(result5, 0);
/// let result6 = crate::advent_code_wasm::modules::day3::get_gear_multiplication("4..\n.*.\n..3");
/// assert_eq!(result6, 12);
/// ```
pub fn get_gear_multiplication(payload: &str) -> i32 {
    let payload_length = payload.bytes().len();

    if payload_length <= 1 {
        return 0; // expect non-trivial payload
    }

    let pitch;
    if let Some(ind) = payload.bytes().position(|b| b == b'\n') {
        pitch = ind + 1;
    } else {
        return 0; // expect multiple rows
    }
    let mut num_ids = Array::from_elem(payload_length, 0);

    if pitch < 2 {
        return 0; // expect row length > 1
    }

    // Dynamic growth should be approached with caution in wasm
    let mut id_value_mapping: Vec<i32> = Vec::with_capacity(1000);
    id_value_mapping.push(0); // id 0 is nothing
    let mut current_id: i32 = 0;
    let mut sub_total: i32 = 0;
    let mut touching_number: bool = false;
    for (c, value) in payload.bytes().zip(num_ids.iter_mut()) {
        if c.is_ascii_digit() {
            if !touching_number {
                sub_total = (c - BYTE_OFFSET) as i32;
                touching_number = true;
                current_id += 1;
            } else {
                sub_total *= 10;
                sub_total += (c - BYTE_OFFSET) as i32;
            }

            *value = current_id;
        } else if touching_number {
            id_value_mapping.push(sub_total);
            sub_total = 0;
            touching_number = false;
        }
    }

    if touching_number && sub_total > 0 {
        id_value_mapping.push(sub_total);
    }

    let mut output: i32 = 0;
    for (i, c) in payload.bytes().enumerate() {
        if c == GEAR_SYMBOL {
            let mut id1: i32 = 0;
            let mut id2: i32 = 0;

            if i > 0 && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i - 1]) {
                continue;
            }
            if i >= pitch && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i - pitch]) {
                continue;
            }
            if i >= pitch + 1
                && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i - pitch - 1])
            {
                continue;
            }
            if i >= pitch - 1
                && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i - pitch + 1])
            {
                continue;
            }
            if i + 1 < payload_length
                && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i + 1])
            {
                continue;
            }
            if i + pitch < payload_length
                && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i + pitch])
            {
                continue;
            }
            if i + pitch + 1 < payload_length
                && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i + pitch + 1])
            {
                continue;
            }
            if i + pitch - 1 < payload_length
                && !two_numbers_stil_unique(&mut id1, &mut id2, &num_ids[i + pitch - 1])
            {
                continue;
            }

            if id1 > 0 && id2 > 0 {
                output += id_value_mapping[id1 as usize] * id_value_mapping[id2 as usize];
            }
        }
    }

    return output;
}
