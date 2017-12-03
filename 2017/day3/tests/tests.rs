extern crate day3;

use day3::*;

#[test]
fn test_sample_input1() {
    assert_eq!(Point { x: 0, y: 0 }, Point::from_number(1))
}

#[test]
fn test_sample_input2() {
    assert_eq!(Point { x: 1, y: 0 }, Point::from_number(2))
}

#[test]
fn test_sample_input3() {
    assert_eq!(Point { x: 1, y: -1 }, Point::from_number(9))
}

#[test]
fn test_sample_input4() {
    assert_eq!(Point { x: -2, y: -2 }, Point::from_number(21))
}
