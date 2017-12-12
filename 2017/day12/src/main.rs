extern crate day12;
use day12::*;

fn main() {
    let input = include_str!("../input.txt");
    let (count, group_count) = count_programs(input);

    println!("Part1 solution: {}", count);
    println!("Part2 solution: {}", group_count);
}
