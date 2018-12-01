extern crate day01;
use day01::*;

fn main() {
    let input = include_str!("../input.txt");
    let result = process_input(input);

    println!("Part1 solution: {}", result.0);
    println!("Part2 solution: {}", result.1);
}
