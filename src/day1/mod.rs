use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};

pub fn part1(filename: &str) -> i32 {
    let file = File::open(filename).expect("file not found");
    let mut total = 0;
    for line in BufReader::new(file).lines() {
        total = total + line.unwrap().parse::<i32>().unwrap();
    }
    return total;
}

pub fn part2(filename: &str) -> i32 {
    // parse the data into a vector of i32s
    let file = File::open(filename).expect("file not found");
    let mut values: Vec<i32> = Vec::new();
    for line in BufReader::new(file).lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        values.push(value);
    }

    // find duplicate frequency
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut total = 0;
    let mut found = false;
    while !found {
        for value in &values {
            total = total + value;
            if frequencies.contains(&total) {
                found = true;
                break;
            } else {
                frequencies.insert(total);
            }
        }
    }
    return total;
}