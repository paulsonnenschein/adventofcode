use std::collections::HashMap;
use std::iter::repeat;
use std::str::FromStr;

type Parsed = Line;

#[derive(Debug)]
pub struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").unwrap();
        let (x1, y1) = left.split_once(',').unwrap();
        let (x2, y2) = right.split_once(',').unwrap();

        Ok(Line {
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap(),
        })
    }
}

pub fn parse(input: &str) -> Vec<Parsed> {
    input
        .lines()
        .map(|line| line.parse::<Parsed>().unwrap())
        .collect()
}

pub fn part1(input: &[Parsed]) -> u32 {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for line in input
        .iter()
        .filter(|line| line.x1 == line.x2 || line.y1 == line.y2)
    {
        for (x, y) in get_points(line).into_iter() {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }

    map.values().filter(|&&val| val >= 2).count() as u32
}

fn get_points(line: &Parsed) -> Vec<(u32, u32)> {
    if line.x1 == line.x2 {
        repeat(line.x1)
            .zip((line.y1.min(line.y2))..=(line.y1.max(line.y2)))
            .collect()
    } else if line.y1 == line.y2 {
        ((line.x1.min(line.x2))..=(line.x1.max(line.x2)))
            .zip(repeat(line.y1))
            .collect()
    } else {
        let x_iter: Vec<_> = if line.x1 > line.x2 {
            (line.x2..=line.x1).rev().collect()
        } else {
            (line.x1..=line.x2).collect()
        };
        let y_iter: Vec<_> = if line.y1 > line.y2 {
            (line.y2..=line.y1).rev().collect()
        } else {
            (line.y1..=line.y2).collect()
        };
        x_iter.into_iter().zip(y_iter.into_iter()).collect()
    }
}

pub fn part2(input: &[Parsed]) -> u32 {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for line in input.iter() {
        for (x, y) in get_points(line).into_iter() {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }

    map.values().filter(|&&val| val >= 2).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run05sample() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
    #[test]
    fn test_get_points() {
        let line = Line {
            x1: 9,
            y1: 7,
            x2: 7,
            y2: 9,
        };
        println!("{:?}", get_points(&line));
    }

    #[test]
    fn run05() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
