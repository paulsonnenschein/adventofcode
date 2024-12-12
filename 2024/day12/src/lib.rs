use pathfinding::matrix::Matrix;
use pathfinding::prelude::bfs_reach;
use std::collections::HashSet;

const EMPTY: u8 = u8::MAX;

pub fn parse(input: &str) -> Matrix<u8> {
    let mat = Matrix::from_rows(
        input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - b'A')),
    )
    .unwrap();

    Matrix::from_fn(mat.rows + 2, mat.columns + 2, |pos| match pos {
        (0, _) | (_, 0) => EMPTY,
        (row, col) => *mat.get((row - 1, col - 1)).unwrap_or(&EMPTY),
    })
}

pub fn solve(input: &Matrix<u8>) -> (usize, usize) {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut total_value_p1 = 0;
    let mut total_value_p2 = 0;

    for (pos, val) in input.items() {
        if *val == EMPTY || seen.contains(&pos) {
            continue;
        }

        let region_fields = bfs_reach(pos, |p| {
            input
                .neighbours(*p, false)
                .filter(|neighbor| input[*neighbor] == *val)
        })
        .collect::<Vec<_>>();

        let perimeter: usize = region_fields
            .iter()
            .map(|field| {
                input
                    .neighbours(*field, false)
                    .filter(|neighbor| input[*neighbor] != *val)
                    .count()
            })
            .sum();

        total_value_p1 += perimeter * region_fields.len();

        let corners: usize = region_fields
            .iter()
            .map(|field| count_corners(*field, *val, input))
            .sum();
        total_value_p2 += region_fields.len() * corners;
        //dbg!(val, perimeter, &region_fields);
        seen.extend(region_fields);
    }
    (total_value_p1, total_value_p2)
}

fn count_corners((row, col): (usize, usize), val: u8, input: &Matrix<u8>) -> usize {
    let up = input[(row - 1, col)];
    let down = input[(row + 1, col)];
    let left = input[(row, col - 1)];
    let right = input[(row, col + 1)];

    let up_right = input[(row - 1, col + 1)];
    let down_right = input[(row + 1, col + 1)];
    let down_left = input[(row + 1, col - 1)];
    let up_left = input[(row - 1, col - 1)];

    let mut corners = 0;

    if (up == val && right == val && up_right != val) || (up != val && right != val) {
        corners += 1;
    }

    if (down == val && right == val && down_right != val) || (down != val && right != val) {
        corners += 1;
    }

    if (down == val && left == val && down_left != val) || (down != val && left != val) {
        corners += 1;
    }

    if (up == val && left == val && up_left != val) || (up != val && left != val) {
        corners += 1;
    }

    corners
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn run12() {
        let input = include_str!("./input.txt");
        let s = Instant::now();
        let parsed = parse(input);
        let (p1, p2) = solve(&parsed);
        let duration = s.elapsed();
        println!("part1: {:?}", p1);
        println!("part2: {:?}", p2);
        println!("duration: {:?}", duration);
    }
}
