use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day03/input.txt");

#[derive(Default, PartialEq, Eq, Debug, Hash)]
struct PartNum {
    val: i32,
    line: i32,
    col: i32,
    len: i32,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Gear {
    line: i32,
    col: i32,
}

struct Coord {
    line: i32,
    col: i32,
}

impl PartNum {
    fn adj_chars(&self, text: &str) -> Vec<(Coord, char)> {
        let mut adj = Vec::new();
        if self.line > 0 {
            let line = text.lines().nth((self.line - 1) as usize).unwrap();
            adj.extend(
                line.chars()
                    .enumerate()
                    .filter(|(i, _)| *i as i32 >= self.col - 1 && *i as i32 <= self.col + self.len)
                    .map(|(i, c)| {
                        (
                            Coord {
                                line: self.line - 1,
                                col: i as i32,
                            },
                            c,
                        )
                    }),
            );
        }
        let line = text.lines().nth((self.line) as usize).unwrap();
        adj.extend(
            line.chars()
                .enumerate()
                .filter(|(i, _)| *i as i32 == self.col - 1 || *i as i32 == self.col + self.len)
                .map(|(i, c)| {
                    (
                        Coord {
                            line: self.line,
                            col: i as i32,
                        },
                        c,
                    )
                }),
        );
        if let Some(line) = text.lines().nth((self.line + 1) as usize) {
            adj.extend(
                line.chars()
                    .enumerate()
                    .filter(|(i, _)| *i as i32 >= self.col - 1 && *i as i32 <= self.col + self.len)
                    .map(|(i, c)| {
                        (
                            Coord {
                                line: self.line + 1,
                                col: i as i32,
                            },
                            c,
                        )
                    }),
            );
        }
        adj
    }

    fn has_adj_symbols(&self, text: &str) -> bool {
        self.adj_chars(text)
            .iter()
            .any(|(_, c)| !c.is_ascii_digit() && *c != '.')
    }

    fn gears(&self, text: &str) -> Vec<Gear> {
        self.adj_chars(text)
            .iter()
            .enumerate()
            .filter_map(|(_, (coord, c))| match c {
                '*' => Some(Gear {
                    col: coord.col,
                    line: coord.line,
                }),
                _ => None,
            })
            .collect()
    }
}

#[test]
fn test_adj_char() {
    let text = "abc
def
ghi";
    let p = PartNum {
        col: 1,
        len: 1,
        line: 1,
        val: 1,
    };
    assert_eq!(
        p.adj_chars(text)
            .iter()
            .map(|(_, x)| *x)
            .collect::<Vec<_>>(),
        vec!['a', 'b', 'c', 'd', 'f', 'g', 'h', 'i']
    );
}

fn parts_in_str(line: i32, text: &str) -> Vec<PartNum> {
    let mut parts = Vec::new();
    let mut op: Option<PartNum> = None;
    for (i, c) in text.chars().enumerate() {
        if c.is_ascii_digit() {
            if let Some(ref mut p) = op {
                p.len += 1;
                p.val *= 10;
                p.val += c.to_digit(10).unwrap() as i32;
            } else {
                op = Some(PartNum {
                    line,
                    val: c.to_digit(10).unwrap() as i32,
                    col: i as i32,
                    len: 1,
                });
            }
        } else if let Some(p) = op {
            parts.push(p);
            op = None;
        }
    }
    if let Some(p) = op {
        parts.push(p);
    }
    parts
}

#[test]
fn test_parts_in_str() {
    let text = "...305.12";
    assert_eq!(
        parts_in_str(0, text),
        vec![
            PartNum {
                line: 0,
                val: 305,
                col: 3,
                len: 3
            },
            PartNum {
                line: 0,
                val: 12,
                col: 7,
                len: 2
            }
        ]
    );
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| parts_in_str(i as i32, l))
        .filter(|p| p.has_adj_symbols(input))
        .map(|p| p.val)
        .sum()
}

fn part2(input: &str) -> i32 {
    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| parts_in_str(i as i32, l))
        .collect::<Vec<_>>();
    let mut gears: HashMap<Gear, HashSet<&PartNum>> = HashMap::new();
    for part in parts.iter() {
        for gear in part.gears(input) {
            gears.entry(gear).or_default().insert(part);
        }
    }
    gears
        .iter()
        .filter(|(_, parts)| parts.len() >= 2)
        .map(|(_, parts)| parts.iter().map(|p| p.val).product::<i32>())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
