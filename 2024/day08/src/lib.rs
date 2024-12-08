use itertools::Itertools;
use pathfinding::matrix::Matrix;
use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.trim().lines().map(|line| line.chars())).unwrap()
}

pub fn part1(input: &Matrix<char>) -> usize {
    let mut freq_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    for (pos, c) in input.items() {
        if *c != '.' {
            freq_map.entry(*c).or_default().insert(pos);
        }
    }

    let mut antinodes = HashSet::new();

    for (_freq, locations) in freq_map.into_iter() {
        for (p1, p2) in locations.iter().tuple_combinations() {
            let delta = (p2.0 as isize - p1.0 as isize, p2.1 as isize - p1.1 as isize);
            let first = (p1.0 as isize - delta.0, p1.1 as isize - delta.1);
            if let (Ok(row), Ok(col)) = (first.0.try_into(), first.1.try_into()) {
                if input.get((row, col)).is_some() {
                    antinodes.insert((row, col));
                }
            }
            let second = (p2.0 as isize + delta.0, p2.1 as isize + delta.1);
            if let (Ok(row), Ok(col)) = (second.0.try_into(), second.1.try_into()) {
                if input.get((row, col)).is_some() {
                    antinodes.insert((row, col));
                }
            }
        }
    }

    antinodes.len()
}

pub fn part2(input: &Matrix<char>) -> usize {
    let mut freq_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    for (pos, c) in input.items() {
        if *c != '.' {
            freq_map.entry(*c).or_default().insert(pos);
        }
    }

    let mut antinodes = HashSet::new();

    for (_freq, locations) in freq_map.into_iter() {
        for (p1, p2) in locations.iter().tuple_combinations() {
            antinodes.insert(*p1);
            antinodes.insert(*p2);

            let delta = (p2.0 as isize - p1.0 as isize, p2.1 as isize - p1.1 as isize);

            antinodes.extend(input.in_direction(*p1, delta));
            antinodes.extend(input.in_direction(*p1, (-delta.0, -delta.1)));
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run08() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
