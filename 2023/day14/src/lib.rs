use grid::Grid;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Rem;

pub fn parse(input: &str) -> Grid<char> {
    let content = input.trim().lines().flat_map(|line| line.chars()).collect();
    Grid::from_vec(content, input.lines().next().unwrap().len())
}

pub fn part1(input: &Grid<char>) -> usize {
    let mut grid = input.clone();

    shift_north(&mut grid);

    score(&grid)
}

fn shift_north(grid: &mut Grid<char>) {
    for col in 0..grid.cols() {
        let mut num_round_rocks = 0;
        for row in (0..grid.rows()).rev() {
            let el = grid[(row, col)];
            match el {
                '.' => {}
                'O' => {
                    num_round_rocks += 1;
                    grid[(row, col)] = '.';
                }
                '#' => {
                    for row_ in (row + 1)..(row + num_round_rocks + 1) {
                        grid[(row_, col)] = 'O';
                    }
                    num_round_rocks = 0;
                }
                _ => unreachable!(),
            }
        }
        for row_ in 0..num_round_rocks {
            grid[(row_, col)] = 'O';
        }
    }
}

fn shift_west(grid: &mut Grid<char>) {
    for row in 0..grid.rows() {
        let mut num_round_rocks = 0;
        for col in (0..grid.cols()).rev() {
            let el = grid[(row, col)];
            match el {
                '.' => {}
                'O' => {
                    num_round_rocks += 1;
                    grid[(row, col)] = '.';
                }
                '#' => {
                    for col_ in (col + 1)..(col + num_round_rocks + 1) {
                        grid[(row, col_)] = 'O';
                    }
                    num_round_rocks = 0;
                }
                _ => unreachable!(),
            }
        }
        for col_ in 0..num_round_rocks {
            grid[(row, col_)] = 'O';
        }
    }
}

fn shift_south(grid: &mut Grid<char>) {
    for col in 0..grid.cols() {
        let mut num_round_rocks = 0;
        for row in 0..grid.rows() {
            let el = grid[(row, col)];
            match el {
                '.' => {}
                'O' => {
                    num_round_rocks += 1;
                    grid[(row, col)] = '.';
                }
                '#' => {
                    for row_ in (row - num_round_rocks)..row {
                        grid[(row_, col)] = 'O';
                    }
                    num_round_rocks = 0;
                }
                _ => unreachable!(),
            }
        }
        let rows = grid.rows();
        for row_ in (rows - num_round_rocks)..rows {
            grid[(row_, col)] = 'O';
        }
    }
}

fn shift_east(grid: &mut Grid<char>) {
    for row in 0..grid.rows() {
        let mut num_round_rocks = 0;
        for col in 0..grid.cols() {
            let el = grid[(row, col)];
            match el {
                '.' => {}
                'O' => {
                    num_round_rocks += 1;
                    grid[(row, col)] = '.';
                }
                '#' => {
                    for col_ in (col - num_round_rocks)..col {
                        grid[(row, col_)] = 'O';
                    }
                    num_round_rocks = 0;
                }
                _ => unreachable!(),
            }
        }
        let cols = grid.cols();
        for col_ in (cols - num_round_rocks)..cols {
            grid[(row, col_)] = 'O';
        }
    }
}

pub fn part2(mut grid: Grid<char>, target: i32) -> usize {
    let mut seen_hashes = Vec::new();
    for _ in 0..target {
        shift_north(&mut grid);
        shift_west(&mut grid);
        shift_south(&mut grid);
        shift_east(&mut grid);

        let hash = hash(&grid);

        if let Some(idx) = seen_hashes.iter().rposition(|(h, _)| h == &hash) {
            // calculate at wich point in the found loop position=target would be
            let loop_len = seen_hashes.len() - idx;
            let prefix = idx - 1;
            let calc_idx = prefix + ((target as usize - (prefix + 1)).rem(loop_len));

            return seen_hashes[calc_idx].1;
        }

        let score = score(&grid);
        seen_hashes.push((hash, score))
    }

    unreachable!()
}

fn hash(grid: &Grid<char>) -> u64 {
    let mut hasher = DefaultHasher::new();
    let data = grid.clone().into_vec();
    data.hash(&mut hasher);
    hasher.finish()
}

fn score(grid: &Grid<char>) -> usize {
    let rows = grid.rows();

    grid.iter_rows()
        .enumerate()
        .map(|(row_idx, row)| row.filter(|c| c == &&'O').count() * (rows - row_idx))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run14() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed, 1000000000));
    }
}
