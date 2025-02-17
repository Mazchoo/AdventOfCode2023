const BYTE_NUMBER_OFFSET: u8 = b'0';

/// Increment reference to some bytes by a certain amount or return empty bytes
/// ```
/// let result = crate::advent_code_wasm::modules::utils::streaming::increment_slice(&"abc".as_bytes(), 2);
/// assert_eq!(result, "c".as_bytes());
/// ```
pub fn increment_slice(slice: &[u8], offset: usize) -> &[u8] {
    return &slice.get(offset..).unwrap_or(&[]);
}

/// Increment reference to some bytes by a certain amount or return empty bytes
/// ```
/// let result = crate::advent_code_wasm::modules::utils::streaming::parse_number_from_stream(&"12c".as_bytes());
/// assert_eq!(result, (12, "c".as_bytes()));
/// ```
pub fn parse_number_from_stream(slice: &[u8]) -> (i32, &[u8]) {
    let mut total: i32 = 0;
    let mut output = slice;
    while let Some(b) = output.get(0) {
        if !b.is_ascii_digit() {
            break;
        }
        let digit = (b - BYTE_NUMBER_OFFSET) as i32;
        total *= 10;
        total += digit;
        output = increment_slice(output, 1);
    }
    return (total, output);
}