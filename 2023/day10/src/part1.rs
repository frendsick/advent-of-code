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
    steps_to_the_farthest_tile(matrix).to_string()
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

fn find_start_coordinate(matrix: &[Vec<Tile>]) -> (usize, usize) {
    if let Some(y) = matrix.iter().position(|row| row.contains(&Tile::Start)) {
        if let Some(x) = matrix[y].iter().position(|tile| tile == &Tile::Start) {
            return (y, x);
        }
    }
    unreachable!("Matrix should contain Start tile")
}

fn steps_to_the_farthest_tile(matrix: Vec<Vec<Tile>>) -> u32 {
    let start_coordinate = find_start_coordinate(&matrix);

    // Start by going down
    let mut current_coordinate = (start_coordinate.0 + 1, start_coordinate.1);
    let mut direction = Direction::Down;
    let mut steps: u32 = 1;

    // Go through the loop until we get back to the Start tile
    loop {
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
        steps += 1;
    }

    // The farthest point in the loop is in the middle
    steps / 2
}

fn move_up(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    *direction = Direction::Up;
    current_coordinate.0 -= 1;
}

fn move_down(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    *direction = Direction::Down;
    current_coordinate.0 += 1;
}

fn move_left(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    *direction = Direction::Left;
    current_coordinate.1 -= 1;
}

fn move_right(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    *direction = Direction::Right;
    current_coordinate.1 += 1;
}

fn move_vertical(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Up => move_up(current_coordinate, direction),
        Direction::Down => move_down(current_coordinate, direction),
        _ => unreachable!("vertical pipe while not going up or down"),
    }
}

fn move_horizontal(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Left => move_left(current_coordinate, direction),
        Direction::Right => move_right(current_coordinate, direction),
        _ => unreachable!("horizontal pipe while not going left or right"),
    }
}

fn move_bend_ne(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Down => move_right(current_coordinate, direction),
        Direction::Left => move_up(current_coordinate, direction),
        _ => unreachable!("NE bend while not going down or left"),
    }
}

fn move_bend_nw(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Down => move_left(current_coordinate, direction),
        Direction::Right => move_up(current_coordinate, direction),
        _ => unreachable!("NW bend while not going down or right"),
    }
}

fn move_bend_se(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Up => move_right(current_coordinate, direction),
        Direction::Left => move_down(current_coordinate, direction),
        _ => unreachable!("SE bend while not going up or left"),
    }
}

fn move_bend_sw(current_coordinate: &mut (usize, usize), direction: &mut Direction) {
    match direction {
        Direction::Up => move_left(current_coordinate, direction),
        Direction::Right => move_down(current_coordinate, direction),
        _ => unreachable!("SW bend while not going up or right"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let solution = "8";
        assert_eq!(solution, solve(input));
    }
}
