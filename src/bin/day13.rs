use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day13/input.txt");

const TEST_INPUT_1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Terrain {
    Ash,
    Rocks,
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            other => panic!("{other:?} is not a valid Terrain"),
        }
    }
}

struct Pattern(Vec<Vec<Terrain>>);

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern(
            s.lines()
                .filter_map(|l| (!l.is_empty()).then(|| l.chars().map(|c| c.into()).collect()))
                .collect(),
        ))
    }
}

impl<'a> Pattern {
    fn cols(&'a self) -> impl Iterator<Item = Vec<Terrain>> + 'a {
        ColIterator {
            pattern: &self,
            current: 0,
        }
    }
}

impl Pattern {
    fn lines(&self) -> impl Iterator<Item = Vec<Terrain>> + '_ {
        self.0.iter().map(|l| l.iter().map(|t| *t).collect())
    }

    fn mirror_i(i: usize, mirror: usize) -> Option<usize> {
        let mirror = mirror as f64 + 0.5;
        let n = 2.0 * mirror - i as f64;
        (n >= 0.0).then_some(n as usize)
    }

    fn is_mirror_line(&self, mirror: usize) -> bool {
        let lines_count = self.0.len();
        for i in 0..lines_count {
            let a = self.lines().nth(i).unwrap();
            let Some(j) = Self::mirror_i(i, mirror) else {
                continue;
            };
            let Some(b) = self.lines().nth(j) else {
                continue;
            };
            if a != b {
                return false;
            }
        }
        true
    }

    fn is_mirror_col(&self, mirror: usize) -> bool {
        let cols_count = self.0[0].len();
        for i in 0..cols_count {
            let a = self.cols().nth(i).unwrap();
            let Some(j) = Self::mirror_i(i, mirror) else {
                continue;
            };
            let Some(b) = self.cols().nth(j) else {
                continue;
            };
            if a != b {
                return false;
            }
        }
        true
    }

    fn mirror_line(&self) -> Option<usize> {
        let l_max = self.0.len();
        for i in 0..(l_max - 1) {
            // Assumes only 1
            if self.is_mirror_line(i) {
                return Some(i);
            }
        }
        None
    }

    fn mirror_col(&self) -> Option<usize> {
        let c_max = self.0[0].len();
        for i in 0..(c_max - 1) {
            // Assumes only 1
            if self.is_mirror_col(i) {
                return Some(i);
            }
        }
        None
    }

    fn score(&self) -> usize {
        self.mirror_line().map(|i| (i + 1) * 100).unwrap_or(0)
            + self.mirror_col().map(|i| i + 1).unwrap_or(0)
    }

    fn hashes(
        &self,
    ) -> (
        HashMap<Vec<Terrain>, Vec<usize>>,
        HashMap<Vec<Terrain>, Vec<usize>>,
    ) {
        let mut lines: HashMap<Vec<Terrain>, Vec<usize>> = HashMap::new();
        for (i, line) in self.lines().enumerate() {
            lines.entry(line).or_default().push(i);
        }
        let mut cols: HashMap<Vec<Terrain>, Vec<usize>> = HashMap::new();
        for (i, col) in self.cols().enumerate() {
            cols.entry(col).or_default().push(i);
        }
        (lines, cols)
    }

    fn find_mirror_line(hashes: HashMap<Vec<Terrain>, Vec<usize>>) -> Option<usize> {
        let mut candidates = Vec::new();
        for (_, v) in hashes {
            if v.len() >= 2 {
                candidates.push((v[0] + v[1]) / 2)
            }
        }
        todo!()
    }
}

#[test]
fn test_mirror() {
    //  x><x
    //  x >< x
    // ><
    //           ><
    //      ><
    // 321123
    // 01234567890123
    assert_eq!(Pattern::mirror_i(1, 2), Some(4)); // 2 + (2-1) +1 = 2
    assert_eq!(Pattern::mirror_i(1, 3), Some(6)); // 3 + (3-1) + 1 = 4
    assert_eq!(Pattern::mirror_i(1, 0), Some(0));
    assert_eq!(Pattern::mirror_i(2, 0), None);
    assert_eq!(Pattern::mirror_i(9, 10), Some(12));
    assert_eq!(Pattern::mirror_i(10, 10), Some(11));
    assert_eq!(Pattern::mirror_i(12, 10), Some(9));
    assert_eq!(Pattern::mirror_i(1, 3), Some(6));
    assert_eq!(Pattern::mirror_i(0, 3), Some(7));
}

#[test]
fn mirror_test() {
    let p: Pattern = "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
        .parse()
        .unwrap();
    assert!(!p.is_mirror_line(5));
    assert!(p.is_mirror_line(3));
    assert!(p.is_mirror_line(6));
    assert_eq!(p.score(), 400);
    let p: Pattern = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
        .parse()
        .unwrap();
    assert!(!p.is_mirror_col(5));
    assert!(p.is_mirror_col(4));
    assert!(p.is_mirror_col(9));
    assert_eq!(p.score(), 5);
}

struct ColIterator<'a> {
    pattern: &'a Pattern,
    current: usize,
}

impl<'a> Iterator for ColIterator<'a> {
    type Item = Vec<Terrain>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.pattern.0[0].len() {
            return None;
        }
        let ret = Some(self.pattern.0.iter().map(|l| l[self.current]).collect());
        self.current += 1;
        ret
    }
}

#[test]
fn test_iter() {
    let input = ".#.
#.#";
    let p = input.parse::<Pattern>().unwrap();
    let mut cols = p.cols();
    assert_eq!(cols.next(), Some(vec![Terrain::Ash, Terrain::Rocks]));
    assert_eq!(cols.next(), Some(vec![Terrain::Rocks, Terrain::Ash]));
    assert_eq!(cols.next(), Some(vec![Terrain::Ash, Terrain::Rocks]));
    assert_eq!(cols.next(), None);
    let mut lines = p.lines();
    assert_eq!(
        lines.next(),
        Some(vec![Terrain::Ash, Terrain::Rocks, Terrain::Ash])
    );
    assert_eq!(
        lines.next(),
        Some(vec![Terrain::Rocks, Terrain::Ash, Terrain::Rocks])
    );
    assert_eq!(lines.next(), None);
}

#[test]
fn test_hashes() {
    let p: Pattern = TEST_INPUT_1.parse().unwrap();
    dbg!(p.hashes());
}

fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(|s| s.parse().unwrap()).collect()
}

fn part1(input: &str) -> i64 {
    parse(input).iter().map(|p|p.score()).sum::<usize>() as i64
}

fn part2(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 405);
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
