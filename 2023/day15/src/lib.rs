use std::ops::Rem;

pub fn parse(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input.iter().map(|part| hash(part)).sum()
}

fn hash(val: &str) -> u32 {
    let mut acc = 0;
    for x in val.chars() {
        acc += x as u32;
        acc *= 17;
        acc = acc.rem(256);
    }

    acc
}

pub fn part2(input: &[&str]) -> usize {
    let mut map = vec![Vec::<(&str, u32)>::new(); 256];

    for op in input {
        let (key, val) = op.split_once(|c| c == '-' || c == '=').unwrap();
        let idx = hash(key) as usize;
        let previous_idx = map[idx].iter().position(|v| v.0 == key);

        if val.is_empty() {
            // remove element
            if let Some(previous_idx) = previous_idx {
                map[idx].remove(previous_idx);
            }
        } else {
            let val = val.parse().unwrap();
            // add / mutate element
            if let Some(previous_idx) = previous_idx {
                map[idx][previous_idx] = (key, val);
            } else {
                map[idx].push((key, val));
            }
        }
    }

    map.iter()
        .enumerate()
        .flat_map(|(i, bucket)| {
            bucket
                .iter()
                .enumerate()
                .map(move |(b_i, val)| (i + 1) * (b_i + 1) * val.1 as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run15() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("cm"), 0);
        assert_eq!(hash("qp"), 1);
        assert_eq!(hash("cm-"), 253);
    }
}
