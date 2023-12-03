pub fn solve(input: &str) -> String {
    let mut total: u32 = 0;
    let mut current_number: u32 = 0;
    let mut adjacent_symbol_found: bool = false;

    // Parse all numbers from lines and sum the ones having adjacent symbols in the grid
    let lines: Vec<&str> = input.lines().map(|s| s.trim_end()).collect();
    for (row, line) in lines.clone().into_iter().enumerate() {
        for (col, character) in line.chars().enumerate() {
            // Append the found digit character to the number
            if character.is_ascii_digit() {
                current_number =
                    current_number * 10 + character.to_digit(10).expect("digit character");
                adjacent_symbol_found |= has_adjacent_symbol(&lines, row, col);
                continue;
            }
            process_parsed_number(&mut total, &mut current_number, &mut adjacent_symbol_found);
        }
        process_parsed_number(&mut total, &mut current_number, &mut adjacent_symbol_found);
    }
    total.to_string()
}

fn process_parsed_number(total: &mut u32, number: &mut u32, has_adjacent_symbol: &mut bool) {
    // Add current number to total if it has adjacent symbols
    if *has_adjacent_symbol {
        *total += *number;
    }

    // Reset the variables
    *number = 0;
    *has_adjacent_symbol = false;
}

fn has_adjacent_symbol(lines: &[&str], row: usize, col: usize) -> bool {
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

    // Filter all valid neighbor characters
    let mut adjacent_characters = neighbors.iter().filter_map(|&(maybe_row, maybe_col)| {
        maybe_row.and_then(|row| {
            maybe_col.and_then(
                // Parse the valid neighbor character
                |col| lines.get(row)?.chars().nth(col),
            )
        })
    });

    // Check if any of the adjacent characters is a symbol
    adjacent_characters.any(is_symbol)
}

fn is_symbol(c: char) -> bool {
    // Any character other than number or dot counts as a symbol in this puzzle
    !c.is_numeric() && c != '.'
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
        let solution = "4361";
        assert_eq!(solution, solve(input));
    }
}
