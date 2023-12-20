use pathfinding::prelude::astar;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|p| p.parse().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &[(u32, u32, u32)]) -> usize {
    let set: HashSet<(u32, u32, u32)> = input.iter().copied().collect();

    input
        .iter()
        .map(|i| {
            possible_neighbors(i)
                .into_iter()
                .filter(|side| !set.contains(side))
                .count()
        })
        .sum()
}

fn possible_neighbors(pos: &(u32, u32, u32)) -> [(u32, u32, u32); 6] {
    [
        (pos.0.wrapping_sub(1), pos.1, pos.2),
        (pos.0.wrapping_add(1), pos.1, pos.2),
        (pos.0, pos.1.wrapping_sub(1), pos.2),
        (pos.0, pos.1.wrapping_add(1), pos.2),
        (pos.0, pos.1, pos.2.wrapping_sub(1)),
        (pos.0, pos.1, pos.2.wrapping_add(1)),
    ]
}

pub fn part2(input: &[(u32, u32, u32)]) -> usize {
    let set: HashSet<(u32, u32, u32)> = input.iter().copied().collect();

    input
        .iter()
        .map(|i| {
            possible_neighbors(i)
                .into_iter()
                .filter(|side| !set.contains(side) && has_path_to_origin(side, &set))
                .count()
        })
        .sum()
}

fn has_path_to_origin(pos: &(u32, u32, u32), occupied_fields: &HashSet<(u32, u32, u32)>) -> bool {
    astar(
        pos,
        |p| {
            possible_neighbors(p)
                .into_iter()
                .filter(|side| !occupied_fields.contains(side))
                .map(|p| (p, 1))
        },
        |p| (p.0 as u64 + p.1 as u64 + p.2 as u64) / 2,
        |p| *p == (0, 0, 0),
    )
    .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run18() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
