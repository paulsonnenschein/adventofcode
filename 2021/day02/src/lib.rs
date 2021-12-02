use std::str::FromStr;

pub enum Action {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();

        let part1 = split.next().unwrap();
        let part2 = split.next().unwrap().parse::<u32>().unwrap();

        let result = match part1 {
            "forward" => Action::Forward(part2),
            "down" => Action::Down(part2),
            "up" => Action::Up(part2),
            _ => unreachable!(),
        };

        Ok(result)
    }
}

pub fn parse(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| line.parse::<Action>().unwrap())
        .collect()
}

pub fn part1(input: &[Action]) -> u32 {
    let mut distance = 0u32;
    let mut depth = 0u32;

    for action in input {
        match action {
            Action::Forward(i) => distance += i,
            Action::Down(i) => depth += i,
            Action::Up(i) => depth -= i,
        };
    }

    distance * depth
}

pub fn part2(input: &[Action]) -> u32 {
    let mut distance = 0u32;
    let mut depth = 0u32;
    let mut aim = 0u32;

    for action in input {
        match action {
            Action::Forward(i) => {
                distance += i;
                depth += aim * i;
            }
            Action::Down(i) => aim += i,
            Action::Up(i) => aim -= i,
        };
    }

    distance * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run02() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
