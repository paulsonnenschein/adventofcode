pub fn str_to_vec(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

pub fn count_steps(input: &[i32]) -> i32 {
    let mut pointer: i32 = 0;
    let mut input_cloned = Vec::from(input);

    let mut steps = 0;
    while let Some(value) = input_cloned.get_mut(pointer as usize) {
        pointer += *value;
        *value += 1;
        steps += 1;
    }

    steps
}

pub fn count_steps_part2(input: &[i32]) -> i32 {
    let mut pointer: i32 = 0;
    let mut input_cloned = Vec::from(input);

    let mut steps = 0;
    while let Some(value) = input_cloned.get_mut(pointer as usize) {
        pointer += *value;
        *value += if *value >= 3 { -1 } else { 1 };
        steps += 1;
    }

    steps
}
