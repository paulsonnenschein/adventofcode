use std::collections::HashSet;

pub fn process_input(input: &str) -> (usize, usize) {
    let chars: Vec<_> = input.trim().chars().collect();

    let part1 = clean_size(&chars);

    let char_types: HashSet<_> = input.trim().to_ascii_lowercase().chars().collect();

    let part2 = char_types.iter().fold(std::usize::MAX, |max, &ch| {
        let input: Vec<char> = chars
            .iter()
            .filter(|&filter_ch| filter_ch.to_ascii_lowercase() != ch)
            .cloned()
            .collect();
        let curr_size = clean_size(&input);
        if curr_size < max {
            curr_size
        } else {
            max
        }
    });

    (part1, part2)
}

fn clean_size(input: &[char]) -> usize {
    let chars = input.to_vec();
    let mut current_size = chars.len();
    let mut current_iter = chars.into_iter().peekable();
    let mut next = vec![];

    loop {
        if let Some(ch) = current_iter.next() {
            if let Some(next_ch) = current_iter.peek().cloned() {
                if (ch.is_ascii_lowercase() && ch.to_ascii_uppercase() == next_ch)
                    || (ch.is_ascii_uppercase() && ch.to_ascii_lowercase() == next_ch)
                {
                    // remove these chars
                    let _ = current_iter.next().unwrap();
                    continue;
                }
            }
            next.push(ch);
        } else if next.len() >= current_size {
            break;
        } else {
            current_size = next.len();
            current_iter = next.into_iter().peekable();
            next = vec![];
        }
    }

    next.len()
}
