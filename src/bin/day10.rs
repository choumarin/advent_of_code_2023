use core::fmt;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const INPUT: &str = include_str!("day10/input.txt");

#[derive(PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => unimplemented!(),
        }
    }
}

impl core::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NS => write!(f, "║"),
            Self::EW => write!(f, "═"),
            Self::NE => write!(f, "╚"),
            Self::NW => write!(f, "╝"),
            Self::SW => write!(f, "╗"),
            Self::SE => write!(f, "╔"),
            Self::Ground => write!(f, " "),
            Self::Start => write!(f, "S"),
        }
    }
}

impl Pipe {
    fn connections(&self) -> Option<Vec<Direction>> {
        match self {
            Pipe::NS => Some(vec![Direction::N, Direction::S]),
            Pipe::EW => Some(vec![Direction::E, Direction::W]),
            Pipe::NE => Some(vec![Direction::N, Direction::E]),
            Pipe::NW => Some(vec![Direction::N, Direction::W]),
            Pipe::SW => Some(vec![Direction::S, Direction::W]),
            Pipe::SE => Some(vec![Direction::S, Direction::E]),
            Pipe::Ground => None,
            Pipe::Start => None, // special
        }
    }
}

#[derive(PartialEq, Eq)]
struct Map(Vec<Vec<Pipe>>);

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect()))
    }
}

impl core::fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.0.iter() {
            for c in line {
                write!(f, "{:?}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn get(&self, coords: Coords) -> Option<&Pipe> {
        if coords.line < 0 || coords.col < 0 {
            return None;
        }
        self.0
            .get(usize::try_from(coords.line).ok()?)?
            .get(usize::try_from(coords.col).ok()?)
    }

    fn get_start(&self) -> Coords {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(line, content)| {
                Some(Coords {
                    line: line as i64,
                    col: content.iter().position(|pipe| *pipe == Pipe::Start)? as i64,
                })
            })
            .next()
            .unwrap()
    }

    fn can_move(&self, coords: &Coords) -> Vec<Coords> {
        let mut ret = Vec::new();
        for dir in Direction::iter() {
            let can_from = match self.get(*coords) {
                Some(Pipe::Start) => true,
                Some(pipe) => {
                    if let Some(v) = pipe.connections() {
                        v.contains(&dir)
                    } else {
                        false
                    }
                }
                None => false,
            };
            if !can_from {
                continue;
            }

            let new_coords = *coords + dir.coords_offset();
            let can_to = match self.get(new_coords) {
                Some(Pipe::Start) => true,
                Some(pipe) => {
                    if let Some(v) = pipe.connections() {
                        v.contains(&dir.invert())
                    } else {
                        false
                    }
                }
                None => false,
            };
            if can_from && can_to {
                ret.push(new_coords);
            }
        }
        ret
    }

    fn max_depth(&self) -> i64 {
        let mut queue = VecDeque::new();
        let mut dmap: HashMap<Coords, i64> = HashMap::new();
        queue.push_back(self.get_start());
        let mut steps = 0;
        while !queue.is_empty() {
            let qlen = queue.len();
            for _ in 0..qlen {
                let current_coords = queue.pop_front().unwrap();
                if dmap.contains_key(&current_coords) {
                    continue;
                }
                dmap.insert(current_coords, steps);
                for new_coords in self.can_move(&current_coords) {
                    if dmap.contains_key(&new_coords) {
                        continue;
                    }
                    queue.push_back(new_coords);
                }
            }
            steps += 1;
        }
        steps - 1
    }
}

#[test]
fn it_parses() {
    let s = ".....
.S-7.
.|.|.
.L-J.
.....";
    dbg!(s.parse::<Map>().unwrap());
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    line: i64,
    col: i64,
}

impl std::ops::Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            line: self.line + rhs.line,
            col: self.col + rhs.col,
        }
    }
}

#[derive(EnumIter, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn coords_offset(&self) -> Coords {
        match self {
            Direction::N => Coords { line: -1, col: 0 },
            Direction::S => Coords { line: 1, col: 0 },
            Direction::E => Coords { line: 0, col: 1 },
            Direction::W => Coords { line: 0, col: -1 },
        }
    }

    fn invert(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

fn part1(input: &str) -> i64 {
    let map = input.parse::<Map>().unwrap();
    map.max_depth()
}

fn part2(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const TEST_INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 4);
        assert_eq!(part1(TEST_INPUT_2), 8);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT_1), 2);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
