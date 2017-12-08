use std::collections::HashMap;


#[derive(Debug)]
pub struct Instruction {
    pub register: String,
    pub action: Action,
    pub condition: Condition,
}

#[derive(Debug)]
pub struct Condition {
    pub register: String,
    pub op: String,
    pub value: i32,
}

#[derive(Debug)]
pub enum Action {
    Inc(i32),
    Dec(i32),
}

pub fn input_to_instruction_list(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let elements: Vec<_> = line.split_whitespace().collect();
            let register = elements[0].to_string();
            let action_str = elements[1].to_string();
            let amount = elements[2].parse::<i32>().unwrap();

            let action = if action_str == "inc" {
                Action::Inc(amount)
            } else {
                Action::Dec(amount)
            };

            let condition_register = elements[4].to_string();
            let condition_op = elements[5].to_string();
            let condition_value = elements[6].parse::<i32>().unwrap();

            Instruction {
                register,
                action,
                condition: Condition {
                    register: condition_register,
                    op: condition_op,
                    value: condition_value,
                },
            }
        })
        .collect()
}

pub fn execute_instructions(instructions: &[Instruction]) {
    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut max_value = std::i32::MIN;

    for instruction in instructions {
        if evaluate_condition(&instruction.condition, &registers) {
            registers.entry(instruction.register.clone()).or_insert(0);

            let value = match instruction.action {
                Action::Inc(value) => value,
                Action::Dec(value) => -value,
            };

            let register_value = registers.get_mut(&instruction.register).unwrap();

            *register_value += value;

            if *register_value > max_value {
                max_value = *register_value;
            }
        }
    }

    let max_register = registers.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();


    println!("{:?}", max_register);
    println!("{:?}", max_value);
}

fn evaluate_condition(condition: &Condition, registers: &HashMap<String, i32>) -> bool {
    let register_value = if registers.contains_key(&condition.register) {
        registers[&condition.register]
    } else {
        0
    };

    match condition.op.as_ref() {
        ">" => register_value > condition.value,
        ">=" => register_value >= condition.value,
        "<" => register_value < condition.value,
        "<=" => register_value <= condition.value,
        "==" => register_value == condition.value,
        "!=" => register_value != condition.value,
        _ => panic!("Invalid pattern!"),
    }
}
