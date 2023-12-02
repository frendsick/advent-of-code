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

fn is_possible_game(input: &str) -> bool {
    for part in input.split(&[':', ',', ';']) {
        // Skip over "Game X"
        if part.starts_with("Game") {
            continue;
        }

        // Parse the color and the amount of cubes
        let mut color_with_count = part.split_whitespace();
        let cube_amount: u32 = color_with_count
            .next()
            .expect("number of colored cubes")
            .parse()
            .expect("valid number");
        let cube_color: &str = color_with_count.next().expect("color");

        // The game is not possible if elf has more colored cubes than available in the bag
        // Expect that the color exists in the bag
        if BAG.get(cube_color).unwrap() < &cube_amount {
            return false;
        }
    }
    true
}

fn parse_game_id(input: &str) -> u32 {
    // The game ID starts after "GAME " and ends at ':'
    let start_ptr: usize = 5;
    let end_ptr: usize = input.find(':').expect("game ID should end with ':'");
    let game_id: u32 = input[start_ptr..end_ptr]
        .parse()
        .expect("game ID should be a number");
    game_id
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
