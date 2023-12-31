use core::fmt;
use std::collections::{HashMap, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod common;
use common::{Coords, Map};

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
            o => panic!("`{}` not implemented", o),
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
            Self::Ground => write!(f, "-"),
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

type PipeMap = Map<Pipe>;

impl PipeMap {
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

    fn depth_map(&self) -> HashMap<Coords, i64> {
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
        dmap
    }

    fn count_in(&self, lop: Vec<Coords>) -> i64 {
        let mut count = 0;
        for (line, content) in self.0.iter().enumerate() {
            let mut inside = false;
            for (col, p) in content.iter().enumerate() {
                let here = Coords {
                    line: line as i64,
                    col: col as i64,
                };
                match p {
                    Pipe::NS | Pipe::NE | Pipe::NW => {
                        if lop.contains(&here) {
                            inside = !inside;
                        } else if inside {
                            // println!("inside {:?}", &here);
                            count += 1;
                        }
                    }
                    Pipe::Start => {
                        let above = here + Direction::N.coords_offset();
                        if lop.contains(&above)
                            && self.get(above).is_some_and(|p| {
                                p.connections().is_some_and(|v| v.contains(&Direction::S))
                            })
                        {
                            inside = !inside;
                        }
                    }
                    _ => {
                        if !lop.contains(&here) && inside {
                            // println!("inside {:?}", &here);
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[test]
fn it_parses() {
    let s = ".....
.S-7.
.|.|.
.L-J.
.....";
    dbg!(s.parse::<Map<Pipe>>().unwrap());
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
    let map = input.parse::<Map<_>>().unwrap();
    let dmap = map.depth_map();
    dmap.into_values().max().unwrap()
}

fn part2(input: &str) -> i64 {
    let map = input.parse::<Map<_>>().unwrap();
    let dmap = map.depth_map();
    let lop = dmap.into_keys().collect();
    map.count_in(lop)
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

    const TEST_INPUT_3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const TEST_INPUT_4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const TEST_INPUT_5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 4);
        assert_eq!(part1(TEST_INPUT_2), 8);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT_3), 4);
        assert_eq!(part2(TEST_INPUT_4), 8);
        // dbg!(TEST_INPUT_5.parse::<Map>().unwrap());
        assert_eq!(part2(TEST_INPUT_5), 10);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
