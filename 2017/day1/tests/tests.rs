extern crate day1;

use day1::*;

#[test]
fn test_sample_input1() {
    let input = Input::from_str("1122");
    let expected = 3;
    assert_eq!(calculate(&input), expected);
}

#[test]
fn test_sample_input2() {
    let input = Input::from_str("1111");
    let expected = 4;
    assert_eq!(calculate(&input), expected);
}

#[test]
fn test_sample_input3() {
    let input = Input::from_str("1234");
    let expected = 0;
    assert_eq!(calculate(&input), expected);
}

#[test]
fn test_sample_input4() {
    let input = Input::from_str("91212129");
    let expected = 9;
    assert_eq!(calculate(&input), expected);
}

#[test]
fn test_actual() {
    let input = Input::from_file("input.txt").unwrap();
    let expected = 1251;
    assert_eq!(calculate(&input), expected);
}
