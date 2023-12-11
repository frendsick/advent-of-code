use std::cmp::{max, min};

const GALAXY_EXPANSION_RATE: u64 = 1000000;

pub fn solve(input: &str) -> String {
    let matrix: Vec<Vec<Tile>> = input.lines().map(parse_tiles).collect();
    let expanded_matrix: Vec<Vec<Tile>> = expand_empty_rows_and_columns(&matrix);
    let galaxy_coordinates: Vec<Coordinate> = find_galaxy_coordinates(&expanded_matrix);

    // Calculate distances between the pairs of galaxies
    galaxy_coordinates
        .iter()
        .enumerate()
        .map(|(index, coordinate)| {
            find_distances_to_pairs(&galaxy_coordinates, coordinate, index, &expanded_matrix)
                .iter()
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Expanded,
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
    matrix: &[Vec<Tile>],
) -> Vec<u64> {
    galaxy_coordinates
        .iter()
        .skip(galaxy_index + 1) // The distance to previous galaxies are already calculated
        .map(|target| find_distance_between_galaxies(source, target, matrix))
        .collect::<Vec<u64>>()
}

fn find_distance_between_galaxies(
    source: &Coordinate,
    target: &Coordinate,
    matrix: &[Vec<Tile>],
) -> u64 {
    let (min_x, max_x) = (min(source.x, target.x), max(source.x, target.x));
    let (min_y, max_y) = (min(source.y, target.y), max(source.y, target.y));

    let vertical_distance = (min_y..max_y)
        .map(|y| {
            if matrix[y][min_x] == Tile::Expanded {
                GALAXY_EXPANSION_RATE
            } else {
                1
            }
        })
        .sum::<u64>();

    let horizontal_distance = (min_x..max_x)
        .map(|x| {
            if matrix[min_y][x] == Tile::Expanded {
                GALAXY_EXPANSION_RATE
            } else {
                1
            }
        })
        .sum::<u64>();

    vertical_distance + horizontal_distance
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
    let mut expanded_matrix: Vec<Vec<Tile>> = matrix.to_owned();

    // Mark empty rows as Expanded
    for y in empty_rows {
        for x in 0..matrix[0].len() {
            expanded_matrix[y][x] = Tile::Expanded;
        }
    }

    // Mark empty columns as Expanded
    expanded_matrix
        .into_iter()
        .map(|mut row| {
            empty_columns.iter().for_each(|x| {
                row[*x] = Tile::Expanded;
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
                    Tile::Galaxy => Some(Coordinate { x, y }),
                    Tile::Empty | Tile::Expanded => None,
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
        let solution = "82000210";
        assert_eq!(solution, solve(input));
    }
}
