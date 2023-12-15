const MULTIPLY_CONSTANT: u32 = 17;

pub fn solve(input: &str) -> String {
    input
        .split(',')
        .map(|step| {
            step.chars()
                .filter(|c| *c != '\n') // Ignore newline characters
                .fold(0, |current_value, character| {
                    (current_value + character as u32) * MULTIPLY_CONSTANT % 256
                })
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let solution = "1320";
        assert_eq!(solution, solve(input));
    }
}
