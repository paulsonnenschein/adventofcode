extern crate day6;
use day6::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input);

    let banks = str_to_vec(&input);
    let (iterations, loop_length) = count_iterations(&banks);

    println!("Part1 solution: {}", iterations);
    println!("Part2 solution: {}", loop_length);
}
