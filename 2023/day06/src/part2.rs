pub fn solve(input: &str) -> String {
    // Parse numbers from each line (Time and Distance)
    let numbers: Vec<u64> = input
        .lines()
        .map(|line| {
            let number_section = line.split(':').last().unwrap();
            parse_number_from_string_with_kerning(number_section)
        })
        .collect();

    assert!(numbers.len() == 2, "Expected two numbers");
    let time = &numbers[0];
    let target_distance = &numbers[1];

    calculate_possible_ways_to_win(*time, *target_distance).to_string()
}

fn calculate_possible_ways_to_win(time: u64, target_distance: u64) -> u64 {
    (1..=time)
        .filter(|&speed| speed * (time - speed) > target_distance)
        .count() as u64
}

fn parse_number_from_string_with_kerning(input: &str) -> u64 {
    remove_whitespace(input).parse().expect("number")
}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let solution = "71503";
        assert_eq!(solution, solve(input));
    }
}
