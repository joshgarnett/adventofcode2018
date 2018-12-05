use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> Vec<i32> {
    // read all lines and then sort them
    let mut chars: Vec<i32> = Vec::new();
    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(file).lines() {
        for c in line.unwrap().chars() {
            chars.push(c as i32);
        }
    }
    chars
}

fn react(chars: &mut Vec<i32>) {
    loop {
        let mut index = 0;
        let start_size = chars.len();
        while index < chars.len() - 1 {
            if (chars[index] - chars[index + 1]).abs() == 32 {
                chars.remove(index);
                chars.remove(index);
            } else {
                index += 1;
            }
        }
        if start_size == chars.len() {
            break;
        }
    }
}

pub fn part1(filename: &str) -> usize {
    let mut chars = parse_file(filename);
    react(&mut chars);
    chars.len()
}

pub fn part2(filename: &str) -> usize {
    let data = parse_file(filename);
    let mut min_length = data.len();
    for i in 65..91 {
        let mut chars = data.clone();
        let mut index = 0;
        while index < chars.len() {
            if chars[index] == i || chars[index] == i + 32 {
                chars.remove(index);
            } else {
                index += 1;
            }
        }

        react(&mut chars);

        if chars.len() < min_length {
            min_length = chars.len();
        }
    }
    min_length
}