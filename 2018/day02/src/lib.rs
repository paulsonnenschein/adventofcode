use std::collections::HashMap;

pub fn process_input(input: &str) -> (i32, String) {
    let vec: (i32, i32) = input
        .lines()
        .map(|line| {
            let mut chars = HashMap::new();

            for ch in line.chars() {
                *chars.entry(ch).or_insert(0) += 1;
            }

            let values: Vec<_> = chars.values().collect();

            (values.contains(&&2), values.contains(&&3))
        }).fold(
            (0, 0),
            |(acc_twos, acc_threes), (conatins_two, conatins_three)| {
                let twos = if conatins_two { acc_twos + 1 } else { acc_twos };
                let threes = if conatins_three {
                    acc_threes + 1
                } else {
                    acc_threes
                };

                (twos, threes)
            },
        );

    let result1 = vec.0 * vec.1;

    let mut result2 = String::new();
    'outer: for line1 in input.lines() {
        'inner: for line2 in input.lines() {
            let mut char_pos = None;
            for (i, (left, right)) in line1.chars().zip(line2.chars()).enumerate() {
                if left != right {
                    if char_pos.is_none() {
                        char_pos = Some(i);
                    } else {
                        // invalid
                        continue 'inner;
                    }
                }
            }
            if let Some(pos) = char_pos {
                result2 = format!("{}{}", &line1[..pos], &line1[(pos + 1)..]).to_string();
                break 'outer;
            }
        }
    }

    (result1, result2)
}
