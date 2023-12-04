const INPUT: &str = include_str!("day03/input.txt");

#[derive(Default, PartialEq, Eq, Debug)]
struct PartNum {
    val: i32,
    line: i32,
    col: i32,
    len: i32,
}

impl PartNum {
    fn adj_chars(&self, text: &str) -> Vec<char> {
        let mut adj = Vec::new();
        if self.line > 0 {
            let line = text.lines().nth((self.line - 1) as usize).unwrap();
            adj.extend(
                line.chars()
                    .enumerate()
                    .filter(|(i, _)| *i as i32 >= self.col - 1 && *i as i32 <= self.col + self.len)
                    .map(|(_, c)| c),
            );
        }
        let line = text.lines().nth((self.line) as usize).unwrap();
        adj.extend(
            line.chars()
                .enumerate()
                .filter(|(i, _)| *i as i32 == self.col - 1 || *i as i32 == self.col + self.len)
                .map(|(_, c)| c),
        );
        if let Some(line) = text.lines().nth((self.line + 1) as usize) {
            adj.extend(
                line.chars()
                    .enumerate()
                    .filter(|(i, _)| *i as i32 >= self.col - 1 && *i as i32 <= self.col + self.len)
                    .map(|(_, c)| c),
            );
        }
        adj
    }

    fn has_adj_symbols(&self, text: &str) -> bool {
        self.adj_chars(text)
            .iter()
            .any(|c| !c.is_digit(10) && *c != '.')
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
        p.adj_chars(text),
        vec!['a', 'b', 'c', 'd', 'f', 'g', 'h', 'i']
    );
}

fn parts_in_str(line: i32, text: &str) -> Vec<PartNum> {
    let mut parts = Vec::new();
    let mut op: Option<PartNum> = None;
    for (i, c) in text.chars().enumerate() {
        if c.is_digit(10) {
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
        } else {
            if let Some(p) = op {
                parts.push(p);
                op = None;
            }
        }
    }
    if let Some(p) = op {
        parts.push(p);
        op = None;
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
        .map(|(i, l)| parts_in_str(i as i32, l))
        .flatten()
        .filter(|p| p.has_adj_symbols(input))
        .map(|p| p.val)
        .sum()
}

fn part2(input: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const test_input: &str = "467..114..
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
        assert_eq!(part1(test_input), 4361);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(test_input), 467835);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
