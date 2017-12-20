extern crate day18;
use day18::*;

fn main() {
    let input = include_str!("../input.txt");
    //let input = include_str!("../test.txt");
    let result = process_input(input);

    println!("Part1 solution: {}", result);
    //println!("Part2 solution: {}", garbage_counter);
}
