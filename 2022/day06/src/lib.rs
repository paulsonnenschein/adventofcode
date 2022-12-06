use itertools::Itertools;

pub fn parse(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

pub fn part1(input: &[char]) -> usize {
    find_unique_window(input, 4)
}

pub fn part2(input: &[char]) -> usize {
    find_unique_window(input, 14)
}

fn find_unique_window(input: &[char], window_size: usize) -> usize {
    input
        .windows(window_size)
        .find_position(|window| window.iter().unique().count() == window_size)
        .map(|(pos, _)| pos + window_size)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run06() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
