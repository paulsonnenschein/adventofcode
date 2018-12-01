use std::collections::HashSet;

pub fn process_input(input: &str) -> (i32, i32) {
    let change_list = input
        .trim()
        .split_whitespace()
        .map(|change| change.parse::<i32>().unwrap());

    let result1 = change_list.clone().sum();

    let mut seen_frequencies = HashSet::new();
    let mut frequency = 0;
    seen_frequencies.insert(frequency);

    for change in change_list.cycle() {
        frequency += change;
        if seen_frequencies.contains(&frequency) {
            break;
        }
        seen_frequencies.insert(frequency);
    }

    (result1, frequency)
}
