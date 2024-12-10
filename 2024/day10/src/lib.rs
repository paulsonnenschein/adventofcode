use pathfinding::matrix::Matrix;
use pathfinding::prelude::{bfs_reach, count_paths};

pub fn parse(input: &str) -> Matrix<u8> {
    Matrix::from_rows(
        input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8)),
    )
    .unwrap()
}

pub fn part1(input: &Matrix<u8>) -> usize {
    input
        .items()
        .filter(|(_pos, val)| **val == 0)
        .map(|(pos, _val)| {
            bfs_reach(pos, |p| {
                let curr = input[*p];
                input
                    .neighbours(*p, false)
                    .filter(move |neighbor| input[*neighbor] == curr + 1)
            })
            .filter(|pos| input[*pos] == 9)
            .count()
        })
        .sum()
}

pub fn part2(input: &Matrix<u8>) -> usize {
    let successors = |p: &(usize, usize)| {
        let curr = input[*p];
        input
            .neighbours(*p, false)
            .filter(move |neighbor| input[*neighbor] == curr + 1)
    };
    input
        .items()
        .filter(|(_pos, val)| **val == 0)
        .map(|(pos, _val)| {
            bfs_reach(pos, successors)
                .filter(|pos| input[*pos] == 9)
                .map(|target| count_paths(pos, successors, |p| *p == target))
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run10() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
