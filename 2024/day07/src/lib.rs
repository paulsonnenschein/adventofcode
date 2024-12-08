use std::collections::HashSet;

pub struct Case {
    test: u64,
    values: Vec<u64>,
}

pub fn parse(input: &str) -> Vec<Case> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (test_str, rest) = line.split_once(": ").unwrap();
            let test = test_str.parse().unwrap();
            let values = rest
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();

            Case { test, values }
        })
        .collect()
}

pub fn part1(input: &[Case]) -> u64 {
    input
        .iter()
        .filter(|case| calc_possible_results_p1(&case.values).contains(&case.test))
        .map(|case| case.test)
        .sum()
}

pub fn part2(input: &[Case]) -> u64 {
    input
        .iter()
        .filter(|case| calc_possible_results_p2(&case.values).contains(&case.test))
        .map(|case| case.test)
        .sum()
}

fn calc_possible_results_p1(input: &[u64]) -> HashSet<u64> {
    let mut solutions = HashSet::from([input[0]]);

    for right in &input[1..] {
        solutions = solutions
            .into_iter()
            .flat_map(|left| [left * right, left + right])
            .collect();
    }

    solutions
}

fn calc_possible_results_p2(input: &[u64]) -> HashSet<u64> {
    let mut solutions = HashSet::from([input[0]]);

    for right in &input[1..] {
        solutions = solutions
            .into_iter()
            .flat_map(|left| {
                let mut st = left.to_string();
                st.push_str(&right.to_string());
                [left * right, left + right, st.parse().unwrap()]
            })
            .collect();
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run07() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
