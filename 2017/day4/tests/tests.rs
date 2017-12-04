extern crate day4;

use day4::*;

#[test]
fn test_sample_input1() {
    assert_eq!(true, is_row_valid("aa bb cc dd ee"))
}

#[test]
fn test_sample_input2() {
    assert_eq!(false, is_row_valid("aa bb cc dd aa"))
}

#[test]
fn test_sample_input3() {
    assert_eq!(true, is_row_valid("aa bb cc dd aaa"))
}

#[test]
fn test_sample_input4() {
    assert_eq!(2, count_valid_rows("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa", is_row_valid))
}