extern crate day13;
use day13::*;

fn main() {
    let input = include_str!("../input.txt");
    let (score, delay) = process_input(input);

    println!("Part1 solution: {}", score);
    println!("Part2 solution: {}", delay);
}
