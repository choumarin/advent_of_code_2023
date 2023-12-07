const INPUT: &str = include_str!("day06/input.txt");

struct Race {
    time: i64,
    distance: i64,
}

fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let mut times_parts = lines.next().unwrap().split(':');
    assert_eq!(times_parts.next().unwrap(), "Time");
    let times = times_parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let mut dist_parts = lines.next().unwrap().split(':');
    assert_eq!(dist_parts.next().unwrap(), "Distance");
    let dists = dist_parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    times
        .into_iter()
        .zip(dists)
        .map(|(t, d)| Race {
            time: t,
            distance: d,
        })
        .collect()
}

impl Race {
    fn roots(&self) -> (f64, f64) {
        (
            (self.time as f64 - f64::sqrt((self.time * self.time - 4 * self.distance) as f64))
                / 2.0,
            (self.time as f64 + f64::sqrt((self.time * self.time - 4 * self.distance) as f64))
                / 2.0,
        )
    }

    fn possible_wins(&self) -> i64 {
        let b = self.roots().1;
        let a = self.roots().0;
        let b = if b.floor() == b {
            (b - 1.0) as i64
        } else {
            b.floor() as i64
        };
        let a = if a.ceil() == a {
            (a + 1.0) as i64
        } else {
            a.ceil() as i64
        };
        b - a + 1
    }
}

// dist(tb, tt) = (tt - tb) * tb - m = -tb^2 + tt*tb - m

// tb = (tt +/- sqrt(tt^2 - 4*m)) / 2

#[test]
fn test_roots() {
    assert_eq!(
        Race {
            time: 7,
            distance: 9
        }
        .possible_wins(),
        4
    );
    assert_eq!(
        Race {
            time: 15,
            distance: 40
        }
        .possible_wins(),
        8
    );
    assert_eq!(
        Race {
            time: 30,
            distance: 200
        }
        .possible_wins(),
        9
    );
}

fn part1(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|r| r.possible_wins())
        .product()
}

fn part2(input: &str) -> i64 {
    let fold = parse(input)
        .into_iter()
        .map(|r| (r.time.to_string(), r.distance.to_string()))
        .fold((String::new(), String::new()), |(at, ad), (t, d)| {
            (at + &t, ad + &d)
        });
    Race {
        time: fold.0.parse().unwrap(),
        distance: fold.1.parse().unwrap(),
    }
    .possible_wins()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_parse1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
}

fn main() {
    println!("part1 {}", part1(INPUT));
    println!("part2 {}", part2(INPUT));
}
