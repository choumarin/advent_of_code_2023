use std::str::FromStr;

use num::complex::ComplexFloat;

const INPUT: &str = include_str!("day12/input.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum PartStatus {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for PartStatus {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            o => panic!("`{}` is not a valid PartStatus", o),
        }
    }
}

impl Into<char> for PartStatus {
    fn into(self) -> char {
        match self {
            PartStatus::Operational => '.',
            PartStatus::Damaged => '#',
            PartStatus::Unknown => '?',
        }
    }
}

struct Line {
    parts: Vec<PartStatus>,
    damaged_groups: Vec<i64>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let parts = split.next().unwrap().chars().map(|c| c.into()).collect();
        let mut damaged_groups = Vec::new();
        if let Some(damaged_group_data) = split.next() {
            damaged_groups = damaged_group_data
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect();
        }
        Ok(Line {
            parts,
            damaged_groups,
        })
    }
}

impl Line {
    // fn is_valid(&self) -> bool {
    //     if self.parts.iter().any(|p| *p == PartStatus::Unknown) {
    //         return false;
    //     }
    //     let s = self
    //         .parts
    //         .iter()
    //         .copied()
    //         .map(|p| Into::<char>::into(p))
    //         .collect::<String>();
    //     let t = s
    //         .split::<char>(PartStatus::Operational.into())
    //         .map(|sub| sub.len() as i64)
    //         .collect::<Vec<_>>();
    //     t == self.damaged_groups
    // }

    fn arrangements_count(&self) -> i64 {
        Self::arrangements_count_recursive(&self.parts, self.damaged_groups.clone())
    }

    fn arrangements_count_recursive(parts: &[PartStatus], damaged_groups: Vec<i64>) -> i64 {
        if parts.is_empty() {
            if damaged_groups.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }
        if damaged_groups.is_empty() {
            if parts.iter().any(|p| *p == PartStatus::Damaged) {
                return 0;
            } else {
                return 1;
            }
        }
        match parts[0] {
            PartStatus::Operational => {
                Self::arrangements_count_recursive(&parts[1..], damaged_groups.clone())
            }
            PartStatus::Damaged => {
                if damaged_groups[0] == 0 {
                    return 0;
                }
                let mut damaged_groups = damaged_groups.clone();
                damaged_groups[0] -= 1;
                Self::arrangements_count_recursive(&parts[1..], damaged_groups)
            }
            PartStatus::Unknown => {
                let mut damaged_groups2 = damaged_groups.clone();
                damaged_groups2[0] -= 1;
                if damaged_groups2[0] == 0 {
                    damaged_groups2.remove(0);
                }
                let it_s_damaged = Self::arrangements_count_recursive(&parts[1..], damaged_groups2);
                let it_s_operational =  Self::arrangements_count_recursive(&parts[1..], damaged_groups.clone());
                it_s_damaged + it_s_operational
            }
        }
    }
}

#[test]
fn test_arrangement_count() {
    assert_eq!(". ".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("# 1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("? ".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("? 1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("?? 1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("?? 2".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("?? ".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("?.? 1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("?.? 1,1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("#.. 1,1".parse::<Line>().unwrap().arrangements_count(), 0);
    assert_eq!("#.. 1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("#.? 1,1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("?#.? 1,1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("#?.? 1,1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("??.? 1,1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("?.?? 1,1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!(
        "??.?? 1,1".parse::<Line>().unwrap().arrangements_count(),
        4
    );
    assert_eq!(
        "???.### 1,1,3".parse::<Line>().unwrap().arrangements_count(),
        1
    );
}

fn part1(input: &str) -> i64 {
    input.lines().map(|l| l.parse::<Line>().unwrap().arrangements_count()).sum()
}

fn part2(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 21);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT_1), 82000210);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
