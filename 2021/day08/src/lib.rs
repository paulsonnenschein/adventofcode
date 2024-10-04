use std::collections::HashSet;

#[derive(Debug)]
pub struct DisplayInput {
    patterns: Vec<HashSet<char>>,
}
pub fn parse(input: &str) -> Vec<DisplayInput> {
    input
        .trim()
        .lines()
        .map(|line| {
            let patterns = line
                .split(' ')
                .filter(|s| *s != "|")
                .map(|s| s.chars().collect())
                .collect();
            DisplayInput { patterns }
        })
        .collect()
}

pub fn part1(input: &[DisplayInput]) -> usize {
    input
        .iter()
        .map(|input| {
            input
                .patterns
                .iter()
                .skip(10)
                .filter(|p| matches!(p.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

pub fn part2(input: &[DisplayInput]) -> u32 {
    input.iter().map(decode).sum()
}

fn decode(input: &DisplayInput) -> u32 {
    // uniquely identified by length
    let one = input
        .patterns
        .iter()
        .find(|p| p.len() == 2)
        .unwrap()
        .clone();
    let four = input
        .patterns
        .iter()
        .find(|p| p.len() == 4)
        .unwrap()
        .clone();
    let seven = input
        .patterns
        .iter()
        .find(|p| p.len() == 3)
        .unwrap()
        .clone();
    let eight = input
        .patterns
        .iter()
        .find(|p| p.len() == 7)
        .unwrap()
        .clone();

    // these share length -> need to find differentiating properties, that dont rely on knowing wich wire maps to wich
    // len == 5: 2, 3, 5
    // len == 6: 0, 6, 9

    // 3 differentiates from 2 and 5, cause its a superset of 1
    let three = input
        .patterns
        .iter()
        .find(|p| p.len() == 5 && p.is_superset(&one))
        .unwrap()
        .clone();
    // same idea
    let nine = input
        .patterns
        .iter()
        .find(|p| p.len() == 6 && p.is_superset(&three))
        .unwrap()
        .clone();

    // 5 differentiates from 2, cause 2 isnt a subset of 9
    let five = input
        .patterns
        .iter()
        .find(|p| p.len() == 5 && !p.is_superset(&one) && p.is_subset(&nine))
        .unwrap()
        .clone();
    let two = input
        .patterns
        .iter()
        .find(|p| p.len() == 5 && !p.is_superset(&one) && p != &&five)
        .unwrap()
        .clone();

    let six = input
        .patterns
        .iter()
        .find(|p| p.len() == 6 && !p.is_superset(&one))
        .unwrap()
        .clone();
    let zero = input
        .patterns
        .iter()
        .find(|p| p.len() == 6 && p.is_superset(&one) && !p.is_superset(&three))
        .unwrap()
        .clone();

    let map = |pattern: &HashSet<char>| -> u32 {
        [
            &zero, &one, &two, &three, &four, &five, &six, &seven, &eight, &nine,
        ]
        .iter()
        .enumerate()
        .find(|p| p.1 == &pattern)
        .unwrap()
        .0 as u32
    };

    map(&input.patterns[10]) * 1000
        + map(&input.patterns[11]) * 100
        + map(&input.patterns[12]) * 10
        + map(&input.patterns[13])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run08() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
