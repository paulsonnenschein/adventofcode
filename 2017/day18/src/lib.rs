use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

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

pub fn process_input_part2(input: &str) -> i64 {
    let instructions: Vec<Vec<String>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|instr| instr.to_string())
                .collect()
        })
        .collect();
    let instruction_clone = instructions.clone();

    let (atob_tx, atob_rx) = mpsc::channel();
    let (btoa_tx, btoa_rx) = mpsc::channel();
    let (tomain_tx, tomain_rx) = mpsc::channel();
    let tomain_tx_clone = mpsc::Sender::clone(&tomain_tx);

    let _ = thread::spawn(move || {
        part2_processor(0, instructions, atob_tx, btoa_rx, tomain_tx);
    });
    let _ = thread::spawn(move || {
        part2_processor(1, instruction_clone, btoa_tx, atob_rx, tomain_tx_clone);
    });

    let mut is_a_waiting = false;
    let mut _a_send_count = 0;
    let mut is_b_waiting = false;
    let mut b_send_count = 0;

    let mut last_was_successful = true;

    loop {
        match tomain_rx.try_recv() {
            Ok(message) => {
                println!("mesg: {:?}", message);
                match message {
                    (0, is_waiting, send_count) => {
                        is_a_waiting = is_waiting;
                        _a_send_count = send_count;
                    }
                    (1, is_waiting, send_count) => {
                        is_b_waiting = is_waiting;
                        b_send_count = send_count;
                    }
                    _ => unreachable!(),
                };
                last_was_successful = true;
            }
            Err(err) => {
                println!("err: {:?}", err);
                if err == mpsc::TryRecvError::Disconnected {
                    break; // both threads are done
                }
                if !last_was_successful && is_a_waiting && is_b_waiting {
                    break; // both threads are waiting
                }
                thread::sleep(Duration::from_millis(100));
                last_was_successful = false;
            }
        }
    }

    b_send_count
}

fn part2_processor(
    name: i64,
    instructions: Vec<Vec<String>>,
    sender: mpsc::Sender<i64>,
    receiver: mpsc::Receiver<i64>,
    tomain_sender: mpsc::Sender<(i64, bool, i64)>,
) {
    let mut registers: HashMap<char, i64> = HashMap::new();
    registers.insert('p', name);
    let mut instruction_pointer: i64 = 0;

    let mut part2 = 0;

    while instruction_pointer >= 0 && instruction_pointer < instructions.len() as i64 {
        let instruction = &instructions[instruction_pointer as usize];

        match instruction[0].as_ref() {
            "snd" => {
                let val = get_value(&instruction[1], &registers);
                let _ = sender.send(val);
                part2 += 1;
                let _ = tomain_sender.send((name, false, part2));
            }
            "set" => {
                let new_val = get_value(&instruction[2], &registers);
                let val = registers.entry(chr(&instruction[1])).or_insert(0);
                *val = new_val;
            }
            "add" => {
                let new_val = get_value(&instruction[2], &registers);
                let val = registers.entry(chr(&instruction[1])).or_insert(0);
                *val += new_val;
            }
            "mul" => {
                let new_val = get_value(&instruction[2], &registers);
                let val = registers.entry(chr(&instruction[1])).or_insert(0);
                *val *= new_val;
            }
            "mod" => {
                let new_val = get_value(&instruction[2], &registers);
                let val = registers.entry(chr(&instruction[1])).or_insert(0);
                *val %= new_val;
            }
            "rcv" => {
                let val = registers.entry(chr(&instruction[1])).or_insert(0);
                let _ = tomain_sender.send((name, true, part2));
                *val = receiver.recv().unwrap();
                let _ = tomain_sender.send((name, false, part2));
            }
            "jgz" => {
                if get_value(&instruction[1], &registers) > 0 {
                    instruction_pointer += get_value(&instruction[2], &registers) - 1;
                };
            }
            _ => unreachable!(),
        };
        instruction_pointer += 1;
    }
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
