use grid::Grid;
use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Grid<char>> {
    input
        .trim()
        .split("\n\n")
        .map(|part| {
            let content = part.lines().flat_map(|line| line.chars()).collect();
            Grid::from_vec(content, part.lines().next().unwrap().len())
        })
        .collect()
}

pub fn part1(input: &[Grid<char>]) -> usize {
    input.iter().map(get_mirror_val).sum()
}

fn get_mirror_val(grid: &Grid<char>) -> usize {
    // check rows
    for ((c_idx, current), (n_idx, next)) in grid.iter_rows().enumerate().tuple_windows() {
        if current.eq(next) {
            let back_iter = (0..c_idx).rev();
            let forward_iter = (n_idx + 1)..grid.rows();
            let all_eq = back_iter.zip(forward_iter).all(|(back_idx, forward_idx)| {
                grid.iter_rows()
                    .nth(back_idx)
                    .unwrap()
                    .eq(grid.iter_rows().nth(forward_idx).unwrap()) // now this is inefficient
            });

            if all_eq {
                return (c_idx + 1) * 100;
            }
        }
    }

    // check cols
    for ((c_idx, current), (n_idx, next)) in grid.iter_cols().enumerate().tuple_windows() {
        if current.eq(next) {
            let back_iter = (0..c_idx).rev();
            let forward_iter = (n_idx + 1)..grid.cols();
            let all_eq = back_iter.zip(forward_iter).all(|(back_idx, forward_idx)| {
                grid.iter_cols()
                    .nth(back_idx)
                    .unwrap()
                    .eq(grid.iter_cols().nth(forward_idx).unwrap()) // now this is inefficient
            });

            if all_eq {
                return c_idx + 1;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run13() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(parsed));
    }
}
