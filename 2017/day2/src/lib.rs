

pub fn calculate_row(row: &[i32]) -> i32 {
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

pub fn calculate_part1(input: &str) -> i32 {
    input.lines()
        .map(|str| {
            str.split_whitespace().filter_map(|sub| sub.parse::<i32>().ok()).collect::<Vec<_>>()
        })
        .map(|row| calculate_row(&row))
        .sum()
}
