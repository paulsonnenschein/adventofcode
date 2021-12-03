type Parsed = String;

pub fn parse(input: &str) -> Vec<Parsed> {
    input
        .lines()
        .map(|line| line.parse::<Parsed>().unwrap())
        .collect()
}

pub fn part1(input: &[Parsed]) -> u32 {
    let columns = input.first().unwrap().chars().count();
    let mut count_ones = vec![0u32; columns];

    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                count_ones[i] += 1;
            }
        }
    }

    let mut result = 0u32;
    let half = (input.len() / 2) as u32;

    for ones in &count_ones {
        result <<= 1;
        if *ones > half {
            result |= 1;
        }
    }

    let mask = 2u32.pow(columns as u32) - 1;
    let inverted = !result & mask;
    println!("{:?} {:?} {:b}", result, inverted, mask);

    result * inverted
}

pub fn part2(input: &[Parsed]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run03sample() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(&parsed));
    }
    #[test]
    fn run03() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(&parsed));
    }
}
