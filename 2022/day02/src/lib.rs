use crate::Outcome::{Draw, Loss, Win};
use crate::Play::{Paper, Rock, Scissors};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn match_outcome(&self, other: Play) -> Outcome {
        match (self, other) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Loss,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
        }
    }

    fn play_needed_for_outcome(&self, desired_outcome: Outcome) -> Play {
        match (self, desired_outcome) {
            (Rock, Draw) | (Paper, Loss) | (Scissors, Win) => Rock,
            (Paper, Draw) | (Scissors, Loss) | (Rock, Win) => Paper,
            (Scissors, Draw) | (Rock, Loss) | (Paper, Win) => Scissors,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "X" => Ok(Rock),
            "B" => Ok(Paper),
            "Y" => Ok(Paper),
            "C" => Ok(Scissors),
            "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

pub fn parse(input: &str) -> Vec<(Play, Play)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(1);
            (left.parse().unwrap(), right.trim().parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &[(Play, Play)]) -> u32 {
    input
        .iter()
        .map(|(opponent_play, my_play)| {
            let outcome = my_play.match_outcome(*opponent_play);
            my_play.score() + outcome.score()
        })
        .sum()
}

pub fn part2(input: Vec<(Play, Play)>) -> u32 {
    input
        .into_iter()
        // translate old assumption of X=Rock,... to X=Loss,...
        .map(|(left, right)| match right {
            Rock => (left, Loss),
            Paper => (left, Draw),
            Scissors => (left, Win),
        })
        .map(|(opponent_play, desired_outcome)| {
            let my_play = opponent_play.play_needed_for_outcome(desired_outcome);
            my_play.score() + desired_outcome.score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run02() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
