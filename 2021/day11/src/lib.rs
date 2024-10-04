use pathfinding::prelude::Matrix;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Matrix<u32> {
    Matrix::from_iter(
        input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
    )
}

pub fn part1(input: &Matrix<u32>) -> u32 {
    let mut input = input.clone();
    (0..100).map(|_| step(&mut input)).sum()
}

pub fn part2(input: &Matrix<u32>) -> u32 {
    let mut input = input.clone();
    (1..).find(|_| step(&mut input) == 100).unwrap()
}

fn step(matrix: &mut Matrix<u32>) -> u32 {
    // increment all by one
    matrix.iter_mut().for_each(|v| *v += 1);

    // flash all the 9
    let mut flash_queue = matrix
        .keys()
        .filter(|key| matrix[*key] == 10)
        .collect::<VecDeque<_>>();

    while let Some(flash) = flash_queue.pop_front() {
        for n in matrix.neighbours(flash, true) {
            matrix[n] += 1;
            if matrix[n] == 10 {
                flash_queue.push_back(n);
            }
        }
    }

    // reset and count flashes
    let mut num_flashes = 0;
    for flashed in matrix.iter_mut().filter(|v| **v >= 10) {
        *flashed = 0;
        num_flashes += 1;
    }

    num_flashes
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
