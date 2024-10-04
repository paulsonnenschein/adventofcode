pub fn parse(input: &str) -> [u64; 9] {
    let mut result = [0; 9];

    for x in input
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
    {
        result[x] += 1;
    }

    result
}

pub fn part(mut input: [u64; 9], days: usize) -> u64 {
    for _ in 0..days {
        let zeros = input[0];
        input[0] = input[1];
        input[1] = input[2];
        input[2] = input[3];
        input[3] = input[4];
        input[4] = input[5];
        input[5] = input[6];
        input[6] = input[7] + zeros;
        input[7] = input[8];
        input[8] = zeros;
    }

    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run06() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part(parsed.clone(), 80));
        println!("{:?}", part(parsed, 256));
    }
}
