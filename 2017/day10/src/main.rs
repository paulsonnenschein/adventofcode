extern crate day10;
use day10::*;

fn main() {
    let input = include_str!("../input.txt");

    println!("Part1 solution: {}", hash(input, 256));
    //println!("Part2 solution: {}", garbage_counter);
}
