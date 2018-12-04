use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

struct Guard {
    id: usize,
    minutes: [usize; 60],
    total_minutes: usize,
}

impl Guard {
    fn add_time(&mut self, start_minute: usize, end_minute: usize) {
        self.total_minutes += end_minute - start_minute;
        for i in start_minute..end_minute {
            self.minutes[i] += 1;
        }
    }
}

/*
File Format:
[1518-11-23 00:00] Guard #2861 begins shift
[1518-11-23 00:29] falls asleep
[1518-11-23 00:44] wakes up
[1518-11-23 00:52] falls asleep
[1518-11-23 00:57] wakes up
*/
fn parse_guard_data(filename: &str) -> HashMap<usize, Guard> {
    // read all lines and then sort them
    let mut lines: Vec<String> = Vec::new();
    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }
    lines.sort();

    let mut guard_id: usize = 0;
    let mut guards: HashMap<usize, Guard> = HashMap::new();
    let mut awake = true;
    let mut sleep_minute: usize = 0;

    let guard_line = Regex::new(r"Guard #(\d+) begins").expect("bad regex");

    for line in lines {
        match guard_line.captures(&line) {
            Some(caps) => {
                guard_id = caps[1].parse::<usize>().expect("not an int");
                guards.entry(guard_id).or_insert(Guard {
                    id: guard_id,
                    minutes: [0; 60],
                    total_minutes: 0,
                });
            }
            None => {
                let minute = line[15..17].parse::<usize>().expect("not an int");
                if awake {
                    sleep_minute = minute;
                    awake = false;
                } else {
                    let g = guards.get_mut(&guard_id).unwrap();
                    g.add_time(sleep_minute, minute);
                    awake = true;
                }
            }
        }
    }

    guards
}

pub fn part1(filename: &str) -> usize {
    let guards = parse_guard_data(filename);

    let mut max_minutes: usize = 0;

    // find guard with the most minutes
    let mut guard_id: usize = 0;
    for g in guards.values() {
        if g.total_minutes > max_minutes {
            max_minutes = g.total_minutes;
            guard_id = g.id;
        }
    }

    // find minute guard is asleep the most often
    let guard = guards.get(&guard_id).expect("invalid guard id");
    max_minutes = 0;
    let mut best_time: usize = 0;
    for i in 0..guard.minutes.len() {
        if guard.minutes[i] > max_minutes {
            max_minutes = guard.minutes[i];
            best_time = i;
        }
    }
    guard.id * best_time
}

pub fn part2(filename: &str) -> usize {
    let guards = parse_guard_data(filename);
    let mut guard_id = 0;
    let mut best_time: usize = 0;
    let mut max_minutes: usize = 0;

    for (_, guard) in guards {
        for i in 0..guard.minutes.len() {
            if guard.minutes[i] > max_minutes {
                guard_id = guard.id;
                best_time = i;
                max_minutes = guard.minutes[i];
            }
        }
    }

    guard_id * best_time
}