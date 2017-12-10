extern crate day9;
use day9::*;

fn main() {
    let input = include_str!("../input.txt");
    let (score, garbage_counter) = count_score(input);

    println!("Part1 solution: {}", score);
    println!("Part2 solution: {}", garbage_counter);
}
