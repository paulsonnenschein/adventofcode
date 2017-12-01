extern crate day1;
use day1::*;

fn main() {
    let input = Input::from_file("input.txt").expect("Cant open input!");

    println!("Part1 solution: {}", calculate_part1(&input));
    println!("Part2 solution: {}", calculate_part2(&input));
}
