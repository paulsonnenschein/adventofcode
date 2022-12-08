use crate::Direction::{Down, Left, Right, Up};
use grid::Grid;
use std::collections::HashSet;

pub fn parse(input: &str) -> Grid<i8> {
    let mut cols = 0;
    let all_trees = input
        .trim()
        .lines()
        .inspect(|line| cols = line.len())
        .flat_map(str::chars)
        .filter_map(|c| c.to_digit(10).map(|num| num as i8))
        .collect::<Vec<_>>();

    Grid::from_vec(all_trees, cols)
}

pub fn part1and2(input: &Grid<i8>) -> (usize, usize) {
    let mut visible_trees = HashSet::<Position>::new();

    let (rows, cols) = input.size();

    // horizontal
    for row in 0..rows {
        let mut min_height = -1_i8;
        for (col, &height) in input.iter_row(row).enumerate() {
            if height > min_height {
                min_height = height;
                visible_trees.insert(Position(row, col));
            }
        }
        min_height = -1;
        for (col, &height) in input.iter_row(row).enumerate().rev() {
            if height > min_height {
                min_height = height;
                visible_trees.insert(Position(row, col));
            }
        }
    }
    // vertical
    for col in 0..cols {
        let mut min_height = -1_i8;
        for (row, &height) in input.iter_col(col).enumerate() {
            if height > min_height {
                min_height = height;
                visible_trees.insert(Position(row, col));
            }
        }
        min_height = -1;
        for (row, &height) in input.iter_col(col).enumerate().rev() {
            if height > min_height {
                min_height = height;
                visible_trees.insert(Position(row, col));
            }
        }
    }

    let part1 = visible_trees.len();
    let part2 = visible_trees
        .into_iter()
        .filter(|pos| !pos.is_edge((rows, cols)))
        .map(|pos| calc_scenic_score(input, pos))
        .max()
        .unwrap();

    (part1, part2)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position(usize, usize);

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Position {
    fn adjust(&self, direction: Direction, bounds: (usize, usize)) -> Option<Position> {
        match direction {
            Up => Some(Position(self.0.checked_sub(1)?, self.1)),
            Right => {
                if self.1 + 1 >= bounds.1 {
                    None
                } else {
                    Some(Position(self.0, self.1 + 1))
                }
            }
            Down => {
                if self.0 + 1 >= bounds.0 {
                    None
                } else {
                    Some(Position(self.0 + 1, self.1))
                }
            }
            Left => Some(Position(self.0, self.1.checked_sub(1)?)),
        }
    }

    fn is_edge(&self, bounds: (usize, usize)) -> bool {
        self.0 == 0 || self.1 == 0 || self.0 + 1 == bounds.0 || self.1 + 1 == bounds.1
    }
}

fn calc_scenic_score(grid: &Grid<i8>, position: Position) -> usize {
    calc_score_direction(grid, position, Up)
        * calc_score_direction(grid, position, Right)
        * calc_score_direction(grid, position, Down)
        * calc_score_direction(grid, position, Left)
}

fn calc_score_direction(grid: &Grid<i8>, mut position: Position, delta: Direction) -> usize {
    let mut score = 0;
    let main_height = grid[position.0][position.1];

    while let Some(new_position) = position.adjust(delta, grid.size()) {
        position = new_position;
        let height = grid[position.0][position.1];

        score += 1;
        if height >= main_height {
            break;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run08() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1and2(&parsed));
    }
}
