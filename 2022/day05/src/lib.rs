use itertools::Itertools;

pub struct Command {
    count: usize,
    from: usize,
    to: usize,
}

pub fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let mut lines = input.trim().lines().peekable();

    // parsing stacks
    let columns = (lines.peek().unwrap().len() + 1) / 4;

    let mut stacks = vec![vec![]; columns];

    while lines.peek().map_or(false, |line| line.contains('[')) {
        let line = lines.next().unwrap();
        let crates = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphabetic());

        for (pos, crate_label) in crates {
            let column = (pos - 1) / 4;
            stacks[column].push(crate_label);
        }
    }

    stacks.iter_mut().for_each(|v| v.reverse());

    // parsing instructions
    let commands = lines
        .skip(2)
        .map(|line| {
            let (count, from, to) = line
                .split_whitespace()
                .flat_map(str::parse)
                .collect_tuple()
                .unwrap();

            Command { count, from, to }
        })
        .collect();

    (stacks, commands)
}

pub fn part1(input: &(Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut stacks = input.0.clone();
    for Command { count, from, to } in &input.1 {
        for _ in 0..*count {
            let val = stacks[*from - 1].pop().unwrap();
            stacks[*to - 1].push(val);
        }
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect()
}

pub fn part2(input: (Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut stacks = input.0;

    for Command { count, from, to } in input.1 {
        let split_idx = stacks[from - 1].len() - count;
        let mut split = stacks[from - 1].split_off(split_idx);
        stacks[to - 1].append(&mut split);
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run05() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
