use nalgebra::matrix;
use regex::Regex;

type Point = (f64, f64);

#[derive(Debug)]
pub struct Item {
    button_a: Point,
    button_b: Point,
    target: Point,
}
pub fn parse(input: &str) -> Vec<Item> {
    let pattern = Regex::new(
        r"^Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)$",
    )
    .unwrap();
    input
        .trim()
        .split("\n\n")
        .map(|line| {
            let (_, [a_x, a_y, b_x, b_y, target_x, target_y]) =
                pattern.captures(line).unwrap().extract();
            Item {
                button_a: (a_x.parse().unwrap(), a_y.parse().unwrap()),
                button_b: (b_x.parse().unwrap(), b_y.parse().unwrap()),
                target: (target_x.parse().unwrap(), target_y.parse().unwrap()),
            }
        })
        .collect()
}

pub fn part1(input: &[Item]) -> usize {
    input.iter().flat_map(optimize).sum()
}

fn optimize(item: &Item) -> Option<usize> {
    // a * a_x + b * b_x = target_x
    // a * a_y + b * b_y = target_y
    // minimize(a * 3 + b)
    // ->
    // |a_x b_x |   |a|   | target_x |
    // |a_y b_y | * |b| = | target_y |
    // ->
    // A * x = b
    // ->
    // x = inv(A) * b

    let a = matrix![
        item.button_a.0, item.button_b.0;
        item.button_a.1, item.button_b.1;
    ];

    let a_inv = a.try_inverse().unwrap();

    let b = matrix![
        item.target.0;
        item.target.1;
    ];

    let x = a_inv * b;

    fn is_almost_int(x: f64) -> bool {
        (x.fract().round() - x.fract()).abs() < 0.001
    }

    if is_almost_int(x.x) && is_almost_int(x.y) {
        let a_pushes = x.x.round() as usize;
        let b_pushes = x.y.round() as usize;
        Some(a_pushes * 3 + b_pushes)
    } else {
        None
    }
}

pub fn part2(input: Vec<Item>) -> usize {
    input
        .into_iter()
        .map(|mut item| {
            item.target.0 += 10000000000000.0;
            item.target.1 += 10000000000000.0;
            item
        })
        .flat_map(|item: Item| optimize(&item))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn run13() {
        let input = include_str!("./input.txt");
        let s = Instant::now();
        let parsed = parse(input);
        let p1 = part1(&parsed);
        let p2 = part2(parsed);
        let duration = s.elapsed();
        println!("part1: {:?}", p1);
        println!("part2: {:?}", p2);
        println!("duration: {:?}", duration);
    }
}
