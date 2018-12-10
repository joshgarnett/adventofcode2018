use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug)]
struct Light {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

impl Light {
    fn step(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
    }

    fn back(&mut self) {
        self.x -= self.v_x;
        self.y -= self.v_y;
    }
}

fn parse_file(filename: &str) -> Vec<Light> {
    let mut lights = Vec::new();
    let file = File::open(filename).expect("file not found");
    // file format: position=<-50948,  20587> velocity=< 5, -2>
    let regex = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").expect("bad regex");
    for line in BufReader::new(file).lines() {
        let data = line.unwrap();
        let caps = match regex.captures(&data) {
            None => panic!("line {} does not match regex", data),
            Some(c) => c,
        };

        let light = Light {
            x: caps[1].parse::<i32>().expect("not an int"),
            y: caps[2].parse::<i32>().expect("not an int"),
            v_x: caps[3].parse::<i32>().expect("not an int"),
            v_y: caps[4].parse::<i32>().expect("not an int"),
        };
        lights.push(light);
    }

    lights
}

fn bounds(lights: &Vec<Light>) -> (i32, i32, i32, i32) {
    let mut minx = lights[0].x;
    let mut miny = lights[0].y;
    let mut maxx = lights[0].x;
    let mut maxy = lights[0].y;
    for l in lights {
        if l.x < minx {
            minx = l.x;
        }
        if l.y < miny {
            miny = l.y;
        }
        if l.x > maxx {
            maxx = l.x;
        }
        if l.y > maxy {
            maxy = l.y;
        }
    }
    (minx, miny, maxx, maxy)
}

fn print_lights(lights: &Vec<Light>) {
    let (minx, miny, maxx, maxy) = bounds(&lights);

    let width = (maxx + 1 - minx) as usize;
    let height = (maxy + 1 - miny) as usize;

    let mut points: Vec<char> = vec!['.'; width * height];

    for l in lights {
        let x = (l.x - minx) as usize;
        let y = (l.y - miny) as usize;
        let i: usize = y * width + x;
        points[i] = '#';
    }

    let mut out = String::new();
    for y in 0..height {
        for x in 0..width {
            let i: usize = y * width + x;
            out.push(points[i]);
        }
        out.push('\n');
    }

    println!("{}", &out);
}

pub fn part1_and_2(filename: &str) -> usize {
    let mut lights = parse_file(filename);

    let mut count = 0;
    let mut last_diffx = 0;
    let mut last_diffy = 0;

    // find where the lights are closest to each other
    loop {
        count += 1;
        for l in &mut lights {
            l.step();
        }
        let (minx, miny, maxx, maxy) = bounds(&lights);
        let diffx = maxx - minx;
        let diffy = maxy - miny;

        if last_diffx != 0 && diffx > last_diffx {
            break;
        }
        if last_diffy != 0 && diffy > last_diffy {
            break;
        }

        last_diffx = diffx;
        last_diffy = diffy;
    }

    // step back once to get correct image
    count -= 1;
    for l in &mut lights {
        l.back();
    }

    println!("Seconds: {}", count);
    print_lights(&lights);

    count
}
