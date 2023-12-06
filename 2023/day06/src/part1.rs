pub fn solve(input: &str) -> String {
    // Parse numbers from each line (Time and Distance)
    let numbers: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            let number_section = line.split(':').last().unwrap();
            parse_numbers_separated_by_spaces(number_section)
        })
        .collect();

    assert!(numbers.len() == 2, "Expected two lines with numbers");
    let target_times = &numbers[0];
    let distances = &numbers[1];

    // Calculate the product of possible ways to win each competition
    target_times
        .iter()
        .zip(distances)
        .map(|(&time, &distance)| calculate_possible_ways_to_win(time, distance))
        .product::<u32>()
        .to_string()
}

fn calculate_possible_ways_to_win(time: u32, target_distance: u32) -> u32 {
    (1..=time)
        .filter(|&speed| speed * (time - speed) > target_distance)
        .count() as u32
}

fn parse_numbers_separated_by_spaces(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .flat_map(str::parse::<u32>)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let solution = "288";
        assert_eq!(solution, solve(input));
    }
}
