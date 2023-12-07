use std::{
    cmp::{max, min},
    ops::RangeInclusive,
    str::FromStr,
};

const INPUT: &str = include_str!("day05/input.txt");

#[derive(Debug, PartialEq, Eq)]
struct MapLine {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

type Map = Vec<MapLine>;

impl FromStr for MapLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        Ok(MapLine {
            dest_start: parts.next().unwrap().parse::<i64>().unwrap(),
            source_start: parts.next().unwrap().parse::<i64>().unwrap(),
            len: parts.next().unwrap().parse::<i64>().unwrap(),
        })
    }
}

#[test]
fn test_map_parse() {
    assert_eq!(
        "50 98 2".parse(),
        Ok(MapLine {
            source_start: 98,
            dest_start: 50,
            len: 2
        })
    );
}

impl MapLine {
    fn source_range(&self) -> RangeInclusive<i64> {
        self.source_start..=(self.source_start + self.len - 1)
    }
    fn offset(&self) -> i64 {
        self.dest_start - self.source_start
    }
}

type SeedsRange = RangeInclusive<i64>;

fn parse_input(input: &str, part2: bool) -> (Vec<SeedsRange>, Vec<Map>) {
    // Assume maps are in the proper order
    let mut lines = input.lines();
    let seed_line = lines.next().unwrap();
    let mut parts = seed_line.split(':');
    assert_eq!(parts.next().unwrap(), "seeds");
    let seeds = if !part2 {
        parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|i| (i.parse().unwrap()..=i.parse().unwrap()))
            .collect::<Vec<_>>()
    } else {
        parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|r| (r[0]..=r[0] + r[1] - 1))
            .collect::<Vec<_>>()
    };
    let mut maps = Vec::new();
    let mut current_map: Option<Vec<MapLine>> = None;
    for line in lines {
        if line.ends_with(':') {
            if let Some(map) = current_map {
                maps.push(map);
                current_map = None;
            }
        } else if line.is_empty() {
            continue;
        } else {
            current_map
                .get_or_insert(Vec::new())
                .push(line.parse().unwrap());
        }
    }
    if let Some(map) = current_map {
        maps.push(map);
    }
    (seeds, maps)
}

fn offset_range(range: SeedsRange, offset: i64) -> SeedsRange {
    SeedsRange::new(range.start() + offset, range.end() + offset)
}

fn intersect(source: SeedsRange, map: &MapLine) -> Option<SeedsRange> {
    let intersection = max(*source.start(), *map.source_range().start())
        ..=min(*source.end(), *map.source_range().end());
    if !intersection.is_empty() {
        Some(intersection.clone())
    } else {
        None
    }
}

fn apply_map(source: SeedsRange, map: &Map) -> Vec<SeedsRange> {
    let mut ret = Vec::new();
    let mut remaining_seeds = vec![source];
    for m in map.iter() {
        let current_seeds = remaining_seeds.clone();
        remaining_seeds.clear();
        for s in current_seeds {
            if let Some(intersection) = intersect(s.clone(), m) {
                ret.push(offset_range(intersection.clone(), m.offset()));
                let exclusions = vec![
                    *s.start()..=(intersection.start() - 1),
                    (*intersection.end() + 1)..=*s.end(),
                ];
                for e in exclusions {
                    if !e.is_empty() {
                        remaining_seeds.push(e);
                    }
                }
            } else {
                remaining_seeds.push(s);
            }
        }
    }
    ret.append(&mut remaining_seeds);

    // TODO: Merge back ?

    ret
}

#[test]
fn test_apply_map() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48";
    let (seeds, maps) = parse_input(input, false);
    assert_eq!(apply_map(seeds[0].clone(), &maps[0]), vec![81..=81]);
    assert_eq!(apply_map(seeds[1].clone(), &maps[0]), vec![14..=14]);
    assert_eq!(apply_map(seeds[2].clone(), &maps[0]), vec![57..=57]);
    assert_eq!(apply_map(seeds[3].clone(), &maps[0]), vec![13..=13]);
    let (seeds, maps) = parse_input(input, true);
    assert_eq!(apply_map(seeds[0].clone(), &maps[0]), vec![81..=94]);
    assert_eq!(apply_map(seeds[1].clone(), &maps[0]), vec![57..=69]);
}

fn part1(input: &str) -> i64 {
    let (mut seeds, maps) = parse_input(input, false);
    for m in maps.iter() {
        let mut next_seeds = Vec::new();
        for s in seeds.drain(..) {
            next_seeds.append(&mut apply_map(s, m));
        }
        seeds = next_seeds;
    }
    seeds.into_iter().map(|s| *s.start()).min().unwrap()
}

fn part2(input: &str) -> i64 {
    let (mut seeds, maps) = parse_input(input, true);
    for m in maps.iter() {
        let mut next_seeds = Vec::new();
        for s in seeds.drain(..) {
            next_seeds.append(&mut apply_map(s, m));
        }
        seeds = next_seeds;
    }
    seeds.into_iter().map(|s| *s.start()).min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const test_input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse1() {
        assert_eq!(part1(test_input), 35);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(test_input), 46);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
