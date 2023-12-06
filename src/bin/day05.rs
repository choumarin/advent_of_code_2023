use std::str::FromStr;

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
            source_start: 50,
            dest_start: 98,
            len: 2
        })
    );
}

type Seeds = Vec<i64>;

fn parse_input(input: &str) -> (Seeds, Vec<Map>) {
    // Assume maps are in the proper order
    let mut lines = input.lines();
    let seed_line = lines.next().unwrap();
    let mut parts = seed_line.split(':');
    assert_eq!(parts.next().unwrap(), "seeds");
    let seeds = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

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

fn apply_maps(source: i64, map: &Map) -> i64 {
    for m in map {
        if m.source_start <= source && source < m.source_start + m.len {
            return source - m.source_start + m.dest_start;
        }
    }
    source
}

#[test]
fn test_apply_map() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48";
    let (_, maps) = parse_input(input);
    dbg!(&maps);
    assert_eq!(apply_maps(79, &maps[0]), 81);
    assert_eq!(apply_maps(1, &maps[0]), 1);
}

fn part1(input: &str) -> i64 {
    let (mut seeds, maps) = parse_input(input);
    for s in seeds.iter_mut() {
        for m in maps.iter() {
            *s = apply_maps(*s, m);
        }
    }
    seeds.into_iter().min().unwrap()
}

fn part2(input: &str) -> i64 {
    unimplemented!()
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
        assert_eq!(part2(test_input), 30);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
