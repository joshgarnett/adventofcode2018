use std::cmp::Ordering;

use rayon::prelude::*;

#[derive(Debug)]
pub struct Answer {
    x: usize,
    y: usize,
    size: usize,
    power: i32,
}

impl PartialOrd for Answer {
    fn partial_cmp(&self, other: &Answer) -> Option<Ordering> {
        Some(self.power.cmp(&other.power))
    }
}

impl Ord for Answer {
    fn cmp(&self, other: &Answer) -> Ordering {
        self.power.cmp(&other.power)
    }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Answer) -> bool {
        self.power == other.power
    }
}

impl Eq for Answer {}

impl Answer {
    pub fn new(x: usize, y: usize, size: usize, power: i32) -> Answer {
        Answer { x, y, size, power }
    }

    pub fn empty() -> Answer {
        Answer { x: 0, y: 0, size: 0, power: 0 }
    }
}

const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 300;

fn calculate_power(x: usize, y: usize, serial: usize) -> i32 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = x + 10;
    // Begin with a power level of the rack ID times the Y coordinate.
    let mut power = rack_id * y;
    // Increase the power level by the value of the grid serial number (your puzzle input).
    power += serial;
    // Set the power level to itself multiplied by the rack ID.
    power = power * rack_id;
    // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    power = (power % 1000) / 100;
    // Subtract 5 from the power level.
    power as i32 - 5
}

fn generate_summed_area_grid(serial: usize) -> [i32; GRID_WIDTH * GRID_HEIGHT] {
    let mut points = [0; GRID_WIDTH * GRID_HEIGHT];
    for y in 1..=GRID_HEIGHT {
        for x in 1..=GRID_HEIGHT {
            let index = (y - 1) * GRID_WIDTH + (x - 1);
            points[index] = calculate_power(x, y, serial);
        }
    }

    let mut summed_area = [0; GRID_WIDTH * GRID_HEIGHT];
    for y in 0..GRID_WIDTH {
        let index = y * GRID_WIDTH;
        summed_area[index] = points[index];
    }

    for x in 1..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let index = y * GRID_WIDTH + x;
            summed_area[index] = points[index] + summed_area[index - 1];
        }
    }

    for x in 0..GRID_WIDTH {
        for y in 1..GRID_HEIGHT {
            let index = y * GRID_WIDTH + x;
            let previous_index = (y - 1) * GRID_WIDTH + x;
            summed_area[index] = summed_area[index] + summed_area[previous_index];
        }
    }

    summed_area
}

fn find_largest_3x3(serial: usize) -> Answer {
    let points = generate_summed_area_grid(serial);

    largest_square(3, &points)
}

#[allow(dead_code)]
fn find_largest_any_size(serial: usize) -> Answer {
    let points = generate_summed_area_grid(serial);

    (2..GRID_WIDTH).map(|s| largest_square(s, &points)).max().unwrap()
}

fn find_largest_any_size_parallel(serial: usize) -> Answer {
    let points = generate_summed_area_grid(serial);

    (2..GRID_WIDTH).into_par_iter().map(|s| largest_square(s, &points)).max().unwrap()
}

fn largest_square(size: usize, points: &[i32; GRID_WIDTH * GRID_HEIGHT]) -> Answer {
    let mut answer = Answer::empty();
    answer.size = size;

    for y in 2..=GRID_HEIGHT - (size - 1) {
        for x in 2..=GRID_HEIGHT - (size - 1) {
            let power = sum_square(size, x - 1, y - 1, &points);
            if power > answer.power {
                answer.power = power;
                answer.x = x;
                answer.y = y;
            }
        }
    }

    answer
}

fn sum_square(size: usize, left_x: usize, left_y: usize, points: &[i32; GRID_WIDTH * GRID_HEIGHT]) -> i32 {
    let left = left_x - 1;
    let right = left_x + size - 1;
    let top = left_y - 1;
    let bottom = left_y + size - 1;

    let left_top_index = top * GRID_WIDTH + left;
    let right_top_index = top * GRID_WIDTH + right;
    let left_bottom_index = bottom * GRID_WIDTH + left;
    let right_bottom_index = bottom * GRID_WIDTH + right;

    points[right_bottom_index] - points[right_top_index] - points[left_bottom_index] + points[left_top_index]
}

pub fn part1() -> Answer {
    find_largest_3x3(8199)
}

pub fn part2() -> Answer {
    find_largest_any_size_parallel(8199)
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn find_largest_any_size_bench(b: &mut Bencher) {
        b.iter(|| find_largest_any_size(18));
    }

    #[bench]
    fn find_largest_any_size_parallel_bench(b: &mut Bencher) {
        b.iter(|| find_largest_any_size_parallel(18));
    }

    #[test]
    fn calculate_power_example1() {
        assert_eq!(-5, calculate_power(122, 79, 57));
    }

    #[test]
    fn calculate_power_example2() {
        assert_eq!(0, calculate_power(217, 196, 39));
    }

    #[test]
    fn calculate_power_example3() {
        assert_eq!(4, calculate_power(101, 153, 71));
    }

    #[test]
    fn find_largest_3x3_example1() {
        assert_eq!(Answer::new(33, 45, 3, 29), find_largest_3x3(18));
    }

    #[test]
    fn find_largest_3x3_example2() {
        assert_eq!(Answer::new(21, 61, 3, 30), find_largest_3x3(42));
    }

    #[test]
    fn find_largest_any_size_example1() {
        assert_eq!(Answer::new(90, 269, 16, 113), find_largest_any_size_parallel(18));
    }

    #[test]
    fn find_largest_any_size_example2() {
        assert_eq!(Answer::new(232, 251, 12, 119), find_largest_any_size_parallel(42));
    }
}
