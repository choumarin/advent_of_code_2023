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

    fn hashes(&self) -> (HashMap<Vec<Terrain>, Vec<usize>>, HashMap<Vec<Terrain>, Vec<usize>>) {
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

fn part1(input: &str) -> i64 {
    unimplemented!()
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
