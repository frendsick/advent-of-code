use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let directions: &str = lines.next().expect("pattern");
    lines.next(); // Skip over the empty line

    // Node, (Left, Right)
    let node_instructions: HashMap<String, (String, String)> =
        lines.map(parse_instruction).collect();

    let mut current_node = "AAA";
    let target_node = "ZZZ";

    // Loop the characters from `directions` indefinitely until the target node is found
    let steps = directions
        .chars()
        .cycle()
        .enumerate()
        .find_map(|(steps, direction)| {
            let options = node_instructions.get(current_node).expect("");
            let next_node = match direction {
                'L' => &options.0,
                'R' => &options.1,
                _ => unreachable!(""),
            };
            if next_node == target_node {
                Some(steps + 1)
            } else {
                current_node = next_node;
                None
            }
        })
        .expect("number of steps to the target node");

    steps.to_string()
}

// (Label, (Left, Right))
fn parse_instruction(line: &str) -> (String, (String, String)) {
    let sides = line.split_once('=').expect("sides of newline");
    let label = sides.0.trim();
    let (left, right) = sides.1.split_once(',').expect("left and right label");

    //  " (ZZZ"
    let left = left.split_once('(').expect("open paren").1;

    //  " ZZZ)"
    let right = right.trim().split_once(')').expect("close paren").0;

    (label.to_string(), (left.to_string(), right.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let solution = "6";
        assert_eq!(solution, solve(input));
    }
}
