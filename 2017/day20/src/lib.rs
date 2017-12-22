#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    v: Velocity,
    a: Acceleration,
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Copy, Clone)]
struct Acceleration {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn tick(&mut self) {
        self.v.apply_acceleration(&self.a);
        self.x += self.v.x;
        self.y += self.v.y;
        self.z += self.v.z;
    }

    fn distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Velocity {
    fn apply_acceleration(&mut self, a: &Acceleration) {
        self.x += a.x;
        self.y += a.y;
        self.z += a.z;
    }
}

lazy_static! {
    static ref RE: Regex = {
        let str = r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>";
        Regex::new(str).unwrap()
    };
}

pub fn process_input(input: &str) -> (usize, usize) {
    let mut points: Vec<Point> = input
        .trim()
        .lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            Point {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                z: cap[3].parse().unwrap(),
                v: Velocity {
                    x: cap[4].parse().unwrap(),
                    y: cap[5].parse().unwrap(),
                    z: cap[6].parse().unwrap(),
                },
                a: Acceleration {
                    x: cap[7].parse().unwrap(),
                    y: cap[8].parse().unwrap(),
                    z: cap[9].parse().unwrap(),
                },
            }
        })
        .collect();
    let mut points2 = points.clone();

    for _i in 0..1_000 {
        for point in &mut points {
            point.tick();
        }
    }

    let smallest = points
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.distance().cmp(&b.1.distance()))
        .unwrap();

    for _i in 0..1_000 {
        let mut map = HashMap::new();
        for point in &mut points2 {
            point.tick();
            *map.entry((point.x, point.y, point.z)).or_insert(0) += 1;
        }

        points2 = points2
            .into_iter()
            .filter(|point| map[&(point.x, point.y, point.z)] == 1)
            .collect();
    }

    println!("{}", points2.len());


    (smallest.0, points2.len())
}
