pub fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

pub fn part1(input: &[u32]) -> u32 {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count() as u32
}

pub fn part2(input: &[u32]) -> u32 {
    let summed: Vec<u32> = input
        .windows(3)
        .map(|window| window.iter().sum::<u32>())
        .collect();
    summed
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run01() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
