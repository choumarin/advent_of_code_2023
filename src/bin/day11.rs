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

    fn galaxies_lines_cols(&self) -> (HashSet<i64>, HashSet<i64>) {
        let mut cols_with_galaxies = HashSet::new();
        let mut lines_with_galaxies = HashSet::new();
        for (line, content) in self.0.iter().enumerate() {
            let mut galaxies_in_line = false;
            for (col, space) in content.iter().enumerate() {
                if *space == Space::Galaxy {
                    cols_with_galaxies.insert(col as i64);
                    galaxies_in_line = true;
                }
            }
            if galaxies_in_line {
                lines_with_galaxies.insert(line as i64);
            }
        }
        (lines_with_galaxies, cols_with_galaxies)
    }

    fn expand(&mut self, factor: i64) {
        let (lines_with_galaxies, cols_with_galaxies) = self.galaxies_lines_cols();
        // insert lines
        let len = self.0.len();
        for i in (0..len).rev() {
            if !lines_with_galaxies.contains(&(i as i64)) {
                for _ in 0..(factor - 1) {
                    self.0.insert(i, self.empty_line())
                }
            }
        }
        // insert cols
        for line in self.0.iter_mut() {
            let len = line.len();
            for i in (0..len).rev() {
                if !cols_with_galaxies.contains(&(i as i64)) {
                    for _ in 0..(factor - 1) {
                        line.insert(i, Space::Empty);
                    }
                }
            }
        }
    }

    fn expand_coords(&self, coords: Coords, factor: i64) -> Coords {
        let (lines_with_galaxies, cols_with_galaxies) = self.galaxies_lines_cols();
        let lines = lines_with_galaxies
            .iter()
            .filter(|&&i| i < coords.line)
            .count() as i64;
        let new_line = (coords.line - lines) * (factor - 1) + coords.line;
        let cols = cols_with_galaxies
            .iter()
            .filter(|&&i| i < coords.col)
            .count() as i64;
        let new_col = (coords.col - cols) * (factor - 1) + coords.col;
        Coords {
            line: new_line,
            col: new_col,
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
    universe.expand(2);

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
    universe.expand(2);
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
    let universe = input.parse::<Universe>().unwrap();
    let mut galaxies = universe.galaxies();
    for g in galaxies.iter_mut() {
        *g = universe.expand_coords(*g, 2);
    }
    let mut total_distance = 0;
    let pairs = uniq_pairs(galaxies);
    for pair in pairs {
        let dist = pair.0.dist_square(&pair.1);
        total_distance += dist;
    }
    total_distance
}

fn part2(input: &str) -> i64 {
    let universe = input.parse::<Universe>().unwrap();
    let mut galaxies = universe.galaxies();
    for g in galaxies.iter_mut() {
        *g = universe.expand_coords(*g, 1_000_000);
    }
    let mut total_distance = 0;
    let pairs = uniq_pairs(galaxies);
    for pair in pairs {
        let dist = pair.0.dist_square(&pair.1);
        total_distance += dist;
    }
    total_distance
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
        assert_eq!(part2(TEST_INPUT_1), 82000210);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
