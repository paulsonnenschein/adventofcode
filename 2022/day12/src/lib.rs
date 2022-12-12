use crate::Field::{End, Normal, Start};
use pathfinding::prelude::astar;
use pathfinding::prelude::Matrix;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Field {
    Start,
    End,
    Normal(u8),
}

impl Field {
    fn value(self) -> i8 {
        match self {
            Start => 0,
            End => 25,
            Normal(v) => v as i8,
        }
    }
    fn can_access(self, other: Self) -> bool {
        (other.value() - self.value()) <= 1
    }
}

pub fn parse(input: &str) -> Matrix<Field> {
    let cols = input.lines().next().unwrap().len();
    let rows = input.trim().lines().count();
    let all_fields: Vec<Field> = input
        .lines()
        .flat_map(str::chars)
        .map(|c| match c {
            'S' => Start,
            'E' => End,
            'a'..='z' => Normal((c as u8) - b'a'),
            _ => unreachable!("invalid char {}", c),
        })
        .collect();
    Matrix::from_vec(rows, cols, all_fields).unwrap()
}

fn shortest_path(
    start_coord: (usize, usize),
    end_coord: (usize, usize),
    grid: &Matrix<Field>,
) -> Option<usize> {
    let result = astar(
        &start_coord,
        |coord| {
            grid.neighbours(coord, false)
                .filter(|c| grid[coord].can_access(grid[c]))
                .map(|c| (c, 1))
                .collect::<Vec<_>>()
        },
        |(row, col)| row.abs_diff(end_coord.0) + col.abs_diff(end_coord.1),
        |&coord| coord == end_coord,
    );

    result.map(|(_path, length)| length)
}

pub fn part1(grid: &Matrix<Field>) -> usize {
    let start_coord = grid.indices().find(|idx| grid[idx] == Start).unwrap();
    let end_coord = grid.indices().find(|idx| grid[idx] == End).unwrap();

    shortest_path(start_coord, end_coord, grid).unwrap()
}

pub fn part2(grid: &Matrix<Field>) -> usize {
    let end_coord = grid.indices().find(|idx| grid[idx] == End).unwrap();

    grid.indices()
        .filter(|idx| grid[idx].value() == 0)
        .filter_map(|idx| shortest_path(idx, end_coord, grid))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run12() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
