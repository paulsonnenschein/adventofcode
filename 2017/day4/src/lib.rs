use std::collections::HashSet;

pub fn is_row_valid(input: &str) -> bool {
    let words: Vec<_> = input.split_whitespace().collect();
    let set: HashSet<_> = input.split_whitespace().collect();
    words.len() == set.len()
}

pub fn count_valid_rows(input: &str) -> i32 {
    input.lines().map(is_row_valid).filter(|&e| e).count() as i32
}
