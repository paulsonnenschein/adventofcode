extern crate day16;
use day16::*;

fn main() {
    let input = include_str!("../input.txt");
    let (result, result2) = process_input(input);

    println!("Part1 solution: {}", result);
    println!("Part2 solution: {}", result2);
}
