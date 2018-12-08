use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use regex::Regex;

const MAX_TIME: i32 = 2147483647;
const NO_WORK: char = '.';

#[derive(Debug)]
struct Step {
    id: char,
    dependencies: Vec<char>,
    time: i32,
}

fn parse_dependencies(filename: &str) -> HashMap<char, Step> {
    let mut steps: HashMap<char, Step> = HashMap::new();
    let file = File::open(filename).expect("file not found");
    // Format: Step G must be finished before step L can begin.
    let claim_regex = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").expect("bad regex");
    for line in BufReader::new(file).lines() {
        let data = line.unwrap();
        let caps = claim_regex.captures(&data).expect("line does not match regex");
        let step_before = caps[1].parse::<char>().expect("not a char");
        let step = caps[2].parse::<char>().expect("not a char");

        // ensure the before step exists
        steps.entry(step_before).or_insert(Step {
            id: step_before,
            dependencies: Vec::new(),
            time: (step_before as i32 - 64) + 60,
        });

        // add the before step to the specified step
        let dependency = steps.entry(step).or_insert(Step {
            id: step,
            dependencies: Vec::new(),
            time: (step as i32 - 64) + 60,
        });
        dependency.dependencies.push(step_before);
    }

    steps
}

fn remove_step(steps: &mut HashMap<char, Step>, remove: char) {
    for step in steps.values_mut() {
        let result = step.dependencies.iter().position(|c| *c == remove);
        match result {
            Some(i) => {
                step.dependencies.remove(i);
                ()
            }
            None => ()
        }
    }
}

pub fn part1(filename: &str) -> String {
    let mut steps = parse_dependencies(filename);
    let mut order: Vec<char> = Vec::new();

    while steps.len() > 0 {
        let mut valid_next: Vec<char> = Vec::new();

        for step in steps.values() {
            if step.dependencies.len() == 0 {
                valid_next.push(step.id);
            }
        }
        valid_next.sort();

        order.push(valid_next[0]);
        remove_step(&mut steps, valid_next[0]);
        steps.remove(&valid_next[0]);
    }

    String::from_iter(order)
}

pub fn part2(filename: &str) -> i32 {
    let mut steps = parse_dependencies(filename);
    let mut worker_complete_time = [0; 5];
    let mut worker_step = [NO_WORK; 5];
    let mut now = 0;
    while steps.len() > 0 {
        // check if any steps available
        let mut valid_next: Vec<char> = Vec::new();
        for step in steps.values() {
            if step.dependencies.len() == 0 && !worker_step.contains(&step.id) {
                valid_next.push(step.id);
            }
        }
        valid_next.sort();

        // attempt to assign work to non-busy workers
        for i in 0..worker_step.len() {
            if valid_next.len() > 0 && worker_step[i] == NO_WORK {
                let c = valid_next.remove(0);
                let step = steps.get(&c).unwrap();
                worker_complete_time[i] = now + step.time;
                worker_step[i] = c;
            }
        }

        // find the next worker that will complete their work
        let mut next_time = MAX_TIME;
        let mut next_complete_worker = 0;
        for i in 0..worker_step.len() {
            if worker_step[i] == NO_WORK {
                continue;
            }
            if worker_complete_time[i] < next_time {
                next_time = worker_complete_time[i];
                next_complete_worker = i;
            }
        }

        // adjust time
        now = worker_complete_time[next_complete_worker];

        // complete workers step
        remove_step(&mut steps, worker_step[next_complete_worker]);
        steps.remove(&worker_step[next_complete_worker]);
        worker_step[next_complete_worker] = NO_WORK;
        worker_complete_time[next_complete_worker] = 0;
    }
    now
}