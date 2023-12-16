#[derive(Debug)]
struct ConditionRecord {
    springs: Vec<Spring>,
    group_sizes: Vec<usize>,
}

#[derive(Debug)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

pub fn solve(input: &str) -> String {
    let spring_rows: Vec<ConditionRecord> = parse_lines(input);
    dbg!(spring_rows);

    "TODO".to_string()
}

fn parse_lines(input: &str) -> Vec<ConditionRecord> {
    input.lines().map(parse_condition_record).collect()
}

fn parse_condition_record(line: &str) -> ConditionRecord {
    let (springs_str, group_sizes_str) = line
        .split_once(' ')
        .expect("Condition record parts are separated by a space");

    let springs = parse_springs(springs_str);
    let group_sizes = parse_group_sizes(group_sizes_str);
    ConditionRecord {
        springs,
        group_sizes,
    }
}

fn parse_springs(input: &str) -> Vec<Spring> {
    input.chars().map(char_to_spring).collect()
}

fn parse_group_sizes(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|c| c.parse().expect("group size"))
        .collect()
}

fn char_to_spring(character: char) -> Spring {
    match character {
        '#' => Spring::Damaged,
        '.' => Spring::Operational,
        '?' => Spring::Unknown,
        _ => unreachable!("The puzzle input consists of '#', '.', and '?' characters"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let solution = "21";
        assert_eq!(solution, solve(input));
    }
}
