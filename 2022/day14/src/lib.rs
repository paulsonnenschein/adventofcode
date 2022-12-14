use crate::Field::{Sand, Wall};
use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::tag,
    character::complete::{i32 as parse_i32, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

type Point = (i32, i32);
type Path = Vec<Point>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Field {
    Sand,
    Wall,
}

const SOURCE: Point = (0, 500);

fn parse_point(i: &str) -> IResult<&str, Point> {
    map(
        separated_pair(parse_i32, tag(","), parse_i32),
        |(col, row)| (row, col),
    )(i)
}

fn parse_path(i: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), parse_point)(i)
}

pub fn parse(input: &str) -> HashMap<Point, Field> {
    let paths = all_consuming(separated_list1(line_ending, parse_path))(input.trim())
        .unwrap()
        .1;

    paths
        .iter()
        .flat_map(|path| {
            path.iter()
                .tuple_windows()
                .flat_map(|(start, end)| fill_path(*start, *end))
        })
        .map(|point| (point, Wall))
        .collect()
}

fn fill_path(start: Point, end: Point) -> Vec<Point> {
    let (d_x, d_y) = ((end.0 - start.0).signum(), (end.1 - start.1).signum());

    let mut path = vec![start];
    let mut current = (start.0 + d_x, start.1 + d_y);

    while current != end {
        path.push(current);
        current = (current.0 + d_x, current.1 + d_y);
    }

    path.push(end);

    path
}

fn draw(input: &HashMap<Point, Field>) {
    let (min_row, max_row) = match input.keys().map(|p| p.0).minmax() {
        MinMaxResult::OneElement(_) | MinMaxResult::NoElements => unreachable!(),
        MinMaxResult::MinMax(min, max) => (min.min(SOURCE.0), max.max(SOURCE.0)),
    };
    let (min_col, max_col) = match input.keys().map(|p| p.1).minmax() {
        MinMaxResult::OneElement(_) | MinMaxResult::NoElements => unreachable!(),
        MinMaxResult::MinMax(min, max) => (min.min(SOURCE.1), max.max(SOURCE.1)),
    };

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if row == SOURCE.0 && col == SOURCE.1 {
                // source
                print!("+");
            } else if row == 172 && col == 492 {
                // 173, 492
                print!("X");
            } else {
                match input.get(&(row, col)) {
                    None => print!(" "),
                    Some(Sand) => print!("o"),
                    Some(Wall) => print!("#"),
                }
            }
        }
        println!();
    }
}

fn simulate(input: &mut HashMap<Point, Field>, void_border: i32) -> bool {
    let mut current = SOURCE;
    loop {
        if current.0 + 1 >= void_border {
            return false; // when the send is in the void, we are done with the simulation
        }

        let down = (current.0 + 1, current.1);
        let down_left = (current.0 + 1, current.1 - 1);
        let down_right = (current.0 + 1, current.1 + 1);

        match (
            input.get(&down),
            input.get(&down_left),
            input.get(&down_right),
        ) {
            (None, _, _) => current = down,
            (_, None, _) => current = down_left,
            (_, _, None) => current = down_right,
            _ => break,
        }
    }

    if current == SOURCE {
        false
    } else {
        input.insert(current, Sand);
        true
    }
}

fn simulate_part2(input: &mut HashMap<Point, Field>, floor: i32) -> bool {
    let mut current = SOURCE;
    loop {
        if current.0 + 1 >= floor {
            break;
        }

        let down = (current.0 + 1, current.1);
        let down_left = (current.0 + 1, current.1 - 1);
        let down_right = (current.0 + 1, current.1 + 1);

        match (
            input.get(&down),
            input.get(&down_left),
            input.get(&down_right),
        ) {
            (None, _, _) => current = down,
            (_, None, _) => current = down_left,
            (_, _, None) => current = down_right,
            _ => break,
        }
    }

    input.insert(current, Sand);

    current != SOURCE
}

pub fn part1(input: &HashMap<Point, Field>) -> usize {
    let void_border = input.keys().map(|(row, _col)| row).max().unwrap() + 1;

    let mut map = input.clone();

    while simulate(&mut map, void_border) {}

    draw(&map);

    map.values().filter(|&&field| field == Sand).count()
}

pub fn part2(mut map: HashMap<Point, Field>) -> usize {
    let floor_row = map.keys().map(|(row, _col)| row).max().unwrap() + 2;

    while simulate_part2(&mut map, floor_row) {}

    draw(&map);

    map.values().filter(|&&field| field == Sand).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run14() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
