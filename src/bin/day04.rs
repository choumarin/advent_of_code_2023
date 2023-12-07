use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("day04/input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: i32,
    have: HashSet<i32>,
    win: HashSet<i32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts1 = s.split('|').collect::<Vec<_>>();
        let parts2 = parts1[0].split(':').collect::<Vec<_>>();
        let parts3 = parts2[0].split_whitespace().collect::<Vec<_>>();
        assert_eq!(parts3[0], "Card");
        Ok(Card {
            id: i32::from_str(parts3[1]).unwrap(),
            have: parts2[1]
                .split_whitespace()
                .map(|i| i32::from_str(i).unwrap())
                .collect(),
            win: parts1[1]
                .split_whitespace()
                .map(|i| i32::from_str(i).unwrap())
                .collect(),
        })
    }
}

impl Card {
    fn matching(&self) -> i32 {
        let mut p: i32 = 0;
        for &h in self.have.iter() {
            if self.win.contains(&h) {
                p += 1;
            }
        }
        p
    }

    fn points(&self) -> i32 {
        let mut p: i32 = 0;
        for &h in self.have.iter() {
            if self.win.contains(&h) {
                if p == 0 {
                    p = 1;
                } else {
                    p *= 2;
                }
            }
        }
        p
    }
}

#[test]
fn test_parse_card() {
    let s = "Card 1: 41 48 17 | 83 86";
    assert_eq!(
        Card::from_str(s),
        Ok(Card {
            id: 1,
            have: HashSet::from([41, 48, 17]),
            win: HashSet::from([83, 86]),
        })
    )
}

#[test]
fn test_points() {
    let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    assert_eq!(Card::from_str(s).unwrap().points(), 8)
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| Card::from_str(l).unwrap().points())
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut all_cards = input
        .lines()
        .map(|l| {
            let c = Card::from_str(l).unwrap();
            (c.id, (c, 1))
        })
        .collect::<BTreeMap<_, _>>();
    // for (&id, (c, _)) in all_cards.iter() {
    //     for i in (id + 1)..(id + c.points()) {
    //         all_cards.get_mut(&i).unwrap().1 += 1;
    //     }
    // }
    for id in all_cards.keys().copied().collect::<Vec<_>>() {
        let matching = all_cards.get(&id).unwrap().0.matching();
        if matching == 0 {
            continue;
        }
        let copies = all_cards.get(&id).unwrap().1;
        for i in (id + 1)..(id + matching + 1) {
            all_cards.get_mut(&i).unwrap().1 += copies;
        }
    }
    // dbg!(&all_cards);
    all_cards.iter().map(|(_, (_, i))| i).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
