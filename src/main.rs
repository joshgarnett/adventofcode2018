extern crate advent;

use advent::day1;
use advent::day2;
use advent::day3;
use advent::day4;
use std::time::Instant;

fn time<T>(name: &str, filename: &str, f: &Fn(&str) -> T) where T: std::fmt::Debug {
    let start = Instant::now();
    let result = f(filename);
    println!("{} - result: {:#?} time: {:#?}", name, result, start.elapsed());
}

fn main() {
    time("Day1 Part1", "data/day1-input.txt", &day1::part1);
    time("Day1 Part2", "data/day1-input.txt", &day1::part2);
    time("Day2 Part1", "data/day2-input.txt", &day2::part1);
    time("Day2 Part2", "data/day2-input.txt", &day2::part2);
    time("Day3 Part1", "data/day3-input.txt", &day3::part1);
    time("Day3 Part2", "data/day3-input.txt", &day3::part2);
    time("Day4 Part1", "data/day4-input.txt", &day4::part1);
    time("Day4 Part2", "data/day4-input.txt", &day4::part2);
}