use crate::Op::{Plus, Times};
use crate::Operand::{Const, Old};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u64 as parse_u64},
    combinator::{all_consuming, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Plus,
    Times,
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Old,
    Const(u64),
}

#[derive(Debug, Clone)]
pub struct MonkeySpec {
    _id: usize,
    current_items: VecDeque<u64>,
    operation: (Op, Operand),
    test: u64,
    target_true: usize,
    target_false: usize,
    inspected_items: usize,
}

impl MonkeySpec {
    fn process_item(&mut self, divisor: Option<u64>) -> Option<(u64, usize)> {
        let item = self.current_items.pop_front()?;

        self.inspected_items += 1;

        let new_val = match self.operation {
            (Plus, Old) => item + item,
            (Plus, Const(val)) => item + val,
            (Times, Old) => item * item,
            (Times, Const(val)) => item * val,
        };
        let relieved_val = if let Some(divisor) = divisor {
            new_val % divisor // I had to look this up :(
        } else {
            new_val / 3
        };

        if relieved_val % self.test == 0 {
            Some((relieved_val, self.target_true))
        } else {
            Some((relieved_val, self.target_false))
        }
    }
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    let parse_plus = map(tag("+"), |_| Plus);
    let parse_times = map(tag("*"), |_| Times);

    alt((parse_plus, parse_times))(i)
}

fn parse_operand(i: &str) -> IResult<&str, Operand> {
    let parse_old = map(tag("old"), |_| Old);
    let parse_const = map(parse_u64, Const);

    alt((parse_old, parse_const))(i)
}

fn parse_monkey(i: &str) -> IResult<&str, MonkeySpec> {
    let (i, monkey_id) = delimited(tag("Monkey "), parse_u64, tag(":\n"))(i)?;
    let (i, current_items) = delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), parse_u64),
        tag("\n"),
    )(i)?;
    let (i, operation) = delimited(
        tag("  Operation: new = old "),
        separated_pair(parse_op, tag(" "), parse_operand),
        tag("\n"),
    )(i)?;
    let (i, test) = delimited(tag("  Test: divisible by "), parse_u64, tag("\n"))(i)?;
    let (i, target_true) =
        delimited(tag("    If true: throw to monkey "), parse_u64, tag("\n"))(i)?;
    let (i, target_false) =
        delimited(tag("    If false: throw to monkey "), parse_u64, tag("\n"))(i)?;

    Ok((
        i,
        MonkeySpec {
            _id: monkey_id as usize,
            current_items: VecDeque::from(current_items),
            operation,
            test,
            target_true: target_true as usize,
            target_false: target_false as usize,
            inspected_items: 0,
        },
    ))
}

pub fn parse(input: &str) -> Vec<MonkeySpec> {
    all_consuming(separated_list0(line_ending, parse_monkey))(input)
        .unwrap()
        .1
}

pub fn part1(input: &[MonkeySpec]) -> usize {
    let mut monkeys = input.to_vec();

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((relieved_item, next_target)) = monkeys[i].process_item(None) {
                monkeys[next_target].current_items.push_back(relieved_item);
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspected_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn part2(input: &[MonkeySpec]) -> usize {
    let mut monkeys = input.to_vec();

    let common_divisor: u64 = monkeys.iter().map(|m| m.test).product();

    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some((relieved_item, next_target)) =
                monkeys[i].process_item(Some(common_divisor))
            {
                monkeys[next_target].current_items.push_back(relieved_item);
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspected_items)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run11() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
