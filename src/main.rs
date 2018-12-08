extern crate advent;

use advent::day1;
use advent::day2;
use advent::day3;
use advent::day4;
use advent::day5;
use advent::day6;
use advent::day7;
use advent::day8;
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
    time("Day5 Part1", "data/day5-input.txt", &day5::part1);
    time("Day5 Part2", "data/day5-input.txt", &day5::part2);
    time("Day6 Part1", "data/day6-input.txt", &day6::part1);
    time("Day6 Part2", "data/day6-input.txt", &day6::part2);
    time("Day7 Part1", "data/day7-input.txt", &day7::part1);
    time("Day7 Part2", "data/day7-input.txt", &day7::part2);
    time("Day8 Part1", "data/day8-input.txt", &day8::part1);
    time("Day8 Part2", "data/day8-input.txt", &day8::part2);
}