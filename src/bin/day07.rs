use core::fmt;
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day07/input.txt");

const CARDS_FACES_P1: &[char] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARDS_FACES_P2: &[char] = &[
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
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
    assert!(HandType::FourKind > HandType::ThreeKind);
}

#[derive(PartialEq, Eq, Debug)]
enum Rules {
    Part1,
    Part2,
}

struct ParsedHand {
    cards: Vec<char>,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    rules: Rules,
}

impl FromStr for ParsedHand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().collect::<Vec<_>>();
        if cards.len() != 5 || !cards.iter().all(|c| CARDS_FACES_P1.contains(c)) {
            return Err(());
        }
        Ok(ParsedHand { cards })
    }
}

impl ParsedHand {
    fn hand(self, rules: Rules) -> Hand {
        Hand {
            hand_type: match rules {
                Rules::Part1 => type_from_cards(&self.cards),
                Rules::Part2 => type_from_cards_with_jocker(&self.cards),
            },
            cards: self.cards,
            rules,
        }
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

fn type_from_cards_with_jocker(cards: &Vec<char>) -> HandType {
    let mut map = HashMap::<char, i32>::new();
    for c in cards {
        *map.entry(*c).or_insert(0) += 1;
    }
    let mut counts_no_jockers = map
        .iter()
        .filter_map(|(&key, val)| (key != 'J').then_some(val))
        .collect::<Vec<_>>();
    counts_no_jockers.sort_unstable_by(|a, b| b.cmp(a));

    if counts_no_jockers.is_empty() {
        return HandType::FiveKind;
    }
    match counts_no_jockers[0] {
        5 => HandType::FiveKind,
        4 => {
            if map.get(&'J') == Some(&1) {
                HandType::FiveKind
            } else {
                HandType::FourKind
            }
        }
        3 => {
            if map.get(&'J') == Some(&2) {
                HandType::FiveKind
            } else if map.get(&'J') == Some(&1) {
                HandType::FourKind
            } else {
                match counts_no_jockers[1] {
                    2 => HandType::House,
                    1 => HandType::ThreeKind,
                    _ => unreachable!(),
                }
            }
        }
        2 => {
            if map.get(&'J') == Some(&3) {
                HandType::FiveKind
            } else if map.get(&'J') == Some(&2) {
                HandType::FourKind
            } else {
                match counts_no_jockers[1] {
                    2 => {
                        if map.get(&'J') == Some(&1) {
                            HandType::House
                        } else {
                            HandType::TwoPair
                        }
                    }
                    1 => {
                        if map.get(&'J') == Some(&1) {
                            HandType::ThreeKind // here was the bug not TwoPairs
                        } else {
                            HandType::OnePair
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        1 => match map.get(&'J') {
            Some(&5) => unreachable!(),
            Some(&4) => HandType::FiveKind,
            Some(&3) => HandType::FourKind,
            Some(&2) => HandType::ThreeKind,
            Some(&1) => HandType::OnePair,
            None => HandType::High,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        "32T3K".parse::<ParsedHand>().unwrap().hand(Rules::Part1),
        Hand {
            cards: vec!['3', '2', 'T', '3', 'K'],
            hand_type: HandType::OnePair,
            rules: Rules::Part1
        }
    );
    assert_eq!(
        "T55J5".parse::<ParsedHand>().unwrap().hand(Rules::Part1),
        Hand {
            cards: vec!['T', '5', '5', 'J', '5'],
            hand_type: HandType::ThreeKind,
            rules: Rules::Part1
        }
    );
    assert_eq!(
        "T55J5".parse::<ParsedHand>().unwrap().hand(Rules::Part2),
        Hand {
            cards: vec!['T', '5', '5', 'J', '5'],
            hand_type: HandType::FourKind,
            rules: Rules::Part2
        }
    );
    assert_eq!(
        "J2TK7".parse::<ParsedHand>().unwrap().hand(Rules::Part2),
        Hand {
            cards: vec!['J', '2', 'T', 'K', '7'],
            hand_type: HandType::OnePair,
            rules: Rules::Part2
        }
    );
    assert_eq!(
        "QJJQ2".parse::<ParsedHand>().unwrap().hand(Rules::Part2),
        Hand {
            cards: vec!['Q', 'J', 'J', 'Q', '2'],
            hand_type: HandType::FourKind,
            rules: Rules::Part2
        }
    );
    assert_eq!(
        "233J4".parse::<ParsedHand>().unwrap().hand(Rules::Part2),
        Hand {
            cards: vec!['2', '3', '3', 'J', '4'],
            hand_type: HandType::ThreeKind,
            rules: Rules::Part2
        }
    );
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert_eq!(self.rules, other.rules);
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                let card_list = match self.rules {
                    Rules::Part1 => CARDS_FACES_P1,
                    Rules::Part2 => CARDS_FACES_P2,
                };
                if a == b {
                    continue;
                }
                let pa = card_list.iter().position(|c| c == a).unwrap();
                let pb = card_list.iter().position(|c| c == b).unwrap();
                return pa.cmp(&pb);
            }
        }
        // should not happen within this context
        unreachable!();
        // std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_cmp() {
    assert!(
        "32T3K".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
            < "T55J5".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
    );
    assert!(
        "33332".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
            > "2AAAA".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
    );
    assert!(
        "77888".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
            > "77788".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
    );
    assert!(
        "KK677".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
            > "KTJJT".parse::<ParsedHand>().unwrap().hand(Rules::Part1)
    );
    assert!(
        "QQQQ2".parse::<ParsedHand>().unwrap().hand(Rules::Part2)
            > "JKKK2".parse::<ParsedHand>().unwrap().hand(Rules::Part2)
    );
}

fn part1(input: &str) -> i64 {
    let mut hands: Vec<(Hand, i64)> = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let hand = parts
                .next()
                .unwrap()
                .parse::<ParsedHand>()
                .unwrap()
                .hand(Rules::Part1);
            let bet = parts.next().unwrap().parse::<i64>().unwrap();
            (hand, bet)
        })
        .collect::<Vec<_>>();
    hands.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    // dbg!(&hands);
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) as i64 * bet)
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut hands: Vec<(Hand, i64)> = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let hand = parts
                .next()
                .unwrap()
                .parse::<ParsedHand>()
                .unwrap()
                .hand(Rules::Part2);
            let bet = parts.next().unwrap().parse::<i64>().unwrap();
            (hand, bet)
        })
        .collect::<Vec<_>>();
    hands.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    // dbg!(&hands);
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bet))| (rank + 1) as i64 * bet)
        .sum()
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
        assert_eq!(part2(TEST_INPUT), 5905);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part1(INPUT), 246409899);
    }
    #[test]
    fn test_real_part2() {
        assert_ne!(part2(INPUT), 244990514);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
