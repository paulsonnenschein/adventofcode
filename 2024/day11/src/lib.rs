use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part1(input: &[u64]) -> usize {
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();

    input
        .iter()
        .map(|num| how_many_stones(*num, 25, &mut cache))
        .sum()
}

pub fn part2(input: &[u64]) -> usize {
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();

    input
        .iter()
        .map(|num| how_many_stones(*num, 75, &mut cache))
        .sum()
}

fn step(stone: u64) -> impl Iterator<Item = u64> {
    match stone {
        0 => vec![1],
        num => {
            let num_str = num.to_string();
            if num_str.len() % 2 == 0 {
                vec![
                    num_str[0..(num_str.len() / 2)].parse::<u64>().unwrap(),
                    num_str[(num_str.len() / 2)..].parse::<u64>().unwrap(),
                ]
            } else {
                vec![num * 2024]
            }
        }
    }
    .into_iter()
}

fn how_many_stones(
    stone: u64,
    steps_left: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if let Some(x) = cache.get(&(stone, steps_left)) {
        return *x;
    }
    let count = if steps_left == 1 {
        step(stone).count()
    } else {
        step(stone)
            .map(|x| how_many_stones(x, steps_left - 1, cache))
            .sum()
    };

    cache.insert((stone, steps_left), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
