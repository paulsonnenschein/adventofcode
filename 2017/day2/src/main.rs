extern crate day2;
use day2::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input);

    println!("Part1 solution: {}", calculate_part1(&input));
    //println!("Part2 solution: {}", calculate_part2(&input));s
}
