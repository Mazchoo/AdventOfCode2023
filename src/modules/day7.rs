// Get ranking of hands according to ordering
use std::cmp::Ordering;
use std::cmp::Reverse;

const BYTE_OFFSET: u8 = b'0';

pub struct Game(pub [char; 5], pub u8, pub i16);

/// Compare two cards where J is wild where each card is a char
/// Perform comparision where J < 2 < 3 ... T < Q < K < A
/// ```
/// use std::cmp::Ordering;
/// use crate::advent_code_wasm::modules::day7::compare_wild_cards;
/// let result1 = compare_wild_cards(&'A', &'K');
/// assert_eq!(result1, Ordering::Greater);
/// let result2 = compare_wild_cards(&'3', &'2');
/// assert_eq!(result2, Ordering::Greater);
/// let result2 = compare_wild_cards(&'J', &'T');
/// assert_eq!(result2, Ordering::Less);
/// ```
pub fn compare_wild_cards(card1: &char, card2: &char) -> std::cmp::Ordering {
    let card_value = |card: &char| -> u8 {
        match card {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => 0, // Invalid card
        }
    };

    let value1: u8 = card_value(card1);
    let value2: u8 = card_value(card2);

    return value1.cmp(&value2);
}

/// Compare two cards where each card is a char 2, 3, 4, ..., T, J, Q, K, A
/// Perform comparision where 2 < 3 ... < K < A
/// ```
/// use std::cmp::Ordering;
/// use crate::advent_code_wasm::modules::day7::compare_cards;
/// let result1 = compare_cards(&'A', &'K');
/// assert_eq!(result1, Ordering::Greater);
/// let result2 = compare_cards(&'3', &'2');
/// assert_eq!(result2, Ordering::Greater);
/// let result2 = compare_cards(&'T', &'J');
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
/// use crate::advent_code_wasm::modules::day7::compare_cards_by_value;
/// let result1 = compare_cards_by_value::<false>(&['A', '2', '2', '2', '2'], &['K', 'A', 'A', 'A', 'A']);
/// assert_eq!(result1, Ordering::Greater);
/// let result2 = compare_cards_by_value::<false>(&['3', '3', '2', '2', '2'], &['3', '2', 'A', 'A', 'A']);
/// assert_eq!(result2, Ordering::Greater);
/// ```
pub fn compare_cards_by_value<const WILD: bool>(
    hand1: &[char; 5],
    hand2: &[char; 5],
) -> std::cmp::Ordering {
    for (card1, card2) in hand1.iter().zip(hand2.iter()) {
        let card_compare_result: Ordering = if WILD {
            compare_wild_cards(card1, card2)
        } else {
            compare_cards(card1, card2)
        };
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
/// use crate::advent_code_wasm::modules::day7::get_hand_rank;
/// let result1 = get_hand_rank(&['A', '2', '2', '2', '2']);
/// assert_eq!(result1, 6); // four of a kind
/// let result2 = get_hand_rank(&['3', '3', '2', '2', '2']);
/// assert_eq!(result2, 5); // full house
/// let result3 = get_hand_rank(&['3', 'A', 'K', '3', 'Q']);
/// assert_eq!(result3, 2); // two of a kind
/// let result4 = get_hand_rank(&['Q', 'A', 'K', 'K', 'Q']);
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

/// Using J as a wild card get the best possible rank
/// If there are no J's the rank is the same as get_hand_rank
/// ```
/// use crate::advent_code_wasm::modules::day7::get_wild_hand_rank;
/// let result1 = get_wild_hand_rank(&['T', '5', '5', 'J', '5']);
/// assert_eq!(result1, 6);
/// let result2 = get_wild_hand_rank(&['3', '2', 'T', '3', 'K']);
/// assert_eq!(result2, 2);
/// let result3 = get_wild_hand_rank(&['2', 'J', '2', 'J', '2']);
/// assert_eq!(result3, 7);
/// ```
pub fn get_wild_hand_rank(hand: &[char; 5]) -> u8 {
    let cards: Vec<char> = vec![
        'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
    ];
    let mut current_hand = hand.clone();
    let a_range = if hand[0] == 'J' {
        &cards
    } else {
        &vec![hand[0]]
    };
    let b_range = if hand[1] == 'J' {
        &cards
    } else {
        &vec![hand[1]]
    };
    let c_range = if hand[2] == 'J' {
        &cards
    } else {
        &vec![hand[2]]
    };
    let d_range = if hand[3] == 'J' {
        &cards
    } else {
        &vec![hand[3]]
    };
    let e_range = if hand[4] == 'J' {
        &cards
    } else {
        &vec![hand[4]]
    };

    let mut max_rank: u8 = 0;
    for a in a_range {
        for b in b_range {
            for c in c_range {
                for d in d_range {
                    for e in e_range {
                        current_hand[0] = *a;
                        current_hand[1] = *b;
                        current_hand[2] = *c;
                        current_hand[3] = *d;
                        current_hand[4] = *e;
                        max_rank = max_rank.max(get_hand_rank(&current_hand));
                    }
                }
            }
        }
    }

    return max_rank;
}

/// Compare two hands together first check if one has greater rank
/// If ranks are the same, compare card by card
/// ```
/// use std::cmp::Ordering;
/// use crate::advent_code_wasm::modules::day7::compare_hands;
/// use crate::advent_code_wasm::modules::day7::Game;
/// let result1 = compare_hands::<false>(&Game(['A', '2', '2', '2', '2'], 0, 0), &Game(['A', 'A', 'A', 'A', 'A'], 0, 0));
/// assert_eq!(result1, Ordering::Less);
/// let result2 = compare_hands::<false>(&Game(['3', '3', '2', '2', '2'], 0, 0), &Game(['3', '2', 'A', 'A', 'A'], 0, 0));
/// assert_eq!(result2, Ordering::Greater);
/// let result3 = compare_hands::<false>(&Game(['A', 'A', 'A', 'A', 'A'], 0, 0), &Game(['A', 'A', 'A', 'A', 'A'], 0, 0));
/// assert_eq!(result3, Ordering::Equal);
/// ```
pub fn compare_hands<const WILD: bool>(game1: &Game, game2: &Game) -> std::cmp::Ordering {
    return match game1.1.cmp(&game2.1) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => compare_cards_by_value::<WILD>(&game1.0, &game2.0),
        Ordering::Greater => Ordering::Greater,
    };
}

/// Turn string describing hands into arrays of chars for hands and
/// The rank is parsed as a u8
/// the bids read as i16 integers
/// ```
/// use crate::advent_code_wasm::modules::day7::parse_card_hands;
/// use crate::advent_code_wasm::modules::day7::Game;
/// let result1 = parse_card_hands::<false>("32T3K 765");
/// assert_eq!(result1[0].0, ['3', '2', 'T', '3', 'K']);
/// assert_eq!(result1[0].1, 2u8);
/// assert_eq!(result1[0].2, 765i16);
/// let result2 = parse_card_hands::<true>("32J3K 123");
/// assert_eq!(result2[0].0, ['3', '2', 'J', '3', 'K']);
/// assert_eq!(result2[0].1, 4u8);
/// assert_eq!(result2[0].2, 123i16);
/// ```
pub fn parse_card_hands<const WILD: bool>(payload: &str) -> Vec<Game> {
    let mut hands_bids: Vec<Game> = vec![];

    let mut reading_hand: bool = true;
    let mut current_hand: [char; 5] = ['2', '2', '2', '2', '2'];
    let mut current_bid: i16 = 0;
    let mut i: usize = 0;
    for c in payload.chars() {
        if c == ' ' {
            reading_hand = false;
        } else if c == '\n' {
            reading_hand = true;
            let rank = if WILD {
                get_wild_hand_rank(&current_hand)
            } else {
                get_hand_rank(&current_hand)
            };
            hands_bids.push(Game(current_hand, rank, current_bid));
            current_bid = 0;
            i = 0;
        } else if c == '\r' {
            continue;
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
        let rank = if WILD {
            get_wild_hand_rank(&current_hand)
        } else {
            get_hand_rank(&current_hand)
        };
        hands_bids.push(Game(current_hand, rank, current_bid));
    }

    return hands_bids;
}

/// Returns bid multiplied by its rank
pub fn multiply_bids_and_order<const WILD: bool>(payload: &str) -> i64 {
    let mut games: Vec<Game> = parse_card_hands::<WILD>(payload);

    games.sort_by(|a, b| compare_hands::<WILD>(&a, &b));

    let mut output: i64 = 0;
    let mut i: i64 = 0;
    for game in games.iter() {
        i += 1;
        output += i * game.2 as i64;
    }

    return output;
}
