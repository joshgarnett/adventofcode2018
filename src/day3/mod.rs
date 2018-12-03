use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

const FABRIC_SIZE: usize = 1000;

#[derive(Debug)]
struct Rect {
    w: usize,
    h: usize,
}

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    size: Rect,
}

fn parse_claims(filename: &str) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();
    let file = File::open(filename).expect("file not found");
    // line format: #1107 @ 509,248: 27x11
    let claim_regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect("bad regex");
    for line in BufReader::new(file).lines() {
        let data = line.unwrap();
        let caps = claim_regex.captures(&data).expect("line does not match regex");
        let claim = Claim {
            id: caps[1].parse::<usize>().expect("not an int"),
            x: caps[2].parse::<usize>().expect("not an int"),
            y: caps[3].parse::<usize>().expect("not an int"),
            size: Rect {
                w: caps[4].parse::<usize>().expect("not an int"),
                h: caps[5].parse::<usize>().expect("not an int"),
            },
        };
        claims.push(claim);
    }

    claims
}

pub fn part1(filename: &str) -> usize {
    let claims = parse_claims(filename);
    let mut fabric = [0; FABRIC_SIZE * FABRIC_SIZE];
    let mut total = 0;

    for claim in claims {
        for x in claim.x..(claim.x + claim.size.w) {
            for y in claim.y..(claim.y + claim.size.h) {
                let index = x + y * FABRIC_SIZE;
                fabric[index] += 1;
                if fabric[index] == 2 {
                    total += 1;
                }
            }
        }
    }
    total
}

pub fn part2(filename: &str) -> usize {
    let claims = parse_claims(filename);
    let mut fabric = [0; FABRIC_SIZE * FABRIC_SIZE];

    // claim sections in fabric
    for claim in &claims {
        for x in claim.x..(claim.x + claim.size.w) {
            for y in claim.y..(claim.y + claim.size.h) {
                fabric[x + y * FABRIC_SIZE] += 1;
            }
        }
    }

    // find claim that is only claimed once
    for claim in &claims {
        let mut valid = true;

        for x in claim.x..(claim.x + claim.size.w) {
            for y in claim.y..(claim.y + claim.size.h) {
                if fabric[x + y * FABRIC_SIZE] != 1 {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }
        if valid {
            return claim.id;
        }
    }
    unreachable!("result not found");
}