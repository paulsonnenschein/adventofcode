use crate::Item::{List, Number};
use nom::sequence::{delimited, pair, separated_pair};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u32 as parse_u32},
    combinator::{all_consuming, map},
    multi::separated_list0,
    IResult,
};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl Eq for Item {}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(v) => Debug::fmt(v, f),
            List(l) => l.fmt(f),
        }
    }
}

pub struct Pair {
    left: Item,
    right: Item,
}

impl Debug for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Pair {\nleft: ")?;
        self.left.fmt(f)?;
        f.write_str("\n")?;
        f.write_str("right: ")?;
        self.right.fmt(f)?;
        f.write_str("\n}\n")
    }
}

fn parse_number(i: &str) -> IResult<&str, u32> {
    parse_u32(i)
}

fn parse_list(i: &str) -> IResult<&str, Vec<Item>> {
    delimited(tag("["), separated_list0(tag(","), parse_item), tag("]"))(i)
}

fn parse_item(i: &str) -> IResult<&str, Item> {
    alt((map(parse_number, Number), map(parse_list, List)))(i)
}

fn parse_pair(i: &str) -> IResult<&str, Pair> {
    map(
        separated_pair(parse_list, line_ending, parse_list),
        |(l, r)| Pair {
            left: List(l),
            right: List(r),
        },
    )(i)
}

pub fn parse(input: &str) -> Vec<Pair> {
    all_consuming(separated_list0(pair(line_ending, line_ending), parse_pair))(input.trim())
        .unwrap()
        .1
}

fn compare(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (Number(l), Number(r)) => l.cmp(r),
        (List(_), Number(r)) => left.cmp(&List(vec![Number(*r)])),
        (Number(l), List(_)) => List(vec![Number(*l)]).cmp(right),
        (List(l), List(r)) => {
            let mut idx = 0;

            loop {
                if l.len() == r.len() && l.len() == idx {
                    return Equal; // both lists are done
                } else if l.len() <= idx {
                    return Less; // only left is done
                } else if r.len() <= idx {
                    return Greater; // only right is done
                }

                let comparison = compare(&l[idx], &r[idx]);
                if comparison != Equal {
                    return comparison;
                }

                idx += 1;
            }
        }
    }
}

pub fn part1(input: &[Pair]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(idx, p)| match compare(&p.left, &p.right) {
            Less => Some(idx + 1),
            _ => None,
        })
        .sum()
}

pub fn part2(input: Vec<Pair>) -> usize {
    let mut input = input
        .into_iter()
        .flat_map(|pair| vec![pair.left, pair.right])
        .collect::<Vec<_>>();

    let divider_2 = List(vec![List(vec![Number(2)])]);
    let divider_6 = List(vec![List(vec![Number(6)])]);

    input.push(divider_2.clone());
    input.push(divider_6.clone());

    input.sort();

    let idx_divider_2 = input.iter().position(|item| item == &divider_2).unwrap();
    let idx_divider_6 = input.iter().position(|item| item == &divider_6).unwrap();

    (idx_divider_2 + 1) * (idx_divider_6 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run13() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }

    #[test]
    fn test_01() {
        /*
        [[1], 2]
        [1]
        */
        assert_eq!(
            Greater,
            compare(
                &List(vec![List(vec![Number(1)]), Number(2)]),
                &List(vec![Number(1)]),
            )
        )
    }
}
