use rayon::prelude::*;
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day12/input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(PartialEq, Eq, Debug)]
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
    fn arrangements_count(&self) -> i64 {
        Self::better_count(&self.parts, &self.damaged_groups, &mut HashMap::new())
        // self.arrangements_count_recursive(self.parts.clone(), &mut HashMap::new())
    }

    fn valid_arrangement(&self, parts: &[PartStatus]) -> bool {
        let s = parts
            .iter()
            .copied()
            .map(Into::<char>::into)
            .collect::<String>();
        let t = s
            .split::<char>(PartStatus::Operational.into())
            .filter(|c| !c.is_empty())
            .map(|sub| sub.len() as i64)
            .collect::<Vec<_>>();
        self.damaged_groups == t
    }

    fn arrangements_count_recursive(
        &self,
        parts: Vec<PartStatus>,
        cache: &mut HashMap<Vec<PartStatus>, i64>,
    ) -> i64 {
        if let Some(res) = cache.get(&parts) {
            println!("cache hit");
            return *res;
        }
        let ret;
        if let Some(next) = parts.iter().position(|p| *p == PartStatus::Unknown) {
            let mut try_operational = parts.clone();
            try_operational[next] = PartStatus::Operational;
            let mut try_damaged = parts.clone();
            try_damaged[next] = PartStatus::Damaged;
            let count_operational = self.arrangements_count_recursive(try_operational, cache);
            let count_damaged = self.arrangements_count_recursive(try_damaged, cache);
            ret = count_operational + count_damaged;
        } else {
            if self.valid_arrangement(&parts) {
                ret = 1;
            } else {
                ret = 0;
            }
        }
        cache.insert(parts, ret);
        ret
    }

    fn unfold(&mut self) {
        let mut copy = self.parts.clone();
        copy.insert(0, PartStatus::Unknown);
        let copy2 = self.damaged_groups.clone();
        for _ in 1..=4 {
            self.parts.append(&mut copy.clone());
            self.damaged_groups.append(&mut copy2.clone());
        }
    }

    fn simplify(&mut self) {
        let mut index = 1;
        let mut prev = self.parts[0];
        while index < self.parts.len() {
            if prev == PartStatus::Operational && self.parts[index] == PartStatus::Operational {
                self.parts.remove(index);
            } else {
                prev = self.parts[index];
                index += 1;
            }
        }
    }

    fn better_count(
        mut parts: &[PartStatus],
        counts: &[i64],
        cache: &mut HashMap<(Vec<PartStatus>, Vec<i64>), i64>,
    ) -> i64 {
        if let Some(skip) = parts.iter().position(|p| *p != PartStatus::Operational) {
            parts = &parts[skip..];
        } else {
            parts = &[];
        }

        if let Some(ret) = cache.get(&(parts.to_vec(), counts.to_vec())) {
            return *ret;
        }

        let ret;

        if parts.is_empty() {
            if counts.is_empty() {
                ret = 1;
            } else {
                ret = 0;
            }
        } else if counts.is_empty() {
            if parts.iter().any(|p| *p == PartStatus::Damaged) {
                ret = 0;
            } else {
                ret = 1;
            }
        } else if parts[0] == PartStatus::Damaged {
            if parts.len() < counts[0] as usize {
                // can't have enough damaged parts
                ret = 0;
            } else if parts[..(counts[0] as usize)]
                .iter()
                .any(|p| *p == PartStatus::Operational)
            {
                // that would be 2 groups
                ret = 0;
            } else
            // all the next counts[0] are either damaged or unknown
            if parts.len() == counts[0] as usize {
                // it's the last group, so the parts can end with a damaged
                if counts.len() == 1 {
                    ret = 1;
                } else {
                    ret = 0;
                }
            } else
            // all the next counts[0] are damaged or unknown AND there's more
            if parts[counts[0] as usize] == PartStatus::Damaged {
                // if that was true, then counts[0] would be +1
                ret = 0;
            } else {
                ret = Self::better_count(&parts[((counts[0] + 1) as usize)..], &counts[1..], cache);
            }
            // move on to the next group, skip the next operational part
        } else {
            let mut damaged = parts.to_vec().clone();
            damaged[0] = PartStatus::Damaged;
            // damaged case + operational (lets just skip)
            ret = Self::better_count(&parts[1..], counts, cache)
                + Self::better_count(&damaged, counts, cache);
        }

        cache.insert((parts.to_vec(), counts.to_vec()), ret);
        return ret;
    }
}

#[test]
fn test_simplify() {
    let mut l = "#..# 1".parse::<Line>().unwrap();
    l.simplify();
    assert_eq!(l, "#.# 1".parse::<Line>().unwrap());
    let mut l = "...# 1".parse::<Line>().unwrap();
    l.simplify();
    assert_eq!(l, ".# 1".parse::<Line>().unwrap());
}

#[test]
fn test_arrangement_count() {
    // assert_eq!(". ".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("# 1".parse::<Line>().unwrap().arrangements_count(), 1);
    // assert_eq!("? ".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("? 1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("?? 1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("?? 2".parse::<Line>().unwrap().arrangements_count(), 1);
    // assert_eq!("?? ".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("?.? 1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("?.? 1,1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("#.. 1,1".parse::<Line>().unwrap().arrangements_count(), 0);
    assert_eq!("#.. 1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("#.? 1,1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("?#.? 1,1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("#?.? 1,1".parse::<Line>().unwrap().arrangements_count(), 1);
    assert_eq!("??.? 1,1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("?.?? 1,1".parse::<Line>().unwrap().arrangements_count(), 2);
    assert_eq!("??.?? 1,1".parse::<Line>().unwrap().arrangements_count(), 4);
    assert_eq!(
        "???.### 1,1,3"
            .parse::<Line>()
            .unwrap()
            .arrangements_count(),
        1
    );
    let mut s = ".??..??...?##. 1,1,3".parse::<Line>().unwrap();
    s.unfold();
    assert_eq!(s.arrangements_count(), 16384);
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut record = l.parse::<Line>().unwrap();
            record.simplify();
            record.arrangements_count()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .par_lines()
        .map(|l| {
            let mut record = l.parse::<Line>().unwrap();
            record.unfold();
            // record.simplify();
            record.arrangements_count()
        })
        .sum()
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
        assert_eq!(part2(TEST_INPUT_1), 525152);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
