extern crate day14;
use day14::*;

fn main() {
    let input = "wenycdww";
    let (count, groups) = count(input);

    println!("Part1 solution: {}", count);
    println!("Part2 solution: {}", groups);
}
