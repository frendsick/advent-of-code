use std::collections::HashSet;
use regex::Regex;

#[derive(Debug, Clone)]
struct Positioned {
    row: usize,
    col: usize,
    value: String,
}

pub fn solve(input: &str) -> String {
    let numbers: Vec<Positioned> = find_by_pattern(input, r"\d+");
    let asterisks: Vec<Positioned> = find_by_pattern(input, r"\*");

    asterisks
        .iter()
        .filter_map(|asterisk| product_of_two_gears(asterisk, &numbers))
        .sum::<u32>()
        .to_string()
}

fn find_by_pattern(input: &str, pattern: &str) -> Vec<Positioned> {
    let number_pattern = Regex::new(pattern).expect("valid pattern");
    let mut numbers = Vec::new();
    for mat in number_pattern.captures_iter(input) {
        let value: String = mat[0].to_string();
        let (row, col) = find_position(input, mat.get(0).expect("match").start());
        numbers.push(Positioned { row, col, value });
    }
    numbers
}

fn product_of_two_gears(asterisk: &Positioned, numbers: &[Positioned]) -> Option<u32> {
    let touching_numbers = find_touching_numbers(asterisk.row, asterisk.col, numbers);

    // Only calculate product if the asterisk is touched by 2 numbers
    if touching_numbers.len() == 2 {
        return Some(touching_numbers.iter().product::<u32>());
    }
    None
}

fn find_touching_numbers(row: usize, col: usize, numbers: &[Positioned]) -> HashSet<u32> {
    let neighbors = [
        (row.checked_sub(1), Some(col)),          // Up
        (row.checked_sub(1), col.checked_sub(1)), // Up-Left
        (row.checked_sub(1), col.checked_add(1)), // Up-Right
        (row.checked_add(1), Some(col)),          // Down
        (row.checked_add(1), col.checked_sub(1)), // Down-Left
        (row.checked_add(1), col.checked_add(1)), // Down-Right
        (Some(row), col.checked_sub(1)),          // Left
        (Some(row), col.checked_add(1)),          // Right
    ];

    // Filter all touching numbers
    neighbors
        .iter()
        .filter_map(|&(maybe_row, maybe_col)| {
            match (maybe_row, maybe_col) {
                (Some(row), Some(col)) => get_number_from_position(row, col, numbers),
                _ => None,
            }
        })
        .collect()
}

fn get_number_from_position(row: usize, col: usize, numbers: &[Positioned]) -> Option<u32> {
    numbers
        .iter()
        .find(|number| {
            // Find the possible number at the given position
            row == number.row && col >= number.col && col < number.col + number.value.len()
        })
        .and_then(|number| number.value.parse::<u32>().ok())
}

// Helper function to find the position (row, col) of a character index in a multiline string
fn find_position(input: &str, index: usize) -> (usize, usize) {
    input
        .chars()
        .take(index)
        .fold((0, 0), |(row, col), c| {
            if c == '\n' {
                (row + 1, 0)
            } else {
                (row, col + 1)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let solution = "467835";
        assert_eq!(solution, solve(input));
    }
}
