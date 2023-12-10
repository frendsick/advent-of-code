use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct PokerPlay<'a> {
    cards: &'a str,
    card_values: Vec<u8>,
    hand: PokerHand,
    bid: u32,
}

impl PartialOrd for PokerPlay<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PokerPlay<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare PokerHand values
        let hand_ordering = self.hand.cmp(&other.hand);
        if hand_ordering != Ordering::Equal {
            return hand_ordering;
        }

        // If PokerHand values are equal, compare card values
        for (self_val, other_val) in self.card_values.iter().zip(&other.card_values) {
            let card_ordering = self_val.cmp(other_val);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }
        Ordering::Equal
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum PokerHand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

lazy_static! {
    static ref LETTER_CARDS: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('A', 14);
        m.insert('K', 13);
        m.insert('Q', 12);
        m.insert('J', 11);
        m.insert('T', 10);
        m
    };
}

pub fn solve(input: &str) -> String {
    // Parse PokenPlays from the input
    let mut plays: Vec<PokerPlay> = input.lines().map(parse_poker_play).collect();

    // Sort the plays from lowest to highest rank
    plays.sort();

    // Calculate the winnings
    plays
        .iter()
        .enumerate()
        .fold(0, |acc, (index, play)| acc + play.bid * (index as u32 + 1))
        .to_string()
}

fn parse_poker_play(line: &str) -> PokerPlay {
    let parts = line.split_once(' ').expect("space divides line parts");
    let cards = parts.0;
    let card_values: Vec<u8> = cards.chars().map(get_card_value).collect();
    let hand = get_poker_hand(&card_values);
    let bid: u32 = parts.1.parse().expect("bid");

    PokerPlay {
        cards,
        card_values,
        hand,
        bid,
    }
}

fn get_card_value(card: char) -> u8 {
    if card.is_ascii_digit() {
        card.to_digit(10).expect("digit") as u8
    } else {
        LETTER_CARDS.get(&card).expect("card value").to_owned()
    }
}

fn get_poker_hand(values: &[u8]) -> PokerHand {
    if is_n_of_a_kind(values, 5) {
        PokerHand::FiveOfAKind
    } else if is_n_of_a_kind(values, 4) {
        PokerHand::FourOfAKind
    } else if is_full_house(values) {
        PokerHand::FullHouse
    } else if is_n_of_a_kind(values, 3) {
        PokerHand::ThreeOfAKind
    } else if is_two_pair(values) {
        PokerHand::TwoPair
    } else if is_n_of_a_kind(values, 2) {
        PokerHand::Pair
    } else {
        PokerHand::HighCard
    }
}

fn is_n_of_a_kind(values: &[u8], n: u8) -> bool {
    let most_common = mode(values).expect("mode");
    let count = values.iter().filter(|&&x| x == most_common).count();
    count >= n.into()
}

fn is_full_house(values: &[u8]) -> bool {
    // Check that there are only two kinds of values in the array
    let set: HashSet<u8> = values.iter().cloned().collect();
    if set.len() != 2 {
        return false;
    }

    // Check that there are 2 or 3 of the first value in the array
    // This check is redundant if FourOfAKind is already checked
    let first_value_count = values.iter().filter(|&value| *value == values[0]).count();
    first_value_count == 2 || first_value_count == 3
}

fn is_two_pair(values: &[u8]) -> bool {
    let set: HashSet<u8> = values.iter().cloned().collect();
    set.len() == 3
}

fn mode(numbers: &[u8]) -> Option<u8> {
    let mut counts = HashMap::new();

    numbers.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let solution = "6440";
        assert_eq!(solution, solve(input));
    }
}
