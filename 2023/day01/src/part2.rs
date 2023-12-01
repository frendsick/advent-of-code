static SPELLED_NUMBERS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn solve(input: &str) -> String {
    let result = input
        .lines()
        .map(combine_first_and_last_number)
        .sum::<u32>();

    result.to_string()
}

fn combine_first_and_last_number(input: &str) -> u32 {
    let mut numbers = (0..input.len()).filter_map(|i| {
        let remaining_line = &input[i..];
        for (key, value) in SPELLED_NUMBERS {
            if remaining_line.starts_with(key) {
                return Some(value.to_owned());
            }
        }
        remaining_line.chars().next().unwrap().to_digit(10)
    });

    let first = numbers.next().unwrap();
    match numbers.last() {
        Some(number) => first * 10 + number,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let solution = "281";
        assert_eq!(solution, solve(input));
    }
}
