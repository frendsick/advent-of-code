const CYCLES: usize = 1_000_000_000;

pub fn solve(input: &str) -> String {
    let mut matrix: TileMatrix = input.lines().map(parse_tiles_from_line).collect();

    // Cycling the matrix enough times should result in a loop
    // Find the loop and gather information for it to calculate what matrix would be the `CYCLES`th
    let matrices: Vec<TileMatrix> = cycle_matrix_until_loop_is_found(&mut matrix, CYCLES);
    let (loop_index, loop_length) = get_loop_info(matrices.clone());

    // Calculate the load for the matrix that would be the `CYCLES`th if we continued cycling
    let final_index: usize = (CYCLES - loop_index) % loop_length + loop_index;
    let final_matrix: &TileMatrix = &matrices[final_index];
    calculate_load(final_matrix).to_string()
}

type TileMatrix = Vec<Vec<Tile>>;
type Coordinate = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Empty,
    CubeRock,
    RoundRock,
}

enum Direction {
    North,
    South,
    East,
    West,
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

fn get_loop_info(matrices: Vec<TileMatrix>) -> (usize, usize) {
    // The last matrix in `matrices` is the first matrix contained twice
    // We can derive the loop index and length by finding its pair
    let last_matrix = matrices.last().unwrap();
    let matrices_len = matrices.len();

    for (i, matrix) in matrices.iter().enumerate().take(matrices_len - 1) {
        if matrix == last_matrix {
            return (i, matrices_len - i - 1);
        }
    }
    unreachable!();
}

fn cycle_matrix_until_loop_is_found(matrix: &mut TileMatrix, cycles: usize) -> Vec<TileMatrix> {
    let mut matrices: Vec<TileMatrix> = vec![matrix.clone()];
    for _ in 0..cycles {
        tilt_lever(matrix, Direction::North);
        tilt_lever(matrix, Direction::West);
        tilt_lever(matrix, Direction::South);
        tilt_lever(matrix, Direction::East);
        if matrices.contains(matrix) {
            matrices.push(matrix.clone());
            return matrices;
        }
        matrices.push(matrix.clone());
    }
    unreachable!("Puzzle input should result in a loop");
}

fn tilt_lever(matrix: &mut TileMatrix, direction: Direction) {
    for (y, row) in matrix.clone().iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::RoundRock {
                match direction {
                    Direction::North => slide_north(matrix, (x, y)),
                    Direction::South => slide_south(matrix, (x, y)),
                    Direction::East => slide_east(matrix, (x, y)),
                    Direction::West => slide_west(matrix, (x, y)),
                }
            }
        }
    }
}

fn slide_north(matrix: &mut TileMatrix, coordinate: Coordinate) {
    let x = coordinate.0;
    let mut target_coordinate = coordinate;
    let mut round_rocks: Vec<Coordinate> = Vec::new();
    for y in (0..coordinate.1).rev() {
        match matrix[y][x] {
            Tile::Empty => target_coordinate = (x, y),
            Tile::CubeRock => {
                target_coordinate = (x, y + 1);
                while coordinate != target_coordinate && round_rocks.contains(&target_coordinate) {
                    target_coordinate.1 += 1
                }
                break;
            }
            Tile::RoundRock => round_rocks.push((x, y)),
        }
    }
    move_tile(matrix, coordinate, target_coordinate)
}

fn slide_south(matrix: &mut TileMatrix, coordinate: Coordinate) {
    let x = coordinate.0;
    let matrix_height = matrix.len();
    let mut round_rocks: Vec<Coordinate> = Vec::new();
    let mut target_coordinate = coordinate;
    for y in coordinate.1..matrix_height {
        match matrix[y][x] {
            Tile::Empty => target_coordinate = (x, y),
            Tile::CubeRock => {
                target_coordinate = (x, y - 1);
                while coordinate != target_coordinate && round_rocks.contains(&target_coordinate) {
                    target_coordinate.1 -= 1
                }
                break;
            }
            Tile::RoundRock => round_rocks.push((x, y)),
        }
    }
    move_tile(matrix, coordinate, target_coordinate)
}

fn slide_west(matrix: &mut TileMatrix, coordinate: Coordinate) {
    let y = coordinate.1;
    let mut target_coordinate = coordinate;
    let mut round_rocks: Vec<Coordinate> = Vec::new();
    for x in (0..coordinate.0).rev() {
        match matrix[y][x] {
            Tile::Empty => target_coordinate = (x, y),
            Tile::CubeRock => {
                target_coordinate = (x + 1, y);
                while coordinate != target_coordinate && round_rocks.contains(&target_coordinate) {
                    target_coordinate.0 += 1
                }
                break;
            }
            Tile::RoundRock => round_rocks.push((x, y)),
        }
    }
    move_tile(matrix, coordinate, target_coordinate)
}

fn slide_east(matrix: &mut TileMatrix, coordinate: Coordinate) {
    let y = coordinate.1;
    let matrix_width = matrix[0].len();
    let mut round_rocks: Vec<Coordinate> = Vec::new();
    let mut target_coordinate = coordinate;
    for x in coordinate.0..matrix_width {
        match matrix[y][x] {
            Tile::Empty => target_coordinate = (x, y),
            Tile::CubeRock => {
                target_coordinate = (x - 1, y);
                while coordinate != target_coordinate && round_rocks.contains(&target_coordinate) {
                    target_coordinate.0 -= 1;
                }
                break;
            }
            Tile::RoundRock => round_rocks.push((x, y)),
        }
    }
    move_tile(matrix, coordinate, target_coordinate)
}

fn move_tile(matrix: &mut TileMatrix, source: Coordinate, target: Coordinate) {
    matrix[source.1][source.0] = Tile::Empty;
    matrix[target.1][target.0] = Tile::RoundRock;
}

fn calculate_load(matrix: &TileMatrix) -> usize {
    let matrix_height: usize = matrix[0].len();
    let mut total_load = 0;
    for (y, row) in matrix.iter().enumerate() {
        for tile in row.iter() {
            if tile == &Tile::RoundRock {
                total_load += matrix_height - y
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
        let solution = "64";
        assert_eq!(solution, solve(input));
    }
}
