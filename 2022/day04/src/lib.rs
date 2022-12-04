use std::ops::RangeInclusive;

type Range = RangeInclusive<u32>;

pub fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut splitter = line
                .split(|c| c == '-' || c == ',')
                .map(|i| i.parse().unwrap());
            let l1 = splitter.next().unwrap();
            let l2 = splitter.next().unwrap();
            let r1 = splitter.next().unwrap();
            let r2 = splitter.next().unwrap();
            (l1..=l2, r1..=r2)
        })
        .collect()
}

pub fn part1(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|(first, second)| {
            (first.contains(second.start()) && first.contains(second.end()))
                || (second.contains(first.start()) && second.contains(first.end()))
        })
        .count()
}

pub fn part2(input: Vec<(Range, Range)>) -> usize {
    input
        .into_iter()
        .filter(|(first, second)| {
            first.contains(second.start())
                || first.contains(second.end())
                || second.contains(first.start())
                || second.contains(first.end())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run04() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
