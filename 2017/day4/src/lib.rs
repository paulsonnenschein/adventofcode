use std::collections::HashSet;
use std::iter::FromIterator;

pub fn is_row_valid(input: &str) -> bool {
    let words: Vec<_> = input.split_whitespace().collect();
    let set: HashSet<_> = words.iter().collect();
    words.len() == set.len()
}
pub fn is_row_valid_part2(input: &str) -> bool {
    let words: Vec<_> = input
        .split_whitespace()
        .map(|word| {
            let mut vec = word.chars().collect::<Vec<_>>();
            vec.sort();
            String::from_iter(vec)
        })
        .collect();
    let set: HashSet<_> = words.iter().collect();
    words.len() == set.len()
}

pub fn count_valid_rows<T>(input: &str, mapper: T) -> i32
where
    T: Fn(&str) -> bool,
{
    input.lines().map(mapper).filter(|&e| e).count() as i32
}
