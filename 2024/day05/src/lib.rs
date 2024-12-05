use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>) {
    let (rules, updates) = input.trim().split_once("\n\n").unwrap();

    let mut rules_map: HashMap<u8, HashSet<u8>> = HashMap::new();
    rules.lines().for_each(|line| {
        let (left, right) = line.split_once('|').unwrap();
        let (left, right) = (left.parse::<u8>().unwrap(), right.parse::<u8>().unwrap());
        rules_map.entry(left).or_default().insert(right);
    });

    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules_map, updates)
}

pub fn part1((rules, updates): &(HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>)) -> u32 {
    updates
        .iter()
        .filter(|update| is_correctly_ordered(update, rules))
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

fn is_correctly_ordered(update: &[u8], rules: &HashMap<u8, HashSet<u8>>) -> bool {
    update
        .windows(2)
        .all(|pair| rules.get(&pair[0]).unwrap().contains(&pair[1]))
}

pub fn part2((rules, updates): &(HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>)) -> u32 {
    updates
        .iter()
        .filter(|update| !is_correctly_ordered(update, rules))
        .map(|update| sorted_update(update, rules))
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

fn sorted_update(update: &[u8], rules: &HashMap<u8, HashSet<u8>>) -> Vec<u8> {
    let mut update = update.to_vec();

    update.sort_by(|a, b| {
        if a == b {
            std::cmp::Ordering::Equal
        } else if rules.get(a).unwrap().contains(b) {
            std::cmp::Ordering::Less
        } else if rules.get(b).unwrap().contains(a) {
            std::cmp::Ordering::Greater
        } else {
            unreachable!("no rule for {:?} and {:?}", a, b);
        }
    });

    update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run05() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
