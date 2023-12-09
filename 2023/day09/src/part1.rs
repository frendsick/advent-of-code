pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(parse_numbers_separated_by_spaces)
        .map(|mut numbers| calculate_extrapolated_number(&mut numbers))
        .sum::<i64>()
        .to_string()
}

fn calculate_extrapolated_number(numbers: &mut Vec<i64>) -> i64 {
    extrapolate_end_numbers(numbers, Vec::new()).iter().sum()
}

fn extrapolate_end_numbers(numbers: &[i64], mut end_numbers: Vec<i64>) -> Vec<i64> {
    if numbers.iter().all(|&num| num == 0) {
        return end_numbers;
    }

    // Calculate differences of each number
    let differences: Vec<i64> = numbers
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    // Save the last number to `end_numbers`
    end_numbers.push(numbers.last().copied().unwrap_or_default());

    // Extrapolate the end numbers for following difference rows recursively
    extrapolate_end_numbers(&differences, end_numbers)
}

fn parse_numbers_separated_by_spaces(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .flat_map(str::parse::<i64>)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let solution = "114";
        assert_eq!(solution, solve(input));
    }
}
