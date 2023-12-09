use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let directions: &str = lines.next().expect("pattern");
    lines.next(); // Skip over the empty line

    // Node, (Left, Right)
    let node_instructions: HashMap<&str, (&str, &str)> = lines.map(parse_instruction).collect();

    // Start simultaneously from all nodes that start with 'A'
    let start_nodes: Vec<&str> = node_instructions
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect();

    let steps_from_target_nodes: Vec<usize> = start_nodes
        .iter()
        .map(|node| steps_to_target_node(node, directions, &node_instructions))
        .collect();

    // Lowest common multiple of all the steps
    lcm(&steps_from_target_nodes).to_string()
}

// (Label, (Left, Right))
fn parse_instruction(line: &str) -> (&str, (&str, &str)) {
    let sides = line.split_once('=').expect("sides of newline");
    let label = sides.0.trim();
    let (left, right) = sides.1.split_once(',').expect("left and right label");

    //  " (ZZZ"
    let left = left.split_once('(').expect("open paren").1;

    //  " ZZZ)"
    let right = right.trim().split_once(')').expect("close paren").0;

    (label, (left, right))
}

fn steps_to_target_node(node: &str, directions: &str, node_instructions: &HashMap<&str, (&str, &str)>) -> usize {
    let mut current_node = node;
    directions
        .chars()
        .cycle()
        .enumerate()
        .find_map(|(steps, direction)| {
            let options = node_instructions.get(current_node).expect("");
            let next_node = match direction {
                'L' => &options.0,
                'R' => &options.1,
                _ => unreachable!(),
            };
            if next_node.ends_with('Z') {
                Some(steps + 1)
            } else {
                current_node = next_node;
                None
            }
        })
        .expect("number of steps to the target node")
}

fn lcm(nums: &[usize]) -> usize {
    nums.iter()
        .fold(nums[0], |lcm, &num| lcm * num / gcd(lcm, num))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let solution = "6";
        assert_eq!(solution, solve(input));
    }
}
