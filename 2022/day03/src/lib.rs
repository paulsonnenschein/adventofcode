use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left_set = left.chars().collect::<HashSet<char>>();
            let right_set = right.chars().collect::<HashSet<char>>();
            let mut intersection = left_set.intersection(&right_set);
            let duplicate = intersection.next().unwrap();
            priority(*duplicate)
        })
        .sum()
}

fn priority(duplicate: char) -> u32 {
    if duplicate >= 'a' {
        (duplicate as u32) - ('a' as u32) + 1
    } else {
        (duplicate as u32) - ('A' as u32) + 27
    }
}

pub fn part2(input: &[&str]) -> u32 {
    input
        .chunks(3)
        .map(|chunk| {
            let mut sets = chunk
                .iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>();
            let first = sets.pop().unwrap();

            let duplicate = first
                .iter()
                .find(move |c| sets.iter().all(|s| s.contains(c)))
                .unwrap();
            priority(*duplicate)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run03() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
