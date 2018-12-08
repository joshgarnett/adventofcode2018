use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_points(filename: &str) -> Vec<Point> {
    // read all lines and then sort them
    let mut points: Vec<Point> = Vec::new();
    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(file).lines() {
        let data = line.unwrap();
        let parts: Vec<&str> = data.split(", ").collect();
        points.push(Point {
            x: parts[0].parse::<i32>().expect("not an int"),
            y: parts[1].parse::<i32>().expect("not an int"),
        });
    }

    points
}

fn find_bounds(points: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut min_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_x = points[0].x;
    let mut max_y = points[0].y;
    for p in points {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }
    (min_x, min_y, max_x, max_y)
}

fn calculate_manhattan(points: &Vec<Point>, min_x:i32, min_y:i32, max_x:i32, max_y:i32) -> Vec<usize> {
    let mut spots: Vec<usize> = vec!(0; points.len());

    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            let mut closest_distance = 100000;
            let mut closest_point: usize = 1000000;
            let mut closest_count = 0;

            for i in 0..points.len() {
                let p = &points[i];

                let point_distance = (p.x - x).abs() + (p.y - y).abs();
                if point_distance < closest_distance {
                    closest_distance = point_distance;
                    closest_point = i;
                    closest_count = 1;
                } else if point_distance == closest_distance {
                    closest_count += 1;
                }
            }

            if closest_count == 1 {
                spots[closest_point] += 1;
            }
        }
    }

    spots
}

pub fn part1(filename: &str) -> usize {
    let points = parse_points(filename);

    let (min_x, min_y, max_x, max_y) = find_bounds(&points);
    let spots = calculate_manhattan(&points, min_x, min_y, max_x, max_y);
    let spots_larger = calculate_manhattan(&points, min_x+10, min_y+10, max_x+10, max_y+10);

    let mut largest = 0;
    for i in 0..spots.len() {
        // throw away any points that saw their size increase when the bounds increased
        if spots[i] != spots_larger[i] {
            continue;
        }
        if spots[i] > largest {
            largest = spots[i];
        }
    }

    largest
}

pub fn part2(filename: &str) -> usize {
    let points = parse_points(filename);

    let (min_x, min_y, max_x, max_y) = find_bounds(&points);

    let mut region_size = 0;

    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            let mut total_distance = 0;
            for i in 0..points.len() {
                let p = &points[i];
                let point_distance = (p.x - x).abs() + (p.y - y).abs();
                total_distance += point_distance;
            }
            if total_distance < 10000 {
                region_size += 1;
            }
        }
    }

    region_size
}