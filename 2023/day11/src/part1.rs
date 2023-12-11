pub fn solve(input: &str) -> String {
    let matrix: Vec<Vec<Tile>> = input.lines().map(parse_tiles).collect();
    let expanded_matrix: Vec<Vec<Tile>> = expand_empty_rows_and_columns(&matrix);
    let galaxy_coordinates: Vec<Coordinate> = find_galaxy_coordinates(&expanded_matrix);

    // Calculate distances between the pairs of galaxies
    galaxy_coordinates
        .iter()
        .enumerate()
        .map(|(index, coordinate)| {
            find_distances_to_pairs(&galaxy_coordinates, coordinate, index)
                .iter()
                .sum::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Galaxy,
}

#[derive(PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn find_distances_to_pairs(
    galaxy_coordinates: &[Coordinate],
    source: &Coordinate,
    galaxy_index: usize,
) -> Vec<u32> {
    galaxy_coordinates
        .iter()
        .skip(galaxy_index + 1) // The distance to previous galaxies are already calculated
        .map(|target| (source.x.abs_diff(target.x) + source.y.abs_diff(target.y)) as u32)
        .collect::<Vec<u32>>()
}

fn expand_empty_rows_and_columns(matrix: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let empty_rows: Vec<usize> = find_empty_rows(matrix);
    let empty_columns: Vec<usize> = find_empty_columns(matrix);
    expand_matrix(matrix, empty_rows, empty_columns)
}

fn expand_matrix(
    matrix: &[Vec<Tile>],
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
) -> Vec<Vec<Tile>> {
    // Expand empty rows to two
    let expanded_matrix = empty_rows
        .into_iter()
        .rev()
        .fold(matrix.to_owned(), |mut acc, row| {
            acc.insert(row, matrix[row].clone());
            acc
        });

    // Expand empty columns to two
    expanded_matrix
        .into_iter()
        .map(|mut row| {
            empty_columns.iter().rev().for_each(|col| {
                row.insert(*col, Tile::Empty);
            });
            row
        })
        .collect()
}

fn find_empty_rows(matrix: &[Vec<Tile>]) -> Vec<usize> {
    matrix
        .iter()
        .enumerate()
        .filter_map(|(y, row)| if galaxy_in_tiles(row) { None } else { Some(y) })
        .collect()
}

fn find_empty_columns(matrix: &[Vec<Tile>]) -> Vec<usize> {
    (0..matrix[0].len())
        .filter(|&x| matrix.iter().all(|row| row[x] == Tile::Empty))
        .collect()
}

fn galaxy_in_tiles(tiles: &[Tile]) -> bool {
    tiles.iter().any(|tile| *tile == Tile::Galaxy)
}

fn find_galaxy_coordinates(matrix: &[Vec<Tile>]) -> Vec<Coordinate> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| match tile {
                    Tile::Empty => None,
                    Tile::Galaxy => Some(Coordinate { x, y }),
                })
        })
        .collect()
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .chars()
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy,
            _ => unreachable!("input should only contain galaxy tiles and empty tiles"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let solution = "374";
        assert_eq!(solution, solve(input));
    }
}
