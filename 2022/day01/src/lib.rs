pub fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect()
}

pub fn part1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

pub fn part2(mut input: Vec<u32>) -> u32 {
    input.sort_by(|a, b| b.cmp(a));
    input.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run01() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
