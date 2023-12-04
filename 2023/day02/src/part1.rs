use once_cell::sync::Lazy;
use std::collections::HashMap;

static BAG: Lazy<HashMap<&'static str, u32>> =
    Lazy::new(|| HashMap::from([("red", 12), ("green", 13), ("blue", 14)]));

pub fn solve(input: &str) -> String {
    input
        .lines()
        .filter_map(parse_possible_game_id)
        .sum::<u32>()
        .to_string()
}

fn parse_possible_game_id(input: &str) -> Option<u32> {
    match is_possible_game(input) {
        true => Some(parse_game_id(input)),
        false => None,
    }
}

fn parse_game_id(input: &str) -> u32 {
    let start_ptr: usize = 5;
    let end_ptr: usize = input.find(':').expect("game ID should end with ':'");
    input[start_ptr..end_ptr]
        .parse()
        .expect("game ID should be a number")
}

fn is_possible_game(input: &str) -> bool {
    input
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        .split(&[':', ',', ';'])
        // Skip over "Game X"
        .filter(|part| !part.starts_with("Game"))
        // Game is possible if all sets of cubes are within the bag
        .all(is_possible_set)
}

fn is_possible_set(game_set: &str) -> bool {
    let mut color_with_count = game_set.split_whitespace();
    let cube_amount: u32 = color_with_count
        .next()
        .expect("number of colored cubes")
        .parse()
        .expect("valid number");
    let cube_color: &str = color_with_count.next().expect("color");
    BAG.get(cube_color).unwrap() >= &cube_amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let solution = "8";
        assert_eq!(solution, solve(input));
    }
}
