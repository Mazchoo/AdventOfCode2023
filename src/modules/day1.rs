// Find the sum of first and last digits in each string
use web_sys::console;

#[cfg(not(target_os = "windows"))]
pub fn log(payload: &str) {
    console::log_1(&JsValue::from_str(payload));
}

#[cfg(target_os = "windows")]
pub fn log(payload: &str) {
    println!("{}", payload);
}

pub fn sum_calibration_values(payload: &str) -> i32 {
    log(payload);
    return 0;
}
