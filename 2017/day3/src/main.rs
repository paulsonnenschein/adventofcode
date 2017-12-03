extern crate day3;

use day3::*;


fn main() {
    let input = 265_149;
    let Point { x, y } = Point::from_number(input);

    println!("Part1 solution: {}", x.abs() + y.abs());
    let Point { x, y } = Point::from_number2(input);
    println!("Part2 solution: {}", x.abs() + y.abs());
}
