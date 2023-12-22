use grid::Grid;
use num::Num;
use std::collections::HashSet;
use std::mem::swap;
use std::ops::Rem;

pub fn parse(input: &str) -> Grid<char> {
    let content = input.trim().lines().flat_map(|line| line.chars()).collect();
    Grid::from_vec(content, input.lines().next().unwrap().len())
}

pub fn part1(input: &Grid<char>, steps: usize) -> usize {
    let (start, _) = input.indexed_iter().find(|(_, c)| c == &&'S').unwrap();

    let mut last_step = HashSet::new();
    last_step.insert(start);
    let mut next_step = HashSet::new();

    for _i in 1..=steps {
        for step in last_step.drain() {
            for next in neighbors(&step).into_iter().filter(|c| input[*c] != '#') {
                next_step.insert(next);
            }
        }

        swap(&mut last_step, &mut next_step);
    }

    last_step.len()
}

fn neighbors<T: Num + Copy>(c: &(T, T)) -> [(T, T); 4] {
    let one = T::one();
    [
        (c.0 - one, c.1),
        (c.0 + one, c.1),
        (c.0, c.1 - one),
        (c.0, c.1 + one),
    ]
}

pub fn part2(input: &Grid<char>, steps: usize) -> usize {
    let (start, _) = input.indexed_iter().find(|(_, c)| c == &&'S').unwrap();

    let start = (start.0 as isize, start.1 as isize);

    let mut last_step = HashSet::with_capacity(steps * steps * 2);
    last_step.insert(start);
    let mut next_step = HashSet::with_capacity(steps * steps * 2);

    for _i in 1..=steps {
        for step in last_step.drain() {
            for next in neighbors(&step)
                .into_iter()
                .filter(|c| get_wrapping(input, c) != '#')
            {
                next_step.insert(next);
            }
        }

        swap(&mut last_step, &mut next_step);
    }

    last_step.len()
}

fn get_wrapping(input: &Grid<char>, coord: &(isize, isize)) -> char {
    let rows = input.rows() as isize;
    let cols = input.cols() as isize;
    let coord = (
        (coord.0.rem(rows) + rows).rem(rows).unsigned_abs(),
        (coord.1.rem(cols) + cols).rem(cols).unsigned_abs(),
    );
    input[coord]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run21() {
        let input = include_str!("./input.txt");
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let parsed = parse(input);
        println!("{:?}", part1(&parsed, 6));
        println!("{:?}", part2(&parsed, 5_00 /*26501365*/));
    }
}
