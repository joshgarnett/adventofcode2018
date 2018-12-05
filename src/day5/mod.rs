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

fn react(input: &Vec<i32>, skip: i32) -> usize {
    let size = input.len();
    let mut stack = vec![0; size];
    let mut size = 0;

    for unit in input {
        if *unit == skip || *unit == skip + 32 {
            continue;
        }
        if size == 0 {
            stack[size] = *unit;
            size += 1;
        } else if (stack[stack.len() - 1] - unit).abs() == 32 {
            size -= 1;
        } else {
            stack[size] = *unit;
            size += 1;
        }
    }
    size
}

pub fn part1(filename: &str) -> usize {
    let chars = parse_file(filename);
    react(&chars, 0)
}

pub fn part2(filename: &str) -> usize {
    let data = parse_file(filename);
    let mut min_length = data.len();
    for i in 65..91 {
        let l = react(&data, i);
        ;

        if l < min_length {
            min_length = l;
        }
    }
    min_length
}
