use std::collections::HashSet;

pub struct Input {
    dots: Vec<(u32, u32)>,
    folds: Vec<(bool, u32)>,
}
pub fn parse(input: &str) -> Input {
    let (dots, folds) = input.trim().split_once("\n\n").unwrap();

    let dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        })
        .collect();

    let folds = folds
        .lines()
        .map(|line| {
            let (direction, idx) = line.split_once('=').unwrap();
            (direction == "fold along x", idx.parse::<u32>().unwrap())
        })
        .collect();

    Input { dots, folds }
}

pub fn part1(input: &Input) -> usize {
    let mut grid = input.dots.iter().copied().collect::<HashSet<_>>();

    do_fold(&mut grid, &input.folds[0]);

    grid.len()
}

fn do_fold(grid: &mut HashSet<(u32, u32)>, (is_x, idx): &(bool, u32)) {
    if *is_x {
        let (mut stays, moves): (HashSet<(u32, u32)>, _) = grid.iter().partition(|&(x, _)| x < idx);
        stays.extend(moves.into_iter().map(|(x, y)| (idx - (x - idx), y)));

        std::mem::swap(grid, &mut stays);
    } else {
        let (mut stays, moves): (HashSet<(u32, u32)>, _) = grid.iter().partition(|&(_, y)| y < idx);
        stays.extend(moves.into_iter().map(|(x, y)| (x, idx - (y - idx))));

        std::mem::swap(grid, &mut stays);
    }
}

pub fn part2(input: &Input) {
    let mut grid = input.dots.iter().copied().collect::<HashSet<_>>();

    for fold in &input.folds {
        do_fold(&mut grid, fold);
    }

    let max_x = grid.iter().map(|(x, _)| x).max().unwrap();
    let max_y = grid.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run13() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
