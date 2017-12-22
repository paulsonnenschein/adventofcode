extern crate day19;
use day19::*;

fn main() {
    let input = include_str!("../input.txt");
    let (output, steps) = process_input(input);

    println!("Part1 solution: {}", output);
    println!("Part2 solution: {}", steps);
}
