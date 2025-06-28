use crate::modules::utils::streaming::{increment_slice, parse_number_from_stream_i32};

pub fn parse_times(payload: &str) -> (Vec<i32>, Vec<i32>) {
    let mut times: Vec<i32> = vec![];
    let mut best_times: Vec<i32> = vec![];

    let mut reading_times = true;
    let mut current_number: i32;
    let mut slice: &[u8] = payload.as_bytes();
    while !slice.is_empty() {
        if slice[0].is_ascii_digit() {
            (current_number, slice) = parse_number_from_stream_i32(slice);
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

pub fn boat_race_ownage(payload: &str) -> f32 {
    let (time, best_times) = parse_times(payload);

    return 0.;
}
