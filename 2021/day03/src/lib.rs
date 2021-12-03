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
    //println!("{:?} {:?} {:b}", result, inverted, mask);

    result * inverted
}

pub fn part2(input: &[Parsed]) -> u32 {
    let columns = input.first().unwrap().chars().count();
    let mut oxygen_candidates = Vec::from(input);

    for col in 0..columns {
        let mut ones_count = 0usize;
        for candidate in &oxygen_candidates {
            if candidate.chars().nth(col).unwrap() == '1' {
                ones_count += 1;
            }
        }
        let zeros_count = oxygen_candidates.len() - ones_count;
        if ones_count >= zeros_count {
            oxygen_candidates.retain(|el| el.chars().nth(col).unwrap() == '1');
        } else {
            oxygen_candidates.retain(|el| el.chars().nth(col).unwrap() == '0');
        }
        if oxygen_candidates.len() == 1 {
            break;
        }
    }

    let mut co2_candidates = Vec::from(input);

    for col in 0..columns {
        let mut ones_count = 0usize;
        for candidate in &co2_candidates {
            if candidate.chars().nth(col).unwrap() == '1' {
                ones_count += 1;
            }
        }
        let zeros_count = co2_candidates.len() - ones_count;
        if ones_count >= zeros_count {
            co2_candidates.retain(|el| el.chars().nth(col).unwrap() == '0');
        } else {
            co2_candidates.retain(|el| el.chars().nth(col).unwrap() == '1');
        }
        if co2_candidates.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&oxygen_candidates[0], 2).unwrap()
        * u32::from_str_radix(&co2_candidates[0], 2).unwrap()
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
        println!("{:?}", part2(&parsed));
    }
    #[test]
    fn run03() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
