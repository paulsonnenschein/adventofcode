extern crate day05;
use day05::*;

fn main() {
    let input = include_str!("../input.txt");
    //let input = include_str!("../sample.txt");
    let result = process_input(input);

    println!("Part1 solution: {}", result.0);
    println!("Part2 solution: {}", result.1);
}
