#[derive(PartialEq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    Ground,
    Start,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &str) -> String {
    let matrix: Vec<Vec<Tile>> = input.lines().map(parse_tiles).collect();
    let visited: Vec<(usize, usize)> = get_loop_coordinates(&matrix);
    calculate_tiles_within_loop(matrix, visited).to_string()
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .chars()
        .map(|c| match c {
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::BendNE,
            'J' => Tile::BendNW,
            'F' => Tile::BendSE,
            '7' => Tile::BendSW,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!(),
        })
        .collect()
}

fn calculate_tiles_within_loop(matrix: Vec<Vec<Tile>>, visited: Vec<(usize, usize)>) -> u32 {
    let mut in_loop = false;
    let mut tiles_within_loop: u32 = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if !visited.contains(&(y, x)) {
                if in_loop {
                    tiles_within_loop += 1;
                }
                continue;
            }
            match tile {
                Tile::VerticalPipe => in_loop = !in_loop,
                Tile::Start => in_loop = !in_loop,
                Tile::BendSE => in_loop = !in_loop,
                Tile::BendSW => in_loop = !in_loop,
                _ => {}
            }
        }
    }
    tiles_within_loop
}

fn find_start_coordinate(matrix: &[Vec<Tile>]) -> (usize, usize) {
    if let Some(y) = matrix.iter().position(|row| row.contains(&Tile::Start)) {
        if let Some(x) = matrix[y].iter().position(|tile| tile == &Tile::Start) {
            return (y, x);
        }
    }
    unreachable!("Matrix should contain Start tile")
}

fn get_loop_coordinates(matrix: &[Vec<Tile>]) -> Vec<(usize, usize)> {
    let start_coordinate = find_start_coordinate(matrix);

    // Start by going down
    let mut current_coordinate = (start_coordinate.0 + 1, start_coordinate.1);
    let mut direction = Direction::Down;
    let mut visited: Vec<(usize, usize)> = Vec::new();

    // Go through the loop until we get back to the Start tile
    loop {
        visited.push(current_coordinate);
        let (y, x) = current_coordinate;
        match matrix[y][x] {
            Tile::VerticalPipe => move_vertical(&mut current_coordinate, &mut direction),
            Tile::HorizontalPipe => move_horizontal(&mut current_coordinate, &mut direction),
            Tile::BendNE => move_bend_ne(&mut current_coordinate, &mut direction),
            Tile::BendNW => move_bend_nw(&mut current_coordinate, &mut direction),
            Tile::BendSE => move_bend_se(&mut current_coordinate, &mut direction),
            Tile::BendSW => move_bend_sw(&mut current_coordinate, &mut direction),
            Tile::Ground => unreachable!("Pipe cannot go through Ground tile"),
            Tile::Start => break,
        }
    }

    // The farthest point in the loop is in the middle
    visited
}

fn move_vertical(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Up => current_coordinate.0 -= 1,
        Direction::Down => current_coordinate.0 += 1,
        _ => unreachable!("vertical pipe while not going up or down"),
    }
}

fn move_horizontal(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Left => current_coordinate.1 -= 1,
        Direction::Right => current_coordinate.1 += 1,
        _ => unreachable!("horizontal pipe while not going left or right"),
    }
}

fn move_bend_ne(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Down => {
            *direction = Direction::Right;
            current_coordinate.1 += 1;
        }
        Direction::Left => {
            *direction = Direction::Up;
            current_coordinate.0 -= 1;
        }
        _ => unreachable!("NE bend while not going down or left"),
    }
}

fn move_bend_nw(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Down => {
            *direction = Direction::Left;
            current_coordinate.1 -= 1;
        }
        Direction::Right => {
            *direction = Direction::Up;
            current_coordinate.0 -= 1;
        }
        _ => unreachable!("NW bend while not going down or right"),
    }
}

fn move_bend_se(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Up => {
            *direction = Direction::Right;
            current_coordinate.1 += 1;
        }
        Direction::Left => {
            *direction = Direction::Down;
            current_coordinate.0 += 1;
        }
        _ => unreachable!("SE bend while not going up or left"),
    }
}

fn move_bend_sw(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Up => {
            *direction = Direction::Left;
            current_coordinate.1 -= 1;
        }
        Direction::Right => {
            *direction = Direction::Down;
            current_coordinate.0 += 1;
        }
        _ => unreachable!("NE bend while not going down or left"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let solution = "10";
        assert_eq!(solution, solve(input));
    }
}
