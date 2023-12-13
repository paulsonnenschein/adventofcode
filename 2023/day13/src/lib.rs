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
    input.iter().map(|g| get_mirror_val(g, 0)).sum()
}

pub fn part2(input: Vec<Grid<char>>) -> usize {
    input.into_iter().map(|g| get_mirror_val(&g, 1)).sum()
}

fn get_mirror_val(grid: &Grid<char>, allowed_differences: usize) -> usize {
    // check rows
    for ((c_idx, current), (n_idx, next)) in grid.iter_rows().enumerate().tuple_windows() {
        let mut differences = current.zip(next).filter(|(c, n)| c != n).count();
        if differences <= allowed_differences {
            let back_iter = (0..c_idx).rev();
            let forward_iter = (n_idx + 1)..grid.rows();
            differences += back_iter
                .zip(forward_iter)
                .map(|(back_idx, forward_idx)| {
                    let back = grid.iter_rows().nth(back_idx).unwrap();
                    let forward = grid.iter_rows().nth(forward_idx).unwrap();

                    back.zip(forward).filter(|(c, n)| c != n).count()
                })
                .sum::<usize>();

            if differences == allowed_differences {
                return (c_idx + 1) * 100;
            }
        }
    }

    // check cols
    for ((c_idx, current), (n_idx, next)) in grid.iter_cols().enumerate().tuple_windows() {
        let mut differences = current.zip(next).filter(|(c, n)| c != n).count();
        if differences <= allowed_differences {
            let back_iter = (0..c_idx).rev();
            let forward_iter = (n_idx + 1)..grid.cols();
            differences += back_iter
                .zip(forward_iter)
                .map(|(back_idx, forward_idx)| {
                    let back = grid.iter_cols().nth(back_idx).unwrap();
                    let forward = grid.iter_cols().nth(forward_idx).unwrap();

                    back.zip(forward).filter(|(c, n)| c != n).count()
                })
                .sum::<usize>();

            if differences == allowed_differences {
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
        println!("{:?}", part2(parsed));
    }
}
