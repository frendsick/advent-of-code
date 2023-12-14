const HORIZONTAL_REFLECTION_MULTIPLIER: usize = 100;

pub fn solve(input: &str) -> String {
    let patterns: Vec<&str> = input.split("\n\n").collect();
    let matrises: Vec<CharMatrix> = patterns.iter().map(parse_char_matrix).collect();
    let reflection_lines: Vec<ReflectionLine> = matrises.iter().map(find_reflection_line).collect();
    reflection_lines
        .iter()
        .fold(0, |acc, line| match line {
            ReflectionLine::Horizontal(row) => acc + row * HORIZONTAL_REFLECTION_MULTIPLIER,
            ReflectionLine::Vertical(column) => acc + column,
        })
        .to_string()
}

type CharMatrix = Vec<Vec<char>>;

enum ReflectionLine {
    Horizontal(usize), // row
    Vertical(usize),   // column
}

fn parse_char_matrix(input: &&str) -> CharMatrix {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn find_reflection_line(matrix: &CharMatrix) -> ReflectionLine {
    find_horizontal_reflection_line(matrix)
        .or(find_vertical_reflection_line(matrix))
        .expect("Puzzle input matrix should have horizontal or vertical reflection line")
}

fn find_horizontal_reflection_line(matrix: &CharMatrix) -> Option<ReflectionLine> {
    matrix.windows(2).enumerate().find_map(|(index, rows)| {
        if rows[0] == rows[1] && is_horizontal_reflection_line(index, matrix) {
            Some(ReflectionLine::Horizontal(index + 1)) // 1-based rows
        } else {
            None
        }
    })
}

fn find_vertical_reflection_line(matrix: &CharMatrix) -> Option<ReflectionLine> {
    let matrix_width: usize = matrix[0].len();
    (0..matrix_width - 1).find_map(|left_ptr| {
        let right_ptr = left_ptr + 1;
        if vertical_lines_match(left_ptr, right_ptr, matrix)
            && is_vertical_reflection_line(left_ptr, matrix)
        {
            Some(ReflectionLine::Vertical(left_ptr + 1)) // 1-based columns
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
        let solution = "405";
        assert_eq!(solution, solve(input));
    }
}
