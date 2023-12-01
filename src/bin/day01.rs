const INPUT: &str = include_str!("day01/input.txt");


fn part1() -> u32 {
    INPUT.lines().map(|l| {
        let d1 = l.chars().find(|c| c.is_numeric()).unwrap().to_digit(10).unwrap();
        let d2 = l.chars().rev().find(|c| c.is_numeric()).unwrap().to_digit(10).unwrap();
        d1 * 10 + d2
    }).sum()
}

fn part2() -> u32 {
    
}

fn main() {

    println!("part1 {}", part1());
}