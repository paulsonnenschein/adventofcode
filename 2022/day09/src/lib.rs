use crate::Direction::{Down, Left, Right, Up};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "R" => Ok(Right),
            "D" => Ok(Down),
            "L" => Ok(Left),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
pub struct Instruction(Direction, i32);

#[derive(Clone)]
struct State {
    head_pos: (i32, i32), // (row, col)
    tail_pos: (i32, i32),
    tail_visited: HashSet<(i32, i32)>,
}

impl State {
    fn new() -> State {
        State {
            head_pos: (0, 0),
            tail_pos: (0, 0),
            tail_visited: HashSet::from([(0, 0)]),
        }
    }

    fn move_head(&mut self, dir: Direction) {
        match dir {
            Up => self.update_head((self.head_pos.0 - 1, self.head_pos.1)),
            Right => self.update_head((self.head_pos.0, self.head_pos.1 + 1)),
            Down => self.update_head((self.head_pos.0 + 1, self.head_pos.1)),
            Left => self.update_head((self.head_pos.0, self.head_pos.1 - 1)),
        }
    }

    fn update_head(&mut self, new_head: (i32, i32)) {
        self.head_pos = new_head;
        if !is_touching(self.head_pos, self.tail_pos) {
            let mut new_tail = self.tail_pos;

            new_tail.0 += (new_head.0 - new_tail.0).signum();
            new_tail.1 += (new_head.1 - new_tail.1).signum();

            self.tail_pos = new_tail;
            self.tail_visited.insert(new_tail);
        }
    }

    #[allow(dead_code)]
    fn print_last_trace(&self) {
        let set = &self.tail_visited;
        let min_row = set.iter().map(|e| e.0).min().unwrap_or(0) - 1;
        let max_row = set.iter().map(|e| e.0).max().unwrap_or(0) + 1;
        let min_col = set.iter().map(|e| e.1).min().unwrap_or(0) - 1;
        let max_col = set.iter().map(|e| e.1).max().unwrap_or(0) + 1;

        for row in min_row..=max_row {
            print!("{: >3}  ", row);
            for col in min_col..=max_col {
                let c = if set.contains(&(row, col)) { '#' } else { '.' };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn is_touching(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    let distance = (head_pos.0 - tail_pos.0).abs() + (head_pos.1 - tail_pos.1).abs();
    if head_pos.0 == tail_pos.0 || head_pos.1 == tail_pos.1 {
        distance <= 1
    } else {
        distance <= 2
    }
}

struct StateChain(Vec<State>);

impl StateChain {
    fn new(size: usize) -> StateChain {
        StateChain(vec![State::new(); size])
    }

    fn move_head(&mut self, dir: Direction) {
        let chain = &mut self.0;

        chain[0].move_head(dir);
        let mut last_tail = chain[0].tail_pos;

        for el in chain.iter_mut().skip(1) {
            el.update_head(last_tail);
            last_tail = el.tail_pos;
        }
    }

    fn last_pos_count(&self) -> usize {
        self.0.last().unwrap().tail_visited.len()
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (dir, distance) = line.split_whitespace().collect_tuple().unwrap();

            Instruction(dir.parse().unwrap(), distance.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> usize {
    let mut state = State::new();

    for Instruction(dir, dist) in input {
        for _ in 0..*dist {
            state.move_head(*dir);
        }
    }
    state.tail_visited.len()
}

pub fn part2(input: &[Instruction]) -> usize {
    let mut state = StateChain::new(10 - 1); // 9 pairs -> 10 elements

    for Instruction(dir, dist) in input {
        for _ in 0..*dist {
            state.move_head(*dir);
        }
    }

    state.last_pos_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run09() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
