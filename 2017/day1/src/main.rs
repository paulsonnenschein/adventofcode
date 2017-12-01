extern crate day1;
use day1::*;

fn main() {
    let input = Input::from_file("input.txt").expect("Cant open input!");

    println!("Solution: {}", calculate(&input));
}
