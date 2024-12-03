use regex::Regex;

#[derive(Debug)]
pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .map(|capture| {
            let all = &capture[0];
            if all.starts_with("don") {
                Instruction::Dont
            } else if all.starts_with("do") {
                Instruction::Do
            } else {
                Instruction::Mul(capture[1].parse().unwrap(), capture[2].parse().unwrap())
            }
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> u32 {
    input
        .iter()
        .map(|instr| match instr {
            Instruction::Mul(l, r) => l * r,
            Instruction::Do => 0,
            Instruction::Dont => 0,
        })
        .sum()
}

pub fn part2(input: &[Instruction]) -> u32 {
    let mut enabled = true;
    let mut sum = 0;
    for instr in input {
        match instr {
            Instruction::Mul(l, r) if enabled => sum += l * r,
            Instruction::Mul(_, _) => {}
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run03() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("part1: {:?}", part1(&parsed));
        println!("part2: {:?}", part2(&parsed));
    }
}
