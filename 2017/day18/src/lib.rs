use std::collections::HashMap;

pub fn process_input(input: &str) -> i64 {
    let instructions: Vec<Vec<&str>> = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut instruction_pointer: i64 = 0;

    let mut last_played = 0;

    let mut part1 = None;

    while instruction_pointer >= 0 && instruction_pointer < instructions.len() as i64 {
        let instruction = &instructions[instruction_pointer as usize];

        match instruction[0] {
            "snd" => last_played = get_value(instruction[1], &registers),
            "set" => {
                let new_val = get_value(instruction[2], &registers);
                let val = registers.entry(chr(instruction[1])).or_insert(0);
                *val = new_val;
            }
            "add" => {
                let new_val = get_value(instruction[2], &registers);
                let val = registers.entry(chr(instruction[1])).or_insert(0);
                *val += new_val;
            }
            "mul" => {
                let new_val = get_value(instruction[2], &registers);
                let val = registers.entry(chr(instruction[1])).or_insert(0);
                *val *= new_val;
            }
            "mod" => {
                let new_val = get_value(instruction[2], &registers);
                let val = registers.entry(chr(instruction[1])).or_insert(0);
                *val %= new_val;
            }
            "rcv" => {
                let val = registers.entry(chr(instruction[1])).or_insert(0);
                if last_played != 0 && part1 == None {
                    println!("{}", last_played);
                    part1 = Some(last_played)
                };
                *val = last_played;
            }
            "jgz" => {
                if get_value(instruction[1], &registers) > 0 {
                    instruction_pointer += instruction[2].parse::<i64>().unwrap() - 1;
                };
            }
            _ => unreachable!(),
        };
        instruction_pointer += 1;
    }


    part1.unwrap()
}

fn get_value(input: &str, registers: &HashMap<char, i64>) -> i64 {
    if chr(input).is_alphabetic() {
        registers[&chr(input)]
    } else {
        input.parse().unwrap()
    }
}

fn chr(input: &str) -> char {
    input.chars().next().unwrap()
}
