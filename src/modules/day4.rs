
const BYTE_OFFSET: u8 = b'0';

pub fn sum_scratch_card_values(payload: &str) -> i32 {
    let mut winning_ind: usize = 0;
    let mut winning_numbers: [u8; 5] = [0, 0, 0, 0, 0];
    let mut current_value: u8 = 0;
    let mut current_line_value: u8 = 0;
    let mut total: i32 = 0;

    for c in payload.chars() {
        if c.is_numeric() {
            // Read next digit
            current_value *= 10;
            current_value += (c as u8) - BYTE_OFFSET;
        } else if (c == ' ' || c == '\n') && current_value > 0 {
            // flush current value
            if winning_ind > 4 {
                if winning_numbers.contains(&current_value) {
                    if current_line_value == 0 {
                        current_line_value = 1;
                    } else {
                        current_line_value *= 2;
                    }
                }
            } else {
                winning_numbers[winning_ind] = current_value;
                winning_ind += 1;
            }

            current_value = 0;
        } else {
            current_value = 0;
        }

        if c == '\n' {
            // flush current line
            winning_ind = 0;
            total += current_line_value as i32;
            current_line_value = 0;
        }
    }

    total += current_line_value as i32;

    return total;
}
