use std::collections::HashMap;

const INPUT: &str = include_str!("day08/input.txt");

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn idx(&self) -> usize {
        match &self {
            Direction::Left => 0,
            Direction::Right => 1,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<&str, Vec<&str>>) {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();
    let mut adj_list = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('=');
        let from = parts.next().unwrap().trim();
        let to = parts
            .next()
            .unwrap()
            .trim_matches(|c: char| c == '(' || c == ')' || c.is_whitespace())
            .split(',')
            .map(|s| s.trim())
            .collect();
        adj_list.insert(from, to);
    }
    (directions, adj_list)
}

fn part1(input: &str) -> i64 {
    let (directions, adj_list) = parse(input);
    let mut current_loc = "AAA";
    let mut steps = 0;
    assert!(!directions.is_empty());
    let mut direction_iter = directions.iter().cycle();
    while current_loc != "ZZZ" {
        current_loc = adj_list.get(current_loc).unwrap()[direction_iter.next().unwrap().idx()];
        steps += 1;
    }
    steps
}

fn part2(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 2);
        assert_eq!(part1(TEST_INPUT_2), 6);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
