use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &[Vec<i32>]) -> usize {
    input.iter().filter(|row| check_part1(row)).count()
}

fn check_part1(row: &[i32]) -> bool {
    row.iter()
        .tuple_windows()
        .all(|(a, b)| a < b && (1..=3).contains(&a.abs_diff(*b)))
        || row
            .iter()
            .tuple_windows()
            .all(|(a, b)| a > b && (1..=3).contains(&a.abs_diff(*b)))
}

pub fn part2(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|row| check_part1(row) || check_part2(row))
        .count()
}

fn check_part2(row: &[i32]) -> bool {
    for skip in 0..row.len() {
        let mut vec = row.to_vec();
        vec.remove(skip);
        if check_part1(&vec) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run02() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
