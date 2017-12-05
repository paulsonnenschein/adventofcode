extern crate day5;
use day5::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input);

    let instructions = str_to_vec(&input);

    println!("Part1 solution: {}", count_steps(&instructions));
    //println!("Part2 solution: {}", 5);
}
