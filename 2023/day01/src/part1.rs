pub fn solve(input: &str) -> String {
    let result = input
        .lines()
        .map(combine_first_and_last_number)
        .sum::<u32>();

    result.to_string()
}

fn combine_first_and_last_number(input: &str) -> u32 {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for c in input.chars() {
        if !c.is_ascii_digit() {
            continue;
        }

        let digit = c.to_digit(10).unwrap();

        if first.is_none() {
            first = Some(digit);
        }

        last = Some(digit);
    }

    first.unwrap() * 10 + last.unwrap()
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
