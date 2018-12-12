use std::collections::HashMap;

use regex::Regex;

struct Pots {
    left: i32,
    state: Vec<u8>,
    rules: HashMap<u8, u8>,
}

impl Pots {
    pub fn process_generations(&mut self, count: usize) {
        for _ in 0..count {
            self.next_generation();
        }
    }

    pub fn next_generation(&mut self) {
        let mut new_state: Vec<u8> = Vec::new();
        let mut found_pot = false;
        for i in 0..self.state.len() + 4 {
            let v = from_vec_to_u8(i as i32 - 2, &self.state);

            let c = match self.rules.get(&v) {
                None => 0,
                Some(c) => *c,
            };
            // trim front
            if !found_pot && c == 0 {
                if i > 1 {
                    self.left += 1;
                }
                continue;
            }
            if i < 2 && c == 1 {
                self.left -= 1;
            }
            found_pot = true;
            new_state.push(c);
        }

        // clean up empty pots on end
        loop {
            let index = new_state.len() - 1;
            if new_state[index] == 0 {
                new_state.remove(index);
            } else {
                break;
            }
        }

        self.state = new_state;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut s = String::new();
        for v in &self.state {
            if *v == 1 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        println!("offset: {} state: {}", self.left, s);
    }

    pub fn sum(&self) -> i32 {
        let mut total = 0;
        for i in 0..self.state.len() {
            if self.state[i] == 1 {
                total += i as i32 + self.left;
            }
        }
        total
    }
}

fn from_vec_to_u8(index: i32, state: &Vec<u8>) -> u8 {
    let l = state.len() as i32;
    let pos_0 = if (index - 2) < 0 { 0 } else { state[(index - 2) as usize] };
    let pos_1 = if (index - 1) < 0 || (index - 1) >= l { 0 } else { state[(index - 1) as usize] };
    let pos_2 = if index < 0 || index >= l { 0 } else { state[index as usize] };
    let pos_3 = if (index + 1) < 0 || (index + 1) >= l { 0 } else { state[(index + 1) as usize] };
    let pos_4 = if (index + 2) >= l { 0 } else { state[(index + 2) as usize] };

    (pos_0 << 4) +
        (pos_1 << 3) +
        (pos_2 << 2) +
        (pos_3 << 1) +
        pos_4
}

fn from_str_to_u8(s: &str) -> u8 {
    let v = s.replace(".", "0").replace("#", "1");
    u8::from_str_radix(&v, 2).unwrap()
}

fn parse_state(state_s: &str, rules_s: &str) -> Pots {
    // parse state
    let mut state = Vec::new();
    for c in state_s.chars() {
        if c == '#' {
            state.push(1);
        } else {
            state.push(0);
        }
    }

    // parse rules
    let mut rules: HashMap<u8, u8> = HashMap::new();
    let regex = Regex::new(r"(.*) => (.*)").expect("bad regex");

    for line in rules_s.lines() {
        let caps = match regex.captures(&line) {
            None => panic!("line {} does not match regex", line),
            Some(c) => c,
        };
        let a = from_str_to_u8(&caps[1]);
        let b = caps[2].parse::<char>().expect("not a char");
        if b == '#' {
            rules.insert(a, 1);
        } else {
            rules.insert(a, 0);
        }
    }

    Pots {
        left: 0,
        state: state,
        rules: rules,
    }
}

pub fn part1() -> i32 {
    let mut pots = parse_state(PART1_STATE, PART1_RULES);
    pots.process_generations(20);
    pots.sum()
}

pub fn part2() -> u128 {
    let mut pots = parse_state(PART1_STATE, PART1_RULES);
    let mut count = 0;
    let mut last = pots.sum();
    let mut last_diff = 0;

    // find generation where delta is the same as previous generation
    loop {
        count += 1;
        pots.next_generation();
        let s = pots.sum();
        let diff = s - last;
        if diff == last_diff {
            break;
        }
        last = s;
        last_diff = diff;
    }

    (50000000000 as u128 - count) * last_diff as u128 + pots.sum() as u128
}

const PART1_STATE: &str = "##..#.#.#..##..#..##..##..#.#....#.....##.#########...#.#..#..#....#.###.###....#..........###.#.#..";
const PART1_RULES: &str = r#"..##. => .
..... => .
##..# => .
...#. => .
#.... => .
...## => #
.#.#. => .
#..#. => #
##.#. => .
#..## => .
..#.. => .
#.#.# => .
###.# => .
###.. => .
.#... => #
.##.# => .
##... => #
..### => .
####. => .
#...# => #
.#..# => #
##### => #
..#.# => #
.#.## => #
#.### => .
....# => .
.###. => .
.#### => #
.##.. => .
##.## => #
#.##. => #
#.#.. => #"#;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STATE: &str = "#..#.#..##......###...###";
    const EXAMPLE_RULES: &str = r#"...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"#;

    #[test]
    fn example1() {
        let mut pots = parse_state(EXAMPLE_STATE, EXAMPLE_RULES);
        pots.process_generations(20);
        assert_eq!(325, pots.sum());
    }
}