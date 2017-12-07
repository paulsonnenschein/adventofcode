use std::collections::HashMap;

pub fn str_to_vec(str: &str) -> Vec<i32> {
    str.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

pub fn count_iterations(input: &[i32]) -> (i32, i32) {
    let mut banks = Vec::from(input);
    let mut map = HashMap::new();
    let mut counter = 1;
    map.insert(banks.clone(), counter);

    balance_banks(&mut banks);
    while !map.contains_key(&banks) {
        counter += 1;
        map.insert(banks.clone(), counter);
        balance_banks(&mut banks);
    }
    let original_index = map[&banks];

    (counter, (counter - original_index) + 1)
}

pub fn balance_banks(input: &mut [i32]) {
    let max_count = *input.iter().max().unwrap();
    let max_position = input.iter().position(|&el| el == max_count).unwrap();
    let input_length = input.len();

    input[max_position] = 0;

    for i in max_position..(max_position + max_count as usize) {
        input[(i + 1) % input_length] += 1;
    }
}
