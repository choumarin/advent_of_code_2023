use std::{collections::HashMap, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const INPUT: &str = include_str!("day02/input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cubes: HashMap<Color, u32>,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand {
            cubes: s
                .split(',')
                .map(|s| {
                    let v = s.split_whitespace().collect::<Vec<&str>>();
                    (Color::from_str(v[1]).unwrap(), u32::from_str(v[0]).unwrap())
                })
                .collect::<HashMap<Color, u32>>(),
        })
    }
}

impl Hand {
    fn power(&self) -> u32 {
        let mut res = 1;
        for c in Color::iter() {
            res *= self.cubes.get(&c).unwrap_or(&0);
        }
        res
    }
}

#[test]
fn test_parse_hand() {
    let s = "7 blue, 4 red, 11 green";
    let h = Hand::from_str(s).unwrap();
    assert_eq!(
        h.cubes,
        HashMap::from([(Color::Blue, 7), (Color::Red, 4), (Color::Green, 11)])
    );
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(':').collect::<Vec<&str>>();
        let game = v[0].split_whitespace().collect::<Vec<&str>>();
        assert_eq!(game[0], "Game");
        let id = u32::from_str(game[1]).unwrap();
        let hands = v[1]
            .split(';')
            .map(|h| Hand::from_str(h).unwrap())
            .collect::<Vec<Hand>>();
        Ok(Game { id, hands })
    }
}

impl Game {
    fn is_possible(&self, initial: &HashMap<Color, u32>) -> bool {
        for hand in self.hands.iter() {
            for (&color, &count) in hand.cubes.iter() {
                if &count > initial.get(&color).unwrap() {
                    return false;
                }
            }
        }
        true
    }

    fn min_initial(&self) -> Hand {
        let mut res = Hand {
            cubes: HashMap::new(),
        };
        for hand in self.hands.iter() {
            for (&color, &count) in hand.cubes.iter() {
                res.cubes
                    .entry(color)
                    .and_modify(|e| *e = std::cmp::max(*e, count))
                    .or_insert(count);
            }
        }
        res
    }
}

#[test]
fn test_parse_game() {
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let g = Game::from_str(s).unwrap();
    assert_eq!(
        g,
        Game {
            id: 1,
            hands: vec![
                Hand {
                    cubes: HashMap::from([(Color::Blue, 3), (Color::Red, 4)])
                },
                Hand {
                    cubes: HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)])
                },
                Hand {
                    cubes: HashMap::from([(Color::Green, 2)])
                },
            ]
        }
    )
}

fn part1(input: &str) -> u32 {
    let initial: HashMap<Color, u32> =
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    input
        .lines()
        .map(|l| Game::from_str(l).unwrap())
        .filter(|g| g.is_possible(&initial))
        .map(|g| g.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| Game::from_str(l).unwrap())
        .map(|g| g.min_initial())
        .map(|h| h.power())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT), 2286);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
