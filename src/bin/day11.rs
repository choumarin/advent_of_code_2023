mod common;
use core::fmt;
use std::collections::HashSet;

use common::{Coords, Map};

const INPUT: &str = include_str!("day11/input.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Galaxy,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            v => panic!("{v:?} not a valid space"),
        }
    }
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}

type Universe = Map<Space>;

impl Universe {
    fn galaxies(&self) -> Vec<Coords> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(line, content)| {
                content.iter().enumerate().filter_map(move |(col, space)| {
                    (*space == Space::Galaxy).then_some({
                        Coords {
                            line: line as i64,
                            col: col as i64,
                        }
                    })
                })
            })
            .collect()
    }

    fn expand(&mut self) {
        let mut cols_with_galaxies = HashSet::new();
        let mut lines_with_galaxies = HashSet::new();
        for (line, content) in self.0.iter().enumerate() {
            let mut galaxies_in_line = false;
            for (col, space) in content.iter().enumerate() {
                if *space == Space::Galaxy {
                    cols_with_galaxies.insert(col);
                    galaxies_in_line = true;
                }
            }
            if galaxies_in_line {
                lines_with_galaxies.insert(line);
            }
        }
        // insert lines
        let len = self.0.len();
        for i in (0..len).rev() {
            if !lines_with_galaxies.contains(&i) {
                self.0.insert(i, self.empty_line())
            }
        }
        // insert cols
        for line in self.0.iter_mut() {
            let len = line.len();
            for i in (0..len).rev() {
                if !cols_with_galaxies.contains(&i) {
                    line.insert(i, Space::Empty);
                }
            }
        }
    }

    fn empty_line(&self) -> Vec<Space> {
        vec![Space::Empty; self.0[0].len()]
    }
}

#[test]
fn test_expand() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    let expanded = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    let mut universe = input.parse::<Universe>().unwrap();
    universe.expand();

    assert_eq!(universe, Universe::from(expanded));
}

#[test]
fn test_galaxies() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let mut universe = input.parse::<Universe>().unwrap();
    universe.expand();
    let galaxies = universe.galaxies();
    assert_eq!(galaxies[4].dist_square(&galaxies[8]), 9);
    assert_eq!(galaxies[0].dist_square(&galaxies[6]), 15);
    assert_eq!(galaxies[2].dist_square(&galaxies[5]), 17);
}

fn uniq_pairs<T: Copy>(v: Vec<T>) -> Vec<(T, T)> {
    let mut pairs = Vec::new();
    for lim in 0..v.len() {
        for s in (lim + 1)..v.len() {
            pairs.push((v[lim], v[s]));
        }
    }
    pairs
}
#[test]
fn test_uniq_pairs() {
    let v = vec![1, 2, 3];
    assert_eq!(uniq_pairs(v), vec![(1, 2), (1, 3), (2, 3)]);
}

fn part1(input: &str) -> i64 {
    let mut universe = input.parse::<Universe>().unwrap();
    universe.expand();
    let galaxies = universe.galaxies();
    let mut total_distance = 0;
    let pairs = uniq_pairs(galaxies);
    for pair in pairs{
        let dist = pair.0.dist_square(&pair.1);
        total_distance += dist;
    }
    total_distance
}

fn part2(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 374);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT_1), 4);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
