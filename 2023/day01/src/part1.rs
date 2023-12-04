pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(combine_first_and_last_number)
        .sum::<u32>()
        .to_string()
}

fn combine_first_and_last_number(input: &str) -> u32 {
    let digits: Vec<u32> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    digits[0] * 10 + digits.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let solution = "142";
        assert_eq!(solution, solve(input));
    }
}
