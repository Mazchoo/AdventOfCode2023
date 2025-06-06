const BYTE_OFFSET: u8 = b'0';

#[inline]
fn update_line_value<const N: usize>(
    winning_numbers: &[u8; N],
    current_value: &u8,
    current_line_value: &mut u32,
) {
    if winning_numbers.contains(current_value) {
        if *current_line_value == 0 {
            *current_line_value = 1;
        } else {
            *current_line_value *= 2;
        }
    }
}

/// Count number of found numbers on some scratch cards
/// ```
/// let result1 = crate::advent_code_wasm::modules::day4::sum_scratch_card_values::<2>("1 3 | 3 1 2 1");
/// assert_eq!(result1, 4);
/// let result2 = crate::advent_code_wasm::modules::day4::sum_scratch_card_values::<2>("11 35 | 35 11 1 1");
/// assert_eq!(result2, 2);
/// let result3 = crate::advent_code_wasm::modules::day4::sum_scratch_card_values::<3>("1 2 3 | 1 2 4 2 3");
/// assert_eq!(result3, 8);
/// ```
pub fn sum_scratch_card_values<const N: usize>(payload: &str) -> u32 {
    let mut winning_ind: usize = 0;
    let mut winning_numbers: [u8; N] = [0; N];
    let mut current_value: u8 = 0;
    let mut current_line_value: u32 = 0;
    let mut total: u32 = 0;

    for c in payload.chars() {
        if c.is_numeric() {
            // Read next digit
            current_value *= 10;
            current_value += (c as u8) - BYTE_OFFSET;
        } else if (c == ' ' || c == '\n' || c == '\r') && current_value > 0 {
            // flush current value
            if winning_ind > N - 1 {
                update_line_value::<N>(&winning_numbers, &current_value, &mut current_line_value);
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
            total += current_line_value;
            current_line_value = 0;
        }
    }

    if current_value > 0 && winning_ind > N - 1 {
        // flush remaining value if present
        update_line_value::<N>(&winning_numbers, &current_value, &mut current_line_value);
    }

    total += current_line_value;

    return total;
}

/// Sum number o
/// ```
/// let result = crate::advent_code_wasm::modules::day4::sum_recursive_scratch_cards::<2, 3>("1 3 | 3 \n 1 2 | 1 2 \n 1 | 1 1");
/// assert_eq!(result, 6);
/// ```
pub fn sum_recursive_scratch_cards<const N: usize, const M: usize>(payload: &str) -> u32 {
    let mut winning_ind: usize = 0;
    let mut winning_numbers: [u8; N] = [0; N];
    let mut current_value: u8 = 0;
    let mut current_line_value: u32 = 0;

    let mut card_ind: usize = 0;
    let mut card_nr_winning: [u32; M] = [0; M];

    for c in payload.chars() {
        if c.is_numeric() {
            // Read next digit
            current_value *= 10;
            current_value += (c as u8) - BYTE_OFFSET;
        } else if (c == ' ' || c == '\n' || c == '\r') && current_value > 0 {
            // flush current value
            if winning_ind > N - 1 {
                if winning_numbers.contains(&current_value) {
                    current_line_value += 1;
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
            card_nr_winning[card_ind] = current_line_value;
            current_line_value = 0;
            card_ind += 1;
            winning_ind = 0;
        }
    }

    if current_value > 0 && winning_ind > N - 1 && winning_numbers.contains(&current_value) {
        // flush remaining value if present
        current_line_value += 1;
    }

    if current_line_value > 0 {
        card_nr_winning[card_ind] = current_line_value;
    }

    let mut card_values: [u32; M] = [1; M];
    for i in (0..=M - 1).rev() {
        if card_nr_winning[i] == 0 {
            continue;
        };
        for j in (i + 1)..=(i + card_nr_winning[i] as usize) {
            if j > M - 1 {
                break;
            };
            card_values[i] += card_values[j];
        }
    }

    return card_values.iter().sum();
}
