pub fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn part1(input: &[u32]) -> u32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..=max)
        .map(|i| input.iter().map(|val| val.abs_diff(i)).sum())
        .min()
        .unwrap()
}
pub fn part2(input: &[u32]) -> u32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..=max)
        .map(|i| {
            input
                .iter()
                .map(|val| triangular_numbers(val.abs_diff(i)))
                .sum()
        })
        .min()
        .unwrap()
}

fn triangular_numbers(x: u32) -> u32 {
    x * (x + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run07() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
