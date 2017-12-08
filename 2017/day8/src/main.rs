extern crate day8;
use day8::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input);

    let instructions = input_to_instruction_list(&input);


    execute_instructions(&instructions);
}
