use core::fmt;
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day07/input.txt");

const CARDS_FACES: &[char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    House,
    FourKind,
    FiveKind,
}

#[test]
fn test_type_ord() {
    assert!(HandType::FiveKind > HandType::FourKind);
}

struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().collect::<Vec<_>>();
        if cards.len() != 5 || !cards.iter().all(|c| CARDS_FACES.contains(c)) {
            return Err(());
        }
        let hand_type = type_from_cards(&cards);
        Ok(Hand { cards, hand_type })
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{").unwrap();
        write!(f, "{}", self.cards.iter().collect::<String>()).unwrap();
        write!(f, ", ").unwrap();
        write!(f, "{:?}", self.hand_type).unwrap();
        write!(f, "}}").unwrap();
        Ok(())
    }
}

fn type_from_cards(cards: &Vec<char>) -> HandType {
    let mut map = HashMap::<char, i32>::new();
    for c in cards {
        *map.entry(*c).or_insert(0) += 1;
    }
    let mut counts = map.into_values().collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    // Assume all is ok for defaults
    match counts[0] {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => match counts[1] {
            2 => HandType::House,
            _ => HandType::ThreeKind,
        },
        2 => match counts[1] {
            2 => HandType::TwoPair,
            _ => HandType::OnePair,
        },
        _ => HandType::High,
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        "32T3K".parse(),
        Ok(Hand {
            cards: vec!['3', '2', 'T', '3', 'K'],
            hand_type: HandType::OnePair
        })
    );
    assert_eq!(
        "T55J5".parse(),
        Ok(Hand {
            cards: vec!['T', '5', '5', 'J', '5'],
            hand_type: HandType::ThreeKind
        })
    );
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if CARDS_FACES.iter().position(|c| c == a).unwrap()
                    == CARDS_FACES.iter().position(|c| c == b).unwrap()
                {
                    continue;
                }
                return CARDS_FACES
                    .iter()
                    .position(|c| c == b)
                    .unwrap()
                    .partial_cmp(&CARDS_FACES.iter().position(|c| c == a).unwrap());
            }
        }
        None
    }
}

#[test]
fn test_cmp() {
    assert!("32T3K".parse::<Hand>().unwrap() < "T55J5".parse::<Hand>().unwrap());
    assert!("33332".parse::<Hand>().unwrap() > "2AAAA".parse::<Hand>().unwrap());
    assert!("KK677".parse::<Hand>().unwrap() > "KTJJT".parse::<Hand>().unwrap());
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

fn part1(input: &str) -> i64 {
    let mut hands = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let hand = parts.next().unwrap().parse::<Hand>().unwrap();
            let bet = parts.next().unwrap().parse::<i64>().unwrap();
            (hand, bet)
        })
        .collect::<Vec<_>>();
    hands.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // dbg!(&hands);
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) as i64 * bet)
        .sum()
}

fn part2(input: &str) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse1() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
