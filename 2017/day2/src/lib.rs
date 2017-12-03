

fn str_to_numvec(input: &str) -> Vec<i32> {
    input.split_whitespace().filter_map(|sub| sub.parse().ok()).collect()
}

pub fn calculate_row_minmax(row: &[i32]) -> i32 {
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

pub fn calculate_row_divisible(row: &[i32]) -> i32 {
    for item in row {
        for item2 in row {
            if item != item2 && item % item2 == 0 {
                return item / item2;
            }
        }
    }
    panic!("Invalid Input")
}

pub fn calculate_part1(input: &str) -> i32 {
    input.lines()
        .map(str_to_numvec)
        .map(|row| calculate_row_minmax(&row))
        .sum()
}

pub fn calculate_part2(input: &str) -> i32 {
    input.lines()
        .map(str_to_numvec)
        .map(|row| calculate_row_divisible(&row))
        .sum()
}
