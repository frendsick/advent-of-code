pub fn solve(input: &str) -> String {
    let matrix: TileMatrix = input.lines().map(parse_tiles_from_line).collect();
    calculate_load_after_tilt(&matrix).to_string()
}

type TileMatrix = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    CubeRock,
    RoundRock,
}

fn parse_tiles_from_line(line: &str) -> Vec<Tile> {
    line.chars()
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::CubeRock,
            'O' => Tile::RoundRock,
            _ => unreachable!("The puzzle input only contains '.', '#', and 'O' characters"),
        })
        .collect()
}

fn calculate_load_after_tilt(matrix: &TileMatrix) -> usize {
    let matrix_height: usize = matrix[0].len();
    let mut total_load = 0;
    for x in 0..matrix_height {
        let mut rock_weight = matrix_height;
        for (y, row) in matrix.iter().enumerate() {
            match row[x] {
                Tile::RoundRock => {
                    total_load += rock_weight;
                    rock_weight -= 1;
                }
                Tile::CubeRock => rock_weight = matrix_height - y - 1,
                Tile::Empty => {}
            }
        }
    }
    total_load
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let solution = "136";
        assert_eq!(solution, solve(input));
    }
}
