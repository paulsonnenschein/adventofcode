use itertools::Itertools;

pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .trim()
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .unzip()
}

pub fn part1(left: &[u32], right: &[u32]) -> u32 {
    let mut left = left.to_vec();
    let mut right = right.to_vec();
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub fn part2(left: &[u32], right: &[u32]) -> u32 {
    let left_counts = left.iter().counts();
    let right_counts = right.iter().counts();

    left_counts
        .iter()
        .map(|(k, v)| *k * (*v as u32) * *right_counts.get(k).unwrap_or(&0) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run01() {
        let input = include_str!("./input.txt");
        let (l, r) = parse(input);
        println!("part1: {:?}", part1(&l, &r));
        println!("part2: {:?}", part2(&l, &r));
    }
}
