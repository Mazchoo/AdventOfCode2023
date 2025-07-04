use crate::modules::utils::streaming::{increment_slice, parse_number_from_stream_i64};

pub fn parse_times(payload: &str) -> (Vec<i64>, Vec<i64>) {
    let mut times: Vec<i64> = vec![];
    let mut best_times: Vec<i64> = vec![];

    let mut reading_times = true;
    let mut current_number: i64;
    let mut slice: &[u8] = payload.as_bytes();
    while !slice.is_empty() {
        if slice[0].is_ascii_digit() {
            (current_number, slice) = parse_number_from_stream_i64(slice);
            if reading_times {
                times.push(current_number);
            } else {
                best_times.push(current_number);
            }
        } else if slice[0] == b'\n' {
            reading_times = false;
            slice = increment_slice(slice, 1);
        } else {
            slice = increment_slice(slice, 1);
        }
    }

    return (times, best_times);
}

pub fn parse_single_time(payload: &str) -> (Vec<i64>, Vec<i64>) {
    let mut times: Vec<i64> = vec![];
    let mut best_times: Vec<i64> = vec![];

    let mut total_number: i64 = 0;
    let mut current_number: i64;
    let mut slice: &[u8] = payload.as_bytes();
    while !slice.is_empty() {
        if slice[0].is_ascii_digit() {
            (current_number, slice) = parse_number_from_stream_i64(slice);

            total_number *= 10i64.pow(current_number.abs().to_string().len() as u32);
            total_number += current_number;
        } else if slice[0] == b'\n' {
            times.push(total_number);
            total_number = 0;
            slice = increment_slice(slice, 1);
        } else {
            slice = increment_slice(slice, 1);
        }
    }
    best_times.push(total_number);

    return (times, best_times);
}

/// Give the number of integers in the range of polynomial with
/// a x^2 + b x + c, where b, c are integers
/// ```
/// let result1 = crate::advent_code_wasm::modules::day6::get_quadtratic_root_range(&1, &-7, &9);
/// assert_eq!(result1, 4);
/// let result2 = crate::advent_code_wasm::modules::day6::get_quadtratic_root_range(&1, &-15, &40);
/// assert_eq!(result2, 8);
/// let result3 = crate::advent_code_wasm::modules::day6::get_quadtratic_root_range(&1, &-30, &200);
/// assert_eq!(result3, 9);
/// ```
pub fn get_quadtratic_root_range(a: &i64, b: &i64, c: &i64) -> i64 {
    let discriminant = b * b - 4 * c * a;
    if discriminant < 0 {
        return 0;
    }
    let a_float = *a as f64;
    let b_float = *b as f64;
    let c_float = *c as f64;

    let sqrt_discrim = (discriminant as f64).sqrt() * b_float.signum();
    let q = -0.5 * (b_float + sqrt_discrim);
    let x1 = q / a_float;
    let x2 = c_float / q;

    let mut lower_bound = x1.min(x2);
    let mut upper_bound = x1.max(x2);
    if lower_bound.fract() == 0. {
        lower_bound += 0.5;
    }
    if upper_bound.fract() == 0. {
        upper_bound -= 0.5;
    }
    let upper_int = upper_bound.floor() as i64;
    let lower_int = lower_bound.ceil() as i64;

    if upper_int < lower_int {
        return 0;
    }

    return upper_int - lower_int + 1;
}

pub fn get_winning_time_product(times: Vec<i64>, best_times: Vec<i64>) -> i64 {
    return times
        .iter()
        .zip(best_times.iter())
        .map(|(x, y)| get_quadtratic_root_range(&1i64, &-x, y))
        .collect::<Vec<i64>>()
        .iter()
        .product();
}

pub fn boat_race_ownage(payload: &str) -> i64 {
    let (times, best_times) = parse_times(payload);

    return get_winning_time_product(times, best_times);
}

pub fn boat_race_one_time(payload: &str) -> i64 {
    let (times, best_times) = parse_single_time(payload);

    return get_winning_time_product(times, best_times);
}
