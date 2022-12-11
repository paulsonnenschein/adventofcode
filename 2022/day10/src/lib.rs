use crate::Instruction::{AddX, Noop};
use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut whitespace = s.split_whitespace();
        match (whitespace.next(), whitespace.next()) {
            (Some("noop"), None) => Ok(Noop),
            (Some("addx"), Some(number)) => Ok(AddX(number.parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

pub fn parse(input: &str) -> Vec<i32> {
    let mut acc = 1;
    input
        .trim()
        .lines()
        .flat_map(FromStr::from_str)
        .flat_map(|instr| match instr {
            Noop => vec![0],
            AddX(amount) => vec![0, amount],
        })
        .map(|delta| {
            let curr = acc;
            acc += delta;
            curr
        })
        .collect()
}

pub fn part1(positions: &[i32]) -> i32 {
    vec![20_usize, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|i| positions[i - 1] * i as i32)
        .sum()
}

pub fn part2(positions: &[i32]) {
    for row in 0..6_usize {
        for col in 0..40 {
            let pos_idx = row * 40 + col;
            let acceptable_range = ((col as i32) - 1)..=((col as i32) + 1);
            if acceptable_range.contains(&positions[pos_idx]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run10() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
