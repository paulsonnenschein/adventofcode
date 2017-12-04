extern crate day4;
use day4::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input);

    println!("Part1 solution: {}", count_valid_rows(&input));
    //println!("Part2 solution: {}", calculate_part2(&input));
}
