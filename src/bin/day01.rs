use std::collections::HashMap;

const INPUT: &str = include_str!("day01/input.txt");

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            first_num(l.to_owned(), false, false).unwrap() * 10 +
            first_num(l.to_owned(), false, true).unwrap()
        })
        .sum()
}

fn first_num(mut s: String, with_letters: bool, reverse: bool) -> Option<u32> {
    let mut nums = HashMap::from([
        (String::from("1"), 1),
        (String::from("2"), 2),
        (String::from("3"), 3),
        (String::from("4"), 4),
        (String::from("5"), 5),
        (String::from("6"), 6),
        (String::from("7"), 7),
        (String::from("8"), 8),
        (String::from("9"), 9),
    ]);

    if with_letters {
        nums.insert(String::from("one"), 1);
        nums.insert(String::from("two"), 2);
        nums.insert(String::from("three"), 3);
        nums.insert(String::from("four"), 4);
        nums.insert(String::from("five"), 5);
        nums.insert(String::from("six"), 6);
        nums.insert(String::from("seven"), 7);
        nums.insert(String::from("eight"), 8);
        nums.insert(String::from("nine"), 9);
    }

    if reverse {
        s = s.chars().rev().collect::<String>();
        let mut t = HashMap::new();
        for (k, v) in nums.drain() {
            t.insert(k.chars().rev().collect::<String>(), v);
        }
        for (k, v) in t.drain() {
            nums.insert(k, v);
        }
    }

    let mut first: Option<(usize, u32)> = None;
    for (n_s, n_i) in nums.iter() {
        if let Some(p) = s.find(n_s) {
            if first.is_none() {
                first = Some((p, *n_i));
            } else {
                if first.unwrap().0 > p {
                    first = Some((p, *n_i));
                }
            }
        }
    }

    if let Some(f) = first {
        return Some(f.1);
    }
    return None;
}

#[test]
fn test_first_num() {
    assert_eq!(first_num(String::from("1"), false, false), Some(1));
    assert_eq!(first_num(String::from("1"), false, true), Some(1));
    assert_eq!(first_num(String::from("12"), false, false), Some(1));
    assert_eq!(first_num(String::from("12"), false, true), Some(2));
    assert_eq!(first_num(String::from("three12"), false, true), Some(2));
    assert_eq!(first_num(String::from("three12"), true, true), Some(2));
    assert_eq!(first_num(String::from("three12"), true, false), Some(3));
    assert_eq!(
        first_num(String::from("qsnvrckf4mjnoneightlgt"), true, false),
        Some(4)
    );
    assert_eq!(
        first_num(String::from("qsnvrckf4mjnoneightlgt"), true, true),
        Some(8)
    );
    assert_eq!(first_num(String::from("goiydabsyfvuse"), true, true), None);
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            first_num(l.to_owned(), true, false).unwrap() * 10 +
            first_num(l.to_owned(), true, true).unwrap()
        })
        .sum()
}

mod test {
    use super::*;

    #[test]
    fn test_parse1() {
        let test_input_1 = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(part1(test_input_1), 142);
    }

    #[test]
    fn test_parse2() {
        let test_input_2 = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(part2(test_input_2), 281);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
