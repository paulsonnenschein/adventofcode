extern crate day15;
use day15::*;

fn main() {
    let gen_a_start = 618;
    let gen_b_start = 814;

    println!(
        "Part1 solution: {}",
        process_input(gen_a_start, gen_b_start)
    );
    println!(
        "Part2 solution: {}",
        process_input_part2(gen_a_start, gen_b_start)
    );
}
