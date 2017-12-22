extern crate day20;
use day20::*;

fn main() {
    let input = include_str!("../input.txt");
    let (part1, part2) = process_input(input);

    println!("Part1 solution: {}", part1);
    println!("Part2 solution: {}", part2);
}
