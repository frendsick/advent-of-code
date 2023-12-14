const HORIZONTAL_REFLECTION_MULTIPLIER: usize = 100;

pub fn solve(input: &str) -> String {
    let patterns: Vec<&str> = input.split("\n\n").collect();
    let matrices: Vec<CharMatrix> = patterns.iter().map(parse_char_matrix).collect();
    matrices
        .iter()
        .map(find_reflection_line_from_altered_matrix)
        .fold(0, |acc, line| match line {
            ReflectionLine::Horizontal(row) => acc + row * HORIZONTAL_REFLECTION_MULTIPLIER,
            ReflectionLine::Vertical(column) => acc + column,
        })
        .to_string()
}

type CharMatrix = Vec<Vec<char>>;

#[derive(Debug, PartialEq)]
enum ReflectionLine {
    Horizontal(usize), // row
    Vertical(usize),   // column
}

fn parse_char_matrix(input: &&str) -> CharMatrix {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn find_reflection_line_from_altered_matrix(matrix: &CharMatrix) -> ReflectionLine {
    // Hacky way to create forbidden line that is impossible for the puzzle input
    let nonexistent_line = &ReflectionLine::Horizontal(13333337);
    let old_reflection_line =
        find_reflection_line(matrix, &nonexistent_line).expect("Reflection line");

    // Brute-force solution:
    // Swap one character at a time and return the first found reflection line which is not same as the original
    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let mut matrix_clone = matrix.clone();
            match matrix[y][x] {
                '#' => matrix_clone[y][x] = '.',
                '.' => matrix_clone[y][x] = '#',
                _ => unreachable!("The puzzle input consists of '#' and '.' characters"),
            }
            if let Some(reflection_line) = find_reflection_line(&matrix_clone, &old_reflection_line)
            {
                return reflection_line;
            }
        }
    }
    unreachable!("Puzzle should contain altered matrix containing valid reflection line")
}

fn find_reflection_line(matrix: &CharMatrix, forbidden: &ReflectionLine) -> Option<ReflectionLine> {
    find_horizontal_reflection_line(matrix, forbidden)
        .or(find_vertical_reflection_line(matrix, forbidden))
}

fn find_horizontal_reflection_line(
    matrix: &CharMatrix,
    forbidden: &ReflectionLine,
) -> Option<ReflectionLine> {
    matrix.windows(2).enumerate().find_map(|(index, rows)| {
        let reflection_line = ReflectionLine::Horizontal(index + 1);
        if reflection_line != *forbidden
            && rows[0] == rows[1]
            && is_horizontal_reflection_line(index, matrix)
        {
            Some(reflection_line)
        } else {
            None
        }
    })
}

fn find_vertical_reflection_line(
    matrix: &CharMatrix,
    forbidden: &ReflectionLine,
) -> Option<ReflectionLine> {
    let matrix_width: usize = matrix[0].len();
    (0..matrix_width - 1).find_map(|left_ptr| {
        let right_ptr = left_ptr + 1;
        let reflection_line = ReflectionLine::Vertical(left_ptr + 1);
        if reflection_line != *forbidden
            && vertical_lines_match(left_ptr, right_ptr, matrix)
            && is_vertical_reflection_line(left_ptr, matrix)
        {
            Some(reflection_line)
        } else {
            None
        }
    })
}

fn is_horizontal_reflection_line(row: usize, matrix: &CharMatrix) -> bool {
    let matrix_height = matrix.len();
    (0..=row)
        .rev()
        .zip(row + 1..matrix_height)
        .all(|(up, down)| matrix[up] == matrix[down])
}

fn is_vertical_reflection_line(column: usize, matrix: &CharMatrix) -> bool {
    let matrix_width = matrix[0].len();
    (0..=column)
        .rev()
        .zip(column + 1..matrix_width)
        .all(|(left_ptr, right_ptr)| vertical_lines_match(left_ptr, right_ptr, matrix))
}

fn vertical_lines_match(left_ptr: usize, right_ptr: usize, matrix: &CharMatrix) -> bool {
    matrix.iter().all(|row| row[left_ptr] == row[right_ptr])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let solution = "400";
        assert_eq!(solution, solve(input));
    }
}
