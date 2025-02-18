use ndarray::Array;

const BYTE_OFFSET: u8 = b'0';

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
        return 0;  // expect multiple rows
    }
    let padding = if payload_length % pitch == 0 { 0 } else { pitch - payload_length % pitch };
    let mut mask = Array::from_elem(payload_length + padding, false);
    let last_line_ind = payload_length - pitch;

    if pitch < 2 {
        return 0; // expect row length > 1
    }

    for (i, value) in payload.bytes().enumerate() {
        if matches!(value, b'.' | b'\r' | b'\n') || value.is_ascii_digit() {
            {};
        } else {
            mask[i] = true;
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
                sub_total += (value - BYTE_OFFSET) as i32;
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
            sub_total = 0;
        }
    }

    if touching_mask && sub_total > 0 {
        output += sub_total;
    }

    return output;
}
