// Find the sum of first and last digits in each string
#[cfg(target_arch = "wasm32")]
use web_sys::console;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
pub fn log(payload: &str) {
    console::log_1(&JsValue::from_str(payload));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(payload: &str) {
    println!("{}", payload);
}

const BYTE_OFFSET: i32 = b'0' as i32;

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
            total += (first_number as i32 - BYTE_OFFSET) * 10 + (last_number as i32 - BYTE_OFFSET);
            new_line = true;
            first_number = '0';
            last_number = '0';
        }
    }

    if !new_line {
        total += (first_number as i32 - BYTE_OFFSET) * 10 + (last_number as i32 - BYTE_OFFSET);
    }

    return total;
}
