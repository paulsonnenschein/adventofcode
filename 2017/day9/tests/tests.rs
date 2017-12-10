extern crate day9;
use day9::*;

#[test]
fn test_sample_input1() {
    assert_eq!(1, count_score("{}").0)
}

#[test]
fn test_sample_input2() {
    assert_eq!(5, count_score("{{},{}}").0)
}