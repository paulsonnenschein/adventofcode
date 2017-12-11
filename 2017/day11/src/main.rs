extern crate day11;
use day11::*;

fn main() {
    let input = include_str!("../input.txt");
    let (distance, max_distance) = process_input(input);

    println!("Part1 solution: {}", distance);
    println!("Part2 solution: {}", max_distance);
}
