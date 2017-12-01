extern crate day1;

use day1::*;

#[test]
fn test_sample_input1() {
    let input = Input::from_string("1122");
    let expected = 3;
    assert_eq!(calculate_part1(&input), expected);
}

#[test]
fn test_sample_input2() {
    let input = Input::from_string("1111");
    let expected = 4;
    assert_eq!(calculate_part1(&input), expected);
}

#[test]
fn test_sample_input3() {
    let input = Input::from_string("1234");
    let expected = 0;
    assert_eq!(calculate_part1(&input), expected);
}

#[test]
fn test_sample_input4() {
    let input = Input::from_string("91212129");
    let expected = 9;
    assert_eq!(calculate_part1(&input), expected);
}

#[test]
fn test_part1() {
    let input = Input::from_file("input.txt").unwrap();
    let expected = 1251;
    assert_eq!(calculate_part1(&input), expected);
}

#[test]
fn test_part2() {
    let input = Input::from_file("input.txt").unwrap();
    let expected = 1244;
    assert_eq!(calculate_part2(&input), expected);
}
