use pathfinding::matrix::directions::{DIRECTIONS_8, NE, NW, SE, SW};
use pathfinding::matrix::Matrix;
use std::collections::HashSet;

pub fn parse(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.trim().lines().map(|line| line.chars())).unwrap()
}

pub fn part1(input: &Matrix<char>) -> usize {
    input
        .items()
        .filter(|(_, c)| **c == 'X')
        .map(|(coord, _)| {
            DIRECTIONS_8
                .iter()
                .filter(|dir| {
                    let mut iter = input.in_direction(coord, **dir).map(|pos| input[pos]);
                    iter.next() == Some('M') && iter.next() == Some('A') && iter.next() == Some('S')
                })
                .count()
        })
        .sum()
}

pub fn part2(input: &Matrix<char>) -> usize {
    let target = HashSet::from([Some('M'), Some('S')]);

    let get_for_directions =
        |coord: (usize, usize), dirs: &[(isize, isize)]| -> HashSet<Option<char>> {
            dirs.iter()
                .map(|dir| input.in_direction(coord, *dir).next().map(|pos| input[pos]))
                .collect()
        };

    input
        .items()
        .filter(|(_, c)| **c == 'A')
        .map(|(coord, _)| {
            // check diag
            if get_for_directions(coord, &[NE, SW]) == target
                && get_for_directions(coord, &[NW, SE]) == target
            {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run04() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
