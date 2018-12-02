use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(filename: &str) -> i32 {
    let file = File::open(filename).expect("file not found");

    let mut two_letters = 0;
    let mut three_letters = 0;
    let mut letters: HashMap<char, i32> = HashMap::new();
    for line in BufReader::new(file).lines() {
        for c in line.unwrap().chars() {
            let count = letters.entry(c).or_insert(0);
            *count += 1;
        }

        let mut found_two = false;
        let mut found_three = false;
        for val in letters.values() {
            if *val == 2 {
                found_two = true;
            } else if *val == 3 {
                found_three = true;
            }
        }

        if found_two {
            two_letters += 1;
        }
        if found_three {
            three_letters += 1;
        }

        letters.clear();
    }
    two_letters * three_letters
}

pub fn part2(filename: &str) -> String {
    // parse all lines into a vector of chars
    let file = File::open(filename).expect("file not found");
    let mut boxes: Vec<Vec<char>> = Vec::new();
    for line in BufReader::new(file).lines() {
        let mut box_chars: Vec<char> = Vec::new();
        for c in line.unwrap().chars() {
            box_chars.push(c);
        }
        boxes.push(box_chars);
    }

    // search for boxes that are the closest match (same letters in same position)
    let box_count = boxes.len();
    let char_count = boxes[0].len();
    let mut box_1 = 0;
    let mut box_2 = 0;
    let mut max_diff = std::i32::MAX;

    for index_1 in 0..(box_count - 1) {
        for index_2 in (index_1 + 1)..box_count {
            let mut diff = 0;
            for i in 0..char_count {
                if boxes[index_1][i] != boxes[index_2][i] {
                    diff += 1;
                }
            }
            if diff < max_diff {
                box_1 = index_1;
                box_2 = index_2;
                max_diff = diff;
            }
        }
    }

    let mut answer: Vec<char> = Vec::new();
    for i in 0..char_count {
        if boxes[box_1][i] == boxes[box_2][i] {
            answer.push(boxes[box_1][i]);
        }
    }

    answer.into_iter().collect()
}
