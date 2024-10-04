pub fn parse(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn map(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',

        _ => unreachable!(),
    }
}

fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,

        _ => unreachable!(),
    }
}

fn score_line(line: &str) -> u32 {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            _ => {
                let expected = stack.pop().unwrap();
                if map(expected) != c {
                    return score(c);
                }
            }
        }
    }

    0
}

pub fn part1(input: &[&str]) -> u32 {
    input.iter().map(|line| score_line(line)).sum()
}
pub fn part2(input: &[&str]) -> u64 {
    let mut scores: Vec<_> = input
        .iter()
        .filter(|line| score_line(line) == 0)
        .map(|line| {
            let mut stack = Vec::new();

            for c in line.chars() {
                match c {
                    '(' | '{' | '[' | '<' => stack.push(c),
                    _ => {
                        stack.pop();
                    }
                }
            }

            let mut score = 0;

            while let Some(c) = stack.pop() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
            }

            score
        })
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run10() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
