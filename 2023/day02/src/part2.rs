use std::cmp::max;
use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(calculate_power_of_fewest_cubes)
        .sum::<u32>()
        .to_string()
}

fn calculate_power_of_fewest_cubes(input: &str) -> u32 {
    let mut cube_map: HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

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

        // Update the amount of cubes based on which is larger, the parsed amount or the previous one
        let current_amount: u32 = *cube_map.get(cube_color).expect("invalid cube color");
        cube_map.insert(cube_color, max(current_amount, cube_amount));
    }

    // Calculate the power of the set of cubes by multiplying them together
    let mut cube_amounts = cube_map.values();
    cube_amounts.next().expect("cube amount")
        * cube_amounts.next().expect("cube amount")
        * cube_amounts.next().expect("cube amount")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let solution = "2286";
        assert_eq!(solution, solve(input));
    }
}
