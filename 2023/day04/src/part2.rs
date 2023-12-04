pub fn solve(input: &str) -> String {
    let lines = input.lines();
    let line_count: usize = lines.clone().count();
    let mut scratchcards: Vec<u32> = vec![1; line_count];

    // Collect scratchcards and count their amount
    lines
        .into_iter()
        .enumerate()
        .for_each(|(i, line)| collect_scratchcards_for_winning_numbers(line, i, &mut scratchcards));
    scratchcards.iter().sum::<u32>().to_string()
}

fn collect_scratchcards_for_winning_numbers(
    line: &str,
    card_index: usize,
    scratchcards: &mut Vec<u32>,
) {
    let current_card_copies = scratchcards[card_index];
    let winning_numbers = calculate_winning_numbers(line);

    // Get `current_card_copies` amount of `winning_numbers` next cards
    scratchcards
        .iter_mut()
        .skip(card_index + 1)
        .take(winning_numbers as usize)
        .for_each(|card_count| *card_count += current_card_copies);
}

fn calculate_winning_numbers(input: &str) -> u32 {
    // Locate used separators in the line
    let my_numbers_ptr = input.find(':').expect("colon index") + 1;
    let vertical_bar_ptr = input.find('|').expect("vertical bar");

    // Parse numbers from the schratchcard sections
    let my_numbers = parse_numbers_separated_by_spaces(&input[my_numbers_ptr..vertical_bar_ptr]);
    let winning_numbers = parse_numbers_separated_by_spaces(&input[vertical_bar_ptr + 1..]);

    // Calculate how many numbers were in my numbers compared to the winning numbers
    my_numbers
        .iter()
        .filter(|num| winning_numbers.contains(num))
        .count() as u32
}

fn parse_numbers_separated_by_spaces(input: &str) -> Vec<u32> {
    // Expects string containing only numbers and whitespaces
    input
        .split(char::is_whitespace)
        .filter_map(|s| match s.parse::<u32>() {
            Result::Ok(digit) => Some(digit),
            Result::Err(_) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let solution = "30";
        assert_eq!(solution, solve(input));
    }
}
