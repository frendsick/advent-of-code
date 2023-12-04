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

fn find_touching_numbers(row: usize, col: usize, numbers: &[Positioned]) -> Vec<u32> {
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
    let mut touching_numbers: Vec<u32> = neighbors
        .iter()
        .filter_map(|&(maybe_row, maybe_col)| {
            maybe_row.and_then(|row| {
                maybe_col.and_then(|col| get_number_from_position(row, col, numbers))
            })
        })
        .collect();

    // Remove duplicates and return the found touching numbers
    touching_numbers.dedup();
    touching_numbers
}

fn get_number_from_position(row: usize, col: usize, numbers: &[Positioned]) -> Option<u32> {
    for number in numbers {
        // We are not in the right row yet
        if number.row < row {
            continue;
        }

        // The number was not found if we went past the possible numbers
        if number.row > row {
            return None;
        }

        // Check if the number overlaps with the position
        let earliest_index = col.saturating_sub(number.value.len() - 1);
        if number.col >= earliest_index && number.col <= col {
            return Some(number.value.parse().expect("number"));
        }
    }
    // No match
    None
}

// Helper function to find the position (row, col) of a character index in a multiline string
fn find_position(input: &str, index: usize) -> (usize, usize) {
    let mut row = 0;
    let mut col = 0;

    for (i, c) in input.chars().enumerate() {
        if i == index {
            break;
        }
        if c == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    (row, col)
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
