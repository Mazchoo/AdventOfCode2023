// Get ranking of hands according to ordering
use std::cmp::Ordering;
use std::cmp::Reverse;

const BYTE_OFFSET: u8 = b'0';

/// Compare two cards where each card is a char 2, 3, 4, ..., T, J, Q, K, A
/// Perform comparision where 2 < 3 ... < K < A
/// ```
/// use std::cmp::Ordering;
/// let result1 = crate::advent_code_wasm::modules::day7::compare_cards('A', 'K');
/// assert_eq!(result1, Ordering::Greater);
/// let result2 = crate::advent_code_wasm::modules::day7::compare_cards('3', '2');
/// assert_eq!(result2, Ordering::Greater);
/// let result2 = crate::advent_code_wasm::modules::day7::compare_cards('T', 'J');
/// assert_eq!(result2, Ordering::Less);
/// ```
pub fn compare_cards(card1: &char, card2: &char) -> std::cmp::Ordering {
    let card_value = |card: &char| -> u8 {
        match card {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0, // Invalid card
        }
    };

    let value1: u8 = card_value(card1);
    let value2: u8 = card_value(card2);

    return value1.cmp(&value2);
}

/// Compare two hands by value compare first values and if one is greater return
/// If both are equal move forward to compare next pair
/// ```
/// use std::cmp::Ordering;
/// let result1 = crate::advent_code_wasm::modules::day7::compare_cards_by_value(&['A', '2', '2', '2', '2'], &['K', 'A', 'A', 'A', 'A']);
/// assert_eq!(result1, Ordering::Greater);
/// let result2 = crate::advent_code_wasm::modules::day7::compare_cards_by_value(&['3', '3', '2', '2', '2'], &['3', '2', 'A', 'A', 'A']);
/// assert_eq!(result2, Ordering::Greater);
/// ```
pub fn compare_cards_by_value(hand1: &[char; 5], hand2: &[char; 5]) -> std::cmp::Ordering {
    for (card1, card2) in hand1.iter().zip(hand2.iter()) {
        let card_compare_result: Ordering = compare_cards(card1, card2);
        if card_compare_result != Ordering::Equal {
            return card_compare_result;
        }
    }
    return Ordering::Equal;
}

/// Return a a rank of the hand
/// All different cards -> 1
/// 2 of a kind -> 2
/// Two pairs -> 3
/// 3 of a kind -> 4
/// 3 of a kind and two of kind -> 5
/// 4 of a kind -> 6
/// 5 of a kind -> 7
/// ```
/// let result1 = crate::advent_code_wasm::modules::day7::get_hand_rank(&['A', '2', '2', '2', '2']);
/// assert_eq!(result1, 6); // four of a kind
/// let result2 = crate::advent_code_wasm::modules::day7::get_hand_rank(&['3', '3', '2', '2', '2']);
/// assert_eq!(result2, 5); // full house
/// let result3 = crate::advent_code_wasm::modules::day7::get_hand_rank(&['3', 'A', 'K', '3', 'Q']);
/// assert_eq!(result3, 2); // two of a kind
/// let result4 = crate::advent_code_wasm::modules::day7::get_hand_rank(&['Q', 'A', 'K', 'K', 'Q']);
/// assert_eq!(result4, 3); // two pairs
/// ```
pub fn get_hand_rank(hand: &[char; 5]) -> u8 {
    // Count frequency of each card
    let mut card_counts: [u8; 5] = [0, 0, 0, 0, 0];
    let mut card_freq: [char; 5] = ['0', '0', '0', '0', '0'];

    let mut i = 0;
    'card: for card in hand {
        for j in 0..(i + 1) {
            if card_freq[j] == *card || card_freq[j] == '0' {
                card_counts[j] += 1;
                card_freq[j] = *card;
                i += 1;
                continue 'card;
            }
        }
    }

    // Extract frequency values and sort in descending order
    card_counts.sort_by_key(|&x| Reverse(x));

    // Map frequency patterns to ranks
    return match card_counts.as_slice() {
        [5, 0, 0, 0, 0] => 7, // Five of a kind
        [4, 1, 0, 0, 0] => 6, // Four of a kind
        [3, 2, 0, 0, 0] => 5, // Full house
        [3, 1, 1, 0, 0] => 4, // Three of a kind
        [2, 2, 1, 0, 0] => 3, // Two pairs
        [2, 1, 1, 1, 0] => 2, // One pair
        [1, 1, 1, 1, 1] => 1, // High card
        _ => 0,               // Invalid hand
    };
}

/// Compare two hands together first check if one has greater rank
/// If ranks are the same, compare card by card
/// ```
/// use std::cmp::Ordering;
/// let result1 = crate::advent_code_wasm::modules::day7::compare_hands(&['A', '2', '2', '2', '2'], &['A', 'A', 'A', 'A', 'A']);
/// assert_eq!(result1, Ordering::Less);
/// let result2 = crate::advent_code_wasm::modules::day7::compare_hands(&['3', '3', '2', '2', '2'], &['3', '2', 'A', 'A', 'A']);
/// assert_eq!(result2, Ordering::Greater);
/// let result3 = crate::advent_code_wasm::modules::day7::compare_hands(&['A', 'A', 'A', 'A', 'A'], &['A', 'A', 'A', 'A', 'A']);
/// assert_eq!(result3, Ordering::Equal);
/// ```
pub fn compare_hands(hand1: &[char; 5], hand2: &[char; 5]) -> std::cmp::Ordering {
    let rank1: u8 = get_hand_rank(hand1);
    let rank2: u8 = get_hand_rank(hand2);

    return match rank1.cmp(&rank2) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => compare_cards_by_value(hand1, hand2),
        Ordering::Greater => Ordering::Greater,
    };
}

/// Turn string describing hands into arrays of chars for hands and
/// the bids read as i16 integers
/// ```
/// let result = crate::advent_code_wasm::modules::day7::parse_card_hands("32T3K 765");
/// assert_eq!(result, (vec![['3', '2', 'T', '3', 'K']], vec![765i16]));
/// ```
pub fn parse_card_hands(payload: &str) -> (Vec<[char; 5]>, Vec<i16>) {
    let mut hands: Vec<[char; 5]> = vec![];
    let mut bids: Vec<i16> = vec![];

    let mut reading_hand: bool = true;
    let mut current_hand: [char; 5] = ['2', '2', '2', '2', '2'];
    let mut current_bid: i16 = 0;
    let mut i: usize = 0;
    for c in payload.chars() {
        if c == ' ' {
            reading_hand = false;
            hands.push(current_hand);
        } else if c == '\n' {
            reading_hand = true;
            bids.push(current_bid);
            current_bid = 0;
            i = 0;
        } else {
            if reading_hand {
                current_hand[i] = c;
                i += 1;
            } else {
                let digit: i16 = (c as u8 - BYTE_OFFSET) as i16;
                current_bid *= 10;
                current_bid += digit;
            }
        }
    }

    if current_bid > 0 {
        bids.push(current_bid);
    }

    return (hands, bids);
}
