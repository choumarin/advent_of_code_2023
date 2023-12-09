use std::str::FromStr;

const INPUT: &str = include_str!("day09/input.txt");

#[derive(PartialEq, Eq, Debug)]
struct Sequence(Vec<Vec<i64>>);

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence(vec![s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()]))
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        "0 3 6 9 12 15".parse(),
        Ok(Sequence(vec![vec![0, 3, 6, 9, 12, 15]]))
    );
}

impl Sequence {
    fn add_line(&mut self) {
        let mut new_line = Vec::new();
        for w in self.0.last().unwrap().windows(2) {
            new_line.push(w[1] - w[0]);
        }
        self.0.push(new_line);
    }

    fn extrapolate(&mut self) {
        self.0.last_mut().unwrap().push(0);

        for i in (1..self.0.len()).rev() {
            let bottom_line = &self.0[i];
            let t = *bottom_line.last().unwrap();
            let top_line = &mut self.0[i - 1];
            top_line.push(top_line.last().unwrap() + t);
        }
    }

    fn extrapolate_back(&mut self) {
        self.0.last_mut().unwrap().insert(0, 0);

        for i in (1..self.0.len()).rev() {
            let bottom_line = &self.0[i];
            let t = *bottom_line.first().unwrap();
            let top_line = &mut self.0[i - 1];
            top_line.insert(0, top_line.first().unwrap() - t);
        }
    }

    fn next(&mut self) -> i64 {
        while !self.0.last().unwrap().iter().all(|i| i == &0) {
            self.add_line();
        }
        self.extrapolate();
        *self.0[0].last().unwrap()
    }

    fn prev(&mut self) -> i64 {
        while !self.0.last().unwrap().iter().all(|i| i == &0) {
            self.add_line();
        }
        self.extrapolate_back();
        *self.0[0].first().unwrap()
    }
}

#[test]
fn test_add_line() {
    let mut s = "0 3 6 9 12 15".parse::<Sequence>().unwrap();
    s.add_line();
    assert_eq!(
        s,
        Sequence(vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3]])
    );
}

#[test]
fn test_extrapolate() {
    let mut s = "0 3 6 9 12 15".parse::<Sequence>().unwrap();
    s.add_line();
    s.add_line();
    assert_eq!(
        s,
        Sequence(vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0]
        ])
    );
    s.extrapolate();
    assert_eq!(
        s,
        Sequence(vec![
            vec![0, 3, 6, 9, 12, 15, 18],
            vec![3, 3, 3, 3, 3, 3],
            vec![0, 0, 0, 0, 0]
        ])
    );
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<Sequence>().unwrap().next())
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<Sequence>().unwrap().prev())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
