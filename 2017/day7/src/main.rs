extern crate day7;
use day7::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input);

    let vec = input_to_vec(&input);
    let base_name = find_base_name(&vec);

    println!("Part1 solution: {}", base_name);
    calculate_part2(&vec, &base_name);
}
