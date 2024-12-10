use std::ops::Range;

pub fn parse(input: &str) -> Vec<u32> {
    let mut id = 0;
    let mut fill_next = true;
    input
        .trim()
        .chars()
        .flat_map(|c| {
            let len = c.to_digit(10).unwrap();

            fill_next = !fill_next;
            if fill_next {
                std::iter::repeat(u32::MAX)
            } else {
                id += 1;
                std::iter::repeat(id - 1)
            }
            .take(len as usize)
        })
        .collect()
}

pub fn part1(input: &[u32]) -> u64 {
    let mut input = input.to_vec();
    let mut left = 0;
    let mut right = input.len() - 1;

    while left < right {
        while input[left] != u32::MAX && left < right {
            left += 1;
        }
        while input[right] == u32::MAX && left < right {
            right -= 1;
        }
        input.swap(left, right);
    }

    input
        .iter()
        .enumerate()
        .take_while(|(_pos, val)| **val != u32::MAX)
        .map(|(pos, val)| pos as u64 * *val as u64)
        .sum()
}

pub fn part2(input: &[u32]) -> u64 {
    let mut input = input.to_vec();

    let mut right = input.len() - 1;

    while right > 0 {
        while input[right] == u32::MAX && right > 0 {
            right -= 1;
        }
        let block_end = right;
        while right > 0 && input[right] == input[block_end] {
            right -= 1;
        }

        let block_range = right + 1..=block_end;
        let block_size = block_end - right;

        //dbg!(&input[right + 1..=block_end], find_empty_range(&input[..=right], block_size));

        if let Some(range) = find_empty_range(&input[..=right], block_size) {
            let val = input[block_end];
            input[range].fill(val);
            input[block_range].fill(u32::MAX);
        }

        //dbg!(&input[right + 1..=block_end]);
    }

    //dbg!(&input);

    input
        .iter()
        .enumerate()
        .filter(|(_pos, val)| **val != u32::MAX)
        .map(|(pos, val)| pos as u64 * *val as u64)
        .sum()
}

fn find_empty_range(input: &[u32], min_size: usize) -> Option<Range<usize>> {
    let mut start = 0;

    while start < input.len() {
        if input[start] == u32::MAX {
            let mut end = start + 1;
            while end < input.len() && input[end] == u32::MAX {
                end += 1;
            }
            if end - start >= min_size {
                return Some(start..(start + min_size));
            }
            start = end;
        } else {
            start += 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run09() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
